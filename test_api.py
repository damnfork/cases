#!/usr/bin/env python3
"""
API 测试脚本
使用方法: python test_api.py [base_url]
默认 base_url: http://localhost:8080
"""

import sys
import time
import requests
import json
from typing import Optional

def test_api(base_url: str = "http://localhost:8080"):
    """测试API功能"""
    
    print("🧪 开始测试 API 功能...")
    print(f"📍 基础URL: {base_url}")
    print()
    
    # 测试1: 获取统计信息
    print("📊 测试1: 获取系统统计信息")
    try:
        response = requests.get(f"{base_url}/api/stats")
        print(f"状态码: {response.status_code}")
        if response.status_code == 200:
            data = response.json()
            print(f"响应: {json.dumps(data, ensure_ascii=False, indent=2)}")
        else:
            print(f"错误: {response.text}")
    except Exception as e:
        print(f"请求失败: {e}")
    print()
    
    # 测试2: 搜索案例
    print("🔍 测试2: 搜索案例")
    try:
        response = requests.get(f"{base_url}/api/search", params={
            "q": "合同",
            "limit": 5
        })
        print(f"状态码: {response.status_code}")
        if response.status_code == 200:
            data = response.json()
            print(f"总结果数: {data.get('total', 0)}")
            print(f"返回结果数: {len(data.get('results', []))}")
            if data.get('results'):
                first_result = data['results'][0]
                print(f"第一个结果: {first_result.get('case_name', 'N/A')}")
        else:
            print(f"错误: {response.text}")
    except Exception as e:
        print(f"请求失败: {e}")
    print()
    
    # 测试3: 测试限流
    print("🚦 测试3: 测试限流功能")
    print("发送10个快速请求来测试限流...")
    success_count = 0
    rate_limit_count = 0
    
    for i in range(10):
        try:
            response = requests.get(f"{base_url}/api/stats")
            if response.status_code == 200:
                success_count += 1
                print(f"请求 {i+1}: ✅ 成功")
            elif response.status_code == 429:
                rate_limit_count += 1
                print(f"请求 {i+1}: 🚦 限流")
            else:
                print(f"请求 {i+1}: ❌ 状态码 {response.status_code}")
        except Exception as e:
            print(f"请求 {i+1}: ❌ 失败 - {e}")
    
    print(f"成功: {success_count}, 限流: {rate_limit_count}")
    print()
    
    # 测试4: 访问API文档页面
    print("📖 测试4: 访问API文档页面")
    try:
        response = requests.get(f"{base_url}/api/docs")
        if response.status_code == 200:
            print("✅ API文档页面访问成功")
            print(f"页面大小: {len(response.content)} 字节")
        else:
            print(f"❌ API文档页面访问失败: {response.status_code}")
    except Exception as e:
        print(f"❌ 请求失败: {e}")
    print()
    
    # 测试5: 测试不存在的案例
    print("🔍 测试5: 测试不存在的案例")
    try:
        response = requests.get(f"{base_url}/api/case/999999")
        print(f"状态码: {response.status_code}")
        if response.status_code == 404:
            print("✅ 正确处理了不存在的案例")
        else:
            print(f"响应: {response.text}")
    except Exception as e:
        print(f"请求失败: {e}")
    print()
    
    print("🎉 API 测试完成！")
    print()
    print("💡 提示:")
    print("   - 如果看到429状态码，说明限流功能正常工作")
    print(f"   - 访问 {base_url}/api/docs 查看完整的API文档")
    print("   - 所有API都支持JSON格式的响应")

def main():
    """主函数"""
    base_url = sys.argv[1] if len(sys.argv) > 1 else "http://localhost:8080"
    test_api(base_url)

if __name__ == "__main__":
    main() 