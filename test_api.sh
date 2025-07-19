#!/bin/bash

# API 测试脚本
# 使用方法: ./test_api.sh [base_url]
# 默认 base_url: http://localhost:8080

BASE_URL=${1:-"http://localhost:8080"}

echo "🧪 开始测试 API 功能..."
echo "📍 基础URL: $BASE_URL"
echo ""

# 测试1: 获取统计信息
echo "📊 测试1: 获取系统统计信息"
echo "GET $BASE_URL/api/stats"
response=$(curl -s -w "\n%{http_code}" "$BASE_URL/api/stats")
http_code=$(echo "$response" | tail -n1)
body=$(echo "$response" | head -n -1)

echo "状态码: $http_code"
echo "响应: $body"
echo ""

# 测试2: 搜索案例
echo "🔍 测试2: 搜索案例"
echo "GET $BASE_URL/api/search?q=合同&limit=5"
response=$(curl -s -w "\n%{http_code}" "$BASE_URL/api/search?q=合同&limit=5")
http_code=$(echo "$response" | tail -n1)
body=$(echo "$response" | head -n -1)

echo "状态码: $http_code"
echo "响应: $body"
echo ""

# 测试3: 测试限流（快速发送多个请求）
echo "🚦 测试3: 测试限流功能"
echo "发送10个快速请求来测试限流..."
for i in {1..10}; do
    response=$(curl -s -w "\n%{http_code}" "$BASE_URL/api/stats")
    http_code=$(echo "$response" | tail -n1)
    echo "请求 $i: 状态码 $http_code"
done
echo ""

# 测试4: 访问API文档页面
echo "📖 测试4: 访问API文档页面"
echo "GET $BASE_URL/api/docs"
response=$(curl -s -w "\n%{http_code}" "$BASE_URL/api/docs")
http_code=$(echo "$response" | tail -n1)

echo "状态码: $http_code"
if [ "$http_code" = "200" ]; then
    echo "✅ API文档页面访问成功"
else
    echo "❌ API文档页面访问失败"
fi
echo ""

echo "🎉 API 测试完成！"
echo ""
echo "💡 提示:"
echo "   - 如果看到429状态码，说明限流功能正常工作"
echo "   - 访问 $BASE_URL/api/docs 查看完整的API文档"
echo "   - 所有API都支持JSON格式的响应" 