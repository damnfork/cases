# API 功能说明

## 概述

本项目现在提供了完整的 RESTful API 接口，支持案例搜索和查询功能。所有API接口都受到限流保护。

## 限流说明

- **限制**: 每分钟最多100个请求
- **超出限制**: 返回 429 Too Many Requests 状态码
- **适用范围**: 所有API接口

## API 端点

### 1. 搜索案例

**端点**: `GET /api/search`

**参数**:
- `q` (可选): 搜索关键词
- `offset` (可选): 偏移量，默认0
- `limit` (可选): 返回结果数量，默认20，最大100

**示例**:
```bash
curl "http://localhost:8080/api/search?q=合同纠纷&limit=10"
```

**响应**:
```json
{
  "total": 1250,
  "offset": 0,
  "limit": 10,
  "results": [
    {
      "id": 12345,
      "doc_id": "https://example.com/case/12345",
      "case_id": "(2023)京01民终1234号",
      "case_name": "张三与李四合同纠纷案",
      "court": "北京市第一中级人民法院",
      "case_type": "民事案件",
      "procedure": "二审",
      "judgment_date": "2023-12-01",
      "public_date": "2023-12-15",
      "parties": "张三;李四",
      "cause": "合同纠纷",
      "legal_basis": "《中华人民共和国民法典》第五百七十七条",
      "full_text": "案件全文内容..."
    }
  ]
}
```

### 2. 获取案例详情

**端点**: `GET /api/case/{id}`

**参数**:
- `id` (路径参数): 案例ID

**示例**:
```bash
curl "http://localhost:8080/api/case/12345"
```

**响应**:
```json
{
  "id": 12345,
  "doc_id": "https://example.com/case/12345",
  "case_id": "(2023)京01民终1234号",
  "case_name": "张三与李四合同纠纷案",
  "court": "北京市第一中级人民法院",
  "case_type": "民事案件",
  "procedure": "二审",
  "judgment_date": "2023-12-01",
  "public_date": "2023-12-15",
  "parties": "张三;李四",
  "cause": "合同纠纷",
  "legal_basis": "《中华人民共和国民法典》第五百七十七条",
  "full_text": "案件全文内容..."
}
```

### 3. 获取系统统计信息

**端点**: `GET /api/stats`

**示例**:
```bash
curl "http://localhost:8080/api/stats"
```

**响应**:
```json
{
  "total_cases": 125000,
  "status": "ok"
}
```

### 4. API 文档页面

**端点**: `GET /api/docs`

访问此端点可以查看完整的API文档，包含所有接口的详细说明、参数、示例和状态码。

## 错误响应格式

当发生错误时，API会返回以下格式的响应：

```json
{
  "error": "ERROR_CODE",
  "message": "错误描述信息"
}
```

### 常见错误码

- `NOT_FOUND`: 请求的资源不存在
- `RATE_LIMIT_EXCEEDED`: 请求频率超过限制
- `INVALID_PARAMETER`: 请求参数无效

## 状态码

- `200 OK`: 请求成功
- `400 Bad Request`: 请求参数错误
- `404 Not Found`: 资源不存在
- `429 Too Many Requests`: 请求频率超限
- `500 Internal Server Error`: 服务器内部错误

## 使用示例

### JavaScript 示例

```javascript
// 搜索案例
async function searchCases(query, limit = 20) {
  const response = await fetch(`/api/search?q=${encodeURIComponent(query)}&limit=${limit}`);
  if (response.ok) {
    const data = await response.json();
    return data.results;
  } else {
    throw new Error(`HTTP ${response.status}: ${response.statusText}`);
  }
}

// 获取案例详情
async function getCase(id) {
  const response = await fetch(`/api/case/${id}`);
  if (response.ok) {
    return await response.json();
  } else {
    throw new Error(`HTTP ${response.status}: ${response.statusText}`);
  }
}

// 获取统计信息
async function getStats() {
  const response = await fetch('/api/stats');
  if (response.ok) {
    return await response.json();
  } else {
    throw new Error(`HTTP ${response.status}: ${response.statusText}`);
  }
}
```

### Python 示例

```python
import requests

# 搜索案例
def search_cases(query, limit=20):
    response = requests.get(f'/api/search', params={
        'q': query,
        'limit': limit
    })
    response.raise_for_status()
    return response.json()['results']

# 获取案例详情
def get_case(case_id):
    response = requests.get(f'/api/case/{case_id}')
    response.raise_for_status()
    return response.json()

# 获取统计信息
def get_stats():
    response = requests.get('/api/stats')
    response.raise_for_status()
    return response.json()
```

## 注意事项

1. 所有API请求都应该包含适当的错误处理
2. 建议实现重试机制，特别是对于429状态码
3. 在生产环境中，建议使用HTTPS
4. API文档页面提供了更详细的接口说明和示例 