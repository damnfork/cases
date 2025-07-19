# API Token 认证功能

## 🔐 功能概述

新增了基于 token 的 API 认证和差异化限流功能，支持：

- **多 token 管理**：在配置文件中管理多个 API token
- **差异化限流**：不同 token 拥有不同的请求频率限制
- **灵活认证**：支持多种 token 传递方式
- **安全验证**：无效 token 返回 401 Unauthorized

## 📝 配置说明

### config.toml 配置格式

```toml
# API 配置
api_rate_limit = 100         # 默认每分钟API请求限制数量（无token时使用）

# API Token 配置
[api_tokens]
"demo-token-1" = { rate_limit = 1000, description = "高级用户token" }
"demo-token-2" = { rate_limit = 500, description = "标准用户token" }
"demo-token-3" = { rate_limit = 100, description = "基础用户token" }
```

### 配置参数说明

- **api_rate_limit**: 默认限流，当没有 token 或没有配置 api_tokens 时使用
- **api_tokens**: token 配置表
  - `key`: token 字符串（用户在请求中传递的实际 token）
  - `rate_limit`: 该 token 的每分钟请求限制
  - `description`: token 描述（可选，用于管理和文档）

## 🚀 使用方法

### 1. Authorization Bearer Token

```bash
curl -H "Authorization: Bearer demo-token-1" \
     "http://localhost:8081/api/search?q=合同纠纷&limit=10"
```

### 2. 自定义头 X-API-Token

```bash
curl -H "X-API-Token: demo-token-2" \
     "http://localhost:8081/api/case/12345"
```

### 3. 无 Token 访问

```bash
curl "http://localhost:8081/api/stats"
# 使用默认限流：100 requests/minute
```

## 📊 限流策略

### 限流层级

1. **Token 级别**：每个 token 有独立的限流计数器
2. **配置驱动**：限流值完全由配置文件控制
3. **内存缓存**：限流器在内存中缓存，重启后重置

### 示例配置场景

```toml
[api_tokens]
"enterprise-key-001" = { rate_limit = 10000, description = "企业客户" }
"premium-user-123" = { rate_limit = 1000, description = "高级用户" }
"standard-user-456" = { rate_limit = 500, description = "标准用户" }
"basic-user-789" = { rate_limit = 100, description = "基础用户" }
"trial-user-000" = { rate_limit = 50, description = "试用用户" }
```

## 🔄 响应状态码

### 成功响应
- **200 OK**: 请求成功，返回数据

### 错误响应
- **401 Unauthorized**: 无效的 API token
- **429 Too Many Requests**: 超过该 token 的请求频率限制
- **400 Bad Request**: 请求参数错误
- **500 Internal Server Error**: 服务器内部错误

### 错误响应示例

```json
// 401 Unauthorized
{
  "error": "INVALID_TOKEN",
  "message": "Invalid API token"
}

// 429 Too Many Requests  
{
  "error": "RATE_LIMIT_EXCEEDED", 
  "message": "Rate limit exceeded. Please try again later."
}
```

## 🧪 测试脚本

使用提供的测试脚本验证功能：

```bash
# 运行 token 认证测试
./test_token_api.sh

# 使用自定义服务器地址测试
./test_token_api.sh http://localhost:8081
```

测试脚本会验证：
- 无 token 访问（默认限流）
- 有效 token 访问（不同限流等级）
- 无效 token 访问（401 错误）
- 不同认证方式（Bearer vs X-API-Token）
- 限流差异化效果

## 💡 实现细节

### 认证流程

1. **Token 提取**：从请求头提取 token
   - `Authorization: Bearer <token>`
   - `X-API-Token: <token>`

2. **Token 验证**：检查 token 是否在配置中存在
   - 存在：获取对应的限流配置
   - 不存在：返回 401 Unauthorized

3. **限流检查**：使用该 token 的专属限流器
   - 通过：继续处理请求
   - 超限：返回 429 Too Many Requests

### 限流器管理

- **缓存机制**：每个 token 创建独立的限流器实例
- **内存存储**：限流器存储在内存中，重启后重置
- **线程安全**：使用 Mutex 保护并发访问

### 性能优化

- **惰性创建**：限流器按需创建，不预先初始化
- **Arc 共享**：使用 Arc 避免重复创建相同配置的限流器
- **高效查找**：使用 HashMap 快速查找 token 配置

## 🔧 管理建议

### Token 管理

1. **命名规范**：使用有意义的 token 名称
   ```toml
   "company-api-prod-v1" = { rate_limit = 5000, description = "生产环境API" }
   "user-dashboard-dev" = { rate_limit = 1000, description = "开发环境仪表板" }
   ```

2. **分级管理**：根据用户等级设置不同限流
   ```toml
   "vip-unlimited" = { rate_limit = 100000, description = "VIP用户" }
   "enterprise" = { rate_limit = 10000, description = "企业用户" }
   "standard" = { rate_limit = 1000, description = "标准用户" }
   "free" = { rate_limit = 100, description = "免费用户" }
   ```

3. **安全考虑**：
   - 使用复杂的 token 字符串
   - 定期轮换 token
   - 监控异常使用模式

### 监控建议

1. **日志记录**：记录 token 使用情况
2. **指标监控**：监控各 token 的请求量和错误率
3. **告警设置**：异常访问模式告警

## 🚀 扩展功能

### 未来可能的扩展

1. **Token 过期**：添加 token 过期时间
2. **IP 白名单**：限制 token 的使用 IP
3. **权限控制**：不同 token 访问不同的 API 端点
4. **使用统计**：详细的 token 使用统计和报告
5. **动态配置**：支持运行时修改 token 配置

### 配置扩展示例

```toml
[api_tokens.advanced-token]
rate_limit = 1000
description = "高级功能token"
expires_at = "2024-12-31T23:59:59Z"
allowed_ips = ["192.168.1.0/24", "10.0.0.0/8"]
allowed_endpoints = ["/api/search", "/api/stats"]
```

这个 token 认证系统为 API 提供了灵活、安全、高性能的访问控制机制！ 