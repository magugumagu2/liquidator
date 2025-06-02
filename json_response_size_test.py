#!/usr/bin/env python3
"""
RPCエンドポイントのJSON応答サイズを測定するスクリプト
さまざまなRPCメソッドのレスポンスサイズとエンコード方法を評価します
"""

import json
import sys
import time
import requests
import zlib
import gzip
import brotli
from tabulate import tabulate
from humanize import naturalsize

# RPC エンドポイント設定
RPC_URL = "YOUR_RPC_ENDPOINT_HERE"

# テストするRPCメソッド
RPC_METHODS = [
    {
        "name": "eth_blockNumber",
        "method": "eth_blockNumber",
        "params": [],
        "description": "最新ブロック番号取得"
    },
    {
        "name": "eth_getBlockByNumber (latest)",
        "method": "eth_getBlockByNumber",
        "params": ["latest", True],
        "description": "最新ブロック情報取得"
    },
    {
        "name": "eth_getBlockByNumber (specific)",
        "method": "eth_getBlockByNumber",
        "params": ["0xBA4500", True],  # 12,175,104 ブロック (適宜調整)
        "description": "特定ブロック情報取得"
    },
    {
        "name": "eth_getLogs (last 100 blocks)",
        "method": "eth_getLogs",
        "params": [{
            "fromBlock": "0xBA4460",  # 12,175,104 - 100 (適宜調整)
            "toBlock": "0xBA4500"     # 12,175,104 (適宜調整)
        }],
        "description": "過去100ブロックのログ取得"
    },
    {
        "name": "eth_getCode (USDT)",
        "method": "eth_getCode",
        "params": ["0xdAC17F958D2ee523a2206206994597C13D831ec7", "latest"],
        "description": "USDTコントラクトのコード取得"
    },
    {
        "name": "eth_getStorageAt (example)",
        "method": "eth_getStorageAt",
        "params": ["0xdAC17F958D2ee523a2206206994597C13D831ec7", "0x0", "latest"],
        "description": "コントラクトのストレージ取得"
    }
]

def send_rpc_request(method, params):
    """RPC リクエストを送信して応答を取得"""
    payload = {
        "jsonrpc": "2.0",
        "method": method,
        "params": params,
        "id": int(time.time() * 1000)
    }
    
    headers = {
        "Content-Type": "application/json",
        "Accept-Encoding": "gzip, deflate, br"  # 圧縮を許可
    }
    
    try:
        response = requests.post(
            RPC_URL,
            headers=headers,
            json=payload,
            timeout=30  # 30秒タイムアウト
        )
        
        if response.status_code == 200:
            # レスポンスのContent-Lengthヘッダーを取得
            content_length = int(response.headers.get('Content-Length', 0)) if 'Content-Length' in response.headers else len(response.content)
            
            # 使用された圧縮方法を確認
            content_encoding = response.headers.get('Content-Encoding', 'none')
            
            try:
                # レスポンスをJSONとしてデコード
                data = response.json()
                
                # JSONデータを文字列に変換して、サイズを計算
                json_str = json.dumps(data)
                json_size = len(json_str.encode('utf-8'))
                
                # 圧縮サイズの計算
                gzip_size = len(gzip.compress(json_str.encode('utf-8')))
                zlib_size = len(zlib.compress(json_str.encode('utf-8')))
                brotli_size = len(brotli.compress(json_str.encode('utf-8')))
                
                return {
                    "success": True,
                    "response": data,
                    "raw_size": json_size,
                    "content_length": content_length,
                    "content_encoding": content_encoding,
                    "compression": {
                        "gzip": gzip_size,
                        "zlib": zlib_size,
                        "brotli": brotli_size
                    }
                }
            except json.JSONDecodeError as e:
                return {
                    "success": False,
                    "error": f"JSONデコードエラー: {str(e)}",
                    "raw_size": len(response.content),
                    "content_length": content_length,
                    "content_encoding": content_encoding
                }
        else:
            return {
                "success": False,
                "error": f"HTTP エラー: {response.status_code}",
                "raw_size": len(response.content),
                "content_length": response.headers.get('Content-Length', 0),
                "content_encoding": response.headers.get('Content-Encoding', 'none')
            }
    except requests.exceptions.Timeout:
        return {
            "success": False,
            "error": "リクエストタイムアウト"
        }
    except requests.exceptions.RequestException as e:
        return {
            "success": False,
            "error": f"リクエストエラー: {str(e)}"
        }

def main():
    """メインの実行関数"""
    print(f"RPCエンドポイントのJSON応答サイズテスト: {RPC_URL}")
    print("=" * 70)
    
    results = []
    
    try:
        for method_info in RPC_METHODS:
            method_name = method_info["name"]
            method = method_info["method"]
            params = method_info["params"]
            description = method_info["description"]
            
            print(f"\n実行中: {method_name} - {description}")
            sys.stdout.write(f"リクエスト送信中...")
            sys.stdout.flush()
            
            start_time = time.time()
            result = send_rpc_request(method, params)
            response_time = time.time() - start_time
            
            if result["success"]:
                result_data = result["response"]
                if "error" in result_data:
                    print(f"\r✗ エラー: {result_data['error']}")
                    results.append([
                        method_name,
                        "エラー",
                        "N/A",
                        "N/A",
                        "N/A",
                        "N/A",
                        "N/A",
                        f"{response_time:.2f}s"
                    ])
                else:
                    raw_size = result["raw_size"]
                    content_length = result["content_length"]
                    content_encoding = result["content_encoding"]
                    compression = result.get("compression", {})
                    
                    # サイズを人間が読みやすい形式に変換
                    human_raw_size = naturalsize(raw_size)
                    human_content_length = naturalsize(content_length)
                    human_gzip = naturalsize(compression.get("gzip", 0)) if "gzip" in compression else "N/A"
                    human_zlib = naturalsize(compression.get("zlib", 0)) if "zlib" in compression else "N/A"
                    human_brotli = naturalsize(compression.get("brotli", 0)) if "brotli" in compression else "N/A"
                    
                    compression_ratio = f"{(content_length / raw_size) * 100:.1f}%" if raw_size > 0 and content_length > 0 else "N/A"
                    
                    print(f"\r✓ 成功: サイズ {human_raw_size}, 転送サイズ {human_content_length} ({content_encoding}), 応答時間 {response_time:.2f}s")
                    
                    results.append([
                        method_name,
                        "成功",
                        human_raw_size,
                        human_content_length,
                        compression_ratio,
                        content_encoding,
                        f"{response_time:.2f}s",
                        f"gzip: {human_gzip}, brotli: {human_brotli}"
                    ])
            else:
                print(f"\r✗ エラー: {result['error']}")
                results.append([
                    method_name,
                    "エラー",
                    "N/A",
                    "N/A",
                    "N/A",
                    "N/A",
                    "N/A",
                    f"{response_time:.2f}s"
                ])
    
    except KeyboardInterrupt:
        print("\nテストが中断されました。")
    
    # 結果の表示
    print("\n\n========== JSON応答サイズ結果 ==========")
    headers = ["メソッド", "状態", "raw サイズ", "転送サイズ", "圧縮率", "エンコード", "応答時間", "圧縮オプション"]
    print(tabulate(results, headers=headers, tablefmt="grid"))
    
    # リスク分析
    print("\n========== リスク分析 ==========")
    for result in results:
        method = result[0]
        status = result[1]
        
        if status == "エラー":
            print(f"⚠️ {method}: 応答取得エラー")
            continue
            
        size = result[2]
        transfer_size = result[3]
        
        # サイズからMBだけを抽出（例：「10.5 MB」→ 10.5）
        try:
            if "MB" in size:
                size_mb = float(size.split()[0])
                
                if size_mb > 10:
                    print(f"⚠️ 高リスク: {method} - 非常に大きいレスポンスサイズ ({size})。JSONパースエラーやメモリ問題が発生する可能性があります。")
                elif size_mb > 5:
                    print(f"⚠️ 中リスク: {method} - 大きいレスポンスサイズ ({size})。JSONパースエラーのリスクがあります。")
                elif size_mb > 1:
                    print(f"⚠️ 低リスク: {method} - やや大きいレスポンスサイズ ({size})。")
        except:
            pass
            
    print("\n========== 提案 ==========")
    print("1. 大きなレスポンスがあるメソッドには、必ずgzip/brotli圧縮を使用してください。")
    print("2. ブロック範囲を小さくして、eth_getLogsのようなメソッドを呼び出すことを検討してください。")
    print("3. 1回のリクエストでパースが難しい大量のデータを返すメソッドは回避してください。")
    print("4. レスポンスサイズが大きいリクエストにはタイムアウト時間を長く設定してください。")
    
    # 最大のレスポンスサイズを持つメソッドを特定
    try:
        max_size_method = max((r for r in results if r[1] == "成功"), key=lambda x: parse_size(x[2]))
        if max_size_method:
            print(f"\n最大のレスポンスサイズ: {max_size_method[0]} ({max_size_method[2]})")
            print(f"このメソッドには特に注意してください。EOF parse errors が発生しやすいです。")
    except:
        pass

def parse_size(size_str):
    """サイズ文字列から数値を抽出する補助関数"""
    try:
        if isinstance(size_str, str):
            parts = size_str.split()
            if len(parts) >= 1:
                value = float(parts[0])
                if "KB" in size_str:
                    return value * 1024
                elif "MB" in size_str:
                    return value * 1024 * 1024
                elif "GB" in size_str:
                    return value * 1024 * 1024 * 1024
                else:
                    return value
        return 0
    except:
        return 0

if __name__ == "__main__":
    # tabulate と humanize パッケージが必要
    # pip install tabulate humanize
    try:
        import tabulate
        import humanize
    except ImportError:
        print("必要なパッケージがありません。以下のコマンドでインストールしてください：")
        print("pip install tabulate humanize brotli")
        sys.exit(1)
        
    main() 