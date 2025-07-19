use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use bincode::config::standard;
use indexmap::IndexSet;
use serde::{Deserialize, Serialize};
use tantivy::{
    DocAddress, Score, TantivyDocument,
    collector::{Count, TopDocs},
    schema::Value,
};
use tracing::info;

use crate::{AppState, Case};

#[derive(Debug, Deserialize)]
pub struct ApiSearchQuery {
    pub q: Option<String>,
    pub offset: Option<usize>,
    pub limit: Option<usize>,
}

#[derive(Debug, Serialize)]
pub struct ApiSearchResponse {
    pub total: usize,
    pub offset: usize,
    pub limit: usize,
    pub results: Vec<ApiCase>,
}

#[derive(Debug, Serialize)]
pub struct ApiCase {
    pub id: u32,
    pub doc_id: String,
    pub case_id: String,
    pub case_name: String,
    pub court: String,
    pub case_type: String,
    pub procedure: String,
    pub judgment_date: String,
    pub public_date: String,
    pub parties: String,
    pub cause: String,
    pub legal_basis: String,
    pub full_text: String,
}

impl From<Case> for ApiCase {
    fn from(case: Case) -> Self {
        Self {
            id: 0, // 将在调用处设置
            doc_id: case.doc_id,
            case_id: case.case_id,
            case_name: case.case_name,
            court: case.court,
            case_type: case.case_type,
            procedure: case.procedure,
            judgment_date: case.judgment_date,
            public_date: case.public_date,
            parties: case.parties,
            cause: case.cause,
            legal_basis: case.legal_basis,
            full_text: case.full_text,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ApiError {
    pub error: String,
    pub message: String,
}

pub async fn api_search(
    Query(query): Query<ApiSearchQuery>,
    State(state): State<AppState>,
) -> Result<Json<ApiSearchResponse>, (StatusCode, Json<ApiError>)> {
    let offset = query.offset.unwrap_or(0);
    let limit = query.limit.unwrap_or(20).min(100); // 最大限制100
    let search_query = query.q.unwrap_or_default();

    let mut ids: IndexSet<u32> = IndexSet::with_capacity(limit);
    let mut total = 0;

    if !search_query.is_empty() {
        info!("API search: {}, offset: {}, limit: {}", search_query, offset, limit);
        
        let (query, _) = state.searcher.query_parser.parse_query_lenient(&search_query);
        let searcher = state.searcher.reader.searcher();
        total = searcher.search(&query, &Count).unwrap();

        let top_docs: Vec<(Score, DocAddress)> = searcher
            .search(&query, &TopDocs::with_limit(limit).and_offset(offset))
            .unwrap_or_default();

        for (_score, doc_address) in top_docs {
            if let Some(id) = searcher
                .doc::<TantivyDocument>(doc_address)
                .unwrap()
                .get_first(state.searcher.id)
                .unwrap()
                .as_u64()
            {
                ids.insert(id as u32);
            }
        }
    }

    let mut results = Vec::with_capacity(ids.len());
    for id in ids {
        if let Some(v) = state.db.get(id.to_be_bytes()).unwrap() {
            let (case, _): (Case, _) = bincode::decode_from_slice(&v, standard()).unwrap();
            let mut api_case: ApiCase = case.into();
            api_case.id = id;
            results.push(api_case);
        }
    }

    Ok(Json(ApiSearchResponse {
        total,
        offset,
        limit,
        results,
    }))
}

pub async fn api_case(
    State(state): State<AppState>,
    Path(id): Path<u32>,
) -> Result<Json<ApiCase>, (StatusCode, Json<ApiError>)> {
    info!("API case request: {}", id);
    
    if let Some(v) = state.db.get(id.to_be_bytes()).unwrap() {
        let (case, _): (Case, _) = bincode::decode_from_slice(&v, standard()).unwrap();
        let mut api_case: ApiCase = case.into();
        api_case.id = id;
        Ok(Json(api_case))
    } else {
        Err((
            StatusCode::NOT_FOUND,
            Json(ApiError {
                error: "NOT_FOUND".to_string(),
                message: "Case not found".to_string(),
            }),
        ))
    }
}

pub async fn api_stats(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ApiError>)> {
    // 获取数据库统计信息
    let stats = serde_json::json!({
        "total_cases": state.db.len().unwrap_or(0),
        "status": "ok"
    });
    
    Ok(Json(stats))
} 