#!/usr/bin/env python3
"""
RPC エンドポイントのネットワーク安定性テスト
リクエストのレイテンシーとエラー率を測定します
"""

import time
import json
import statistics
import sys
import requests
from datetime import datetime, timedelta
from urllib.parse import urlparse

# 設定
RPC_URL = "YOUR_RPC_ENDPOINT_HERE"
TEST_DURATION_MINUTES = 30
REQUEST_INTERVAL_SECONDS = 5

def send_request():
    """RPC エンドポイントに単一のリクエストを送信"""
    payload = {
        "jsonrpc": "2.0",
        "method": "eth_blockNumber",
        "params": [],
        "id": int(time.time() * 1000)
    }
    
    start_time = time.time()
    try:
        response = requests.post(
            RPC_URL,
            headers={"Content-Type": "application/json"},
            json=payload,
            timeout=10  # 10秒でタイムアウト
        )
        response_time = time.time() - start_time
        
        # レスポンスの検証
        if response.status_code == 200:
            try:
                data = response.json()
                if "result" in data:
                    return {
                        "success": True,
                        "latency": response_time,
                        "status_code": response.status_code,
                        "block_number": int(data["result"], 16) if data["result"] else None
                    }
                else:
                    return {
                        "success": False,
                        "latency": response_time,
                        "status_code": response.status_code,
                        "error": f"無効なレスポンス: {data}"
                    }
            except json.JSONDecodeError:
                return {
                    "success": False,
                    "latency": response_time,
                    "status_code": response.status_code,
                    "error": "JSONデコードエラー"
                }
        else:
            return {
                "success": False,
                "latency": response_time,
                "status_code": response.status_code,
                "error": f"HTTP エラー: {response.status_code}"
            }
    except requests.exceptions.Timeout:
        return {
            "success": False,
            "latency": time.time() - start_time,
            "error": "リクエストタイムアウト"
        }
    except requests.exceptions.ConnectionError:
        return {
            "success": False,
            "latency": time.time() - start_time,
            "error": "接続エラー"
        }
    except Exception as e:
        return {
            "success": False,
            "latency": time.time() - start_time,
            "error": f"予期せぬエラー: {str(e)}"
        }

def run_test():
    """指定された期間、定期的にリクエストを送信してネットワーク安定性をテスト"""
    print(f"RPC エンドポイント安定性テストを開始します: {RPC_URL}")
    print(f"テスト期間: {TEST_DURATION_MINUTES} 分")
    print(f"リクエスト間隔: {REQUEST_INTERVAL_SECONDS} 秒")
    print("=" * 50)
    
    host = urlparse(RPC_URL).netloc
    print(f"対象ホスト: {host}")
    
    # 統計情報を保存するリスト
    latencies = []
    status_codes = []
    errors = []
    
    end_time = datetime.now() + timedelta(minutes=TEST_DURATION_MINUTES)
    request_count = 0
    success_count = 0
    
    try:
        while datetime.now() < end_time:
            request_count += 1
            
            # 現在時刻と進捗を表示
            progress = (datetime.now() - (end_time - timedelta(minutes=TEST_DURATION_MINUTES))) / timedelta(minutes=TEST_DURATION_MINUTES)
            current_time = datetime.now().strftime("%H:%M:%S")
            sys.stdout.write(f"\r[{current_time}] リクエスト #{request_count} 送信中... ({progress:.1%} 完了)")
            sys.stdout.flush()
            
            # リクエスト送信
            result = send_request()
            
            if result["success"]:
                success_count += 1
                latencies.append(result["latency"])
                status_codes.append(result["status_code"])
                sys.stdout.write(f" ✓ {result['latency']:.3f}秒")
            else:
                errors.append(result["error"])
                sys.stdout.write(f" ✗ エラー: {result['error']}")
            
            sys.stdout.flush()
            
            # 次のリクエストまで待機
            time.sleep(REQUEST_INTERVAL_SECONDS)
            
    except KeyboardInterrupt:
        print("\nテストが中断されました。")
    
    # 結果の表示
    print("\n\n============ テスト結果 ============")
    print(f"総リクエスト数: {request_count}")
    print(f"成功したリクエスト: {success_count}")
    print(f"失敗したリクエスト: {request_count - success_count}")
    print(f"成功率: {(success_count / request_count) * 100:.2f}%")
    
    if latencies:
        print("\n--- レイテンシー統計 (秒) ---")
        print(f"最小: {min(latencies):.3f}")
        print(f"最大: {max(latencies):.3f}")
        print(f"平均: {sum(latencies) / len(latencies):.3f}")
        print(f"中央値: {statistics.median(latencies):.3f}")
        if len(latencies) > 1:
            print(f"標準偏差: {statistics.stdev(latencies):.3f}")
        
        # レイテンシーの分布
        bins = [0, 0.1, 0.5, 1, 2, 5, float('inf')]
        bin_names = ["0-100ms", "100-500ms", "500ms-1s", "1-2s", "2-5s", "5s+"]
        distribution = [0] * len(bin_names)
        
        for lat in latencies:
            for i, threshold in enumerate(bins[1:]):
                if lat < threshold:
                    distribution[i] += 1
                    break
        
        print("\n--- レイテンシー分布 ---")
        for i, name in enumerate(bin_names):
            percentage = (distribution[i] / len(latencies)) * 100
            print(f"{name}: {distribution[i]} ({percentage:.1f}%)")
    
    if errors:
        print("\n--- エラー分析 ---")
        error_counts = {}
        for error in errors:
            error_counts[error] = error_counts.get(error, 0) + 1
        
        for error, count in error_counts.items():
            print(f"{error}: {count} 回 ({(count / len(errors)) * 100:.1f}%)")
    
    # 結論
    print("\n--- 結論 ---")
    if success_count / request_count > 0.99:
        print("非常に安定しています (99%以上の成功率)")
    elif success_count / request_count > 0.95:
        print("安定しています (95%以上の成功率)")
    elif success_count / request_count > 0.9:
        print("やや不安定です (90-95%の成功率)")
    else:
        print("不安定です (90%未満の成功率)")
    
    if latencies and statistics.mean(latencies) < 0.2:
        print("レイテンシーは優れています (平均200ms未満)")
    elif latencies and statistics.mean(latencies) < 0.5:
        print("レイテンシーは良好です (平均500ms未満)")
    elif latencies and statistics.mean(latencies) < 1:
        print("レイテンシーは許容範囲内です (平均1秒未満)")
    elif latencies:
        print("レイテンシーは高めです (平均1秒以上)")

if __name__ == "__main__":
    run_test() 