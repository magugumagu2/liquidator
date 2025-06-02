// RPCエンドポイントのレート制限をテストするスクリプト
const fetch = require('node-fetch');

// テスト対象のRPCエンドポイント
const RPC_URL = 'YOUR_RPC_ENDPOINT_HERE';

// リクエストを送信する関数
async function sendRequest() {
    const payload = {
        jsonrpc: '2.0',
        method: 'eth_blockNumber',
        params: [],
        id: Date.now()
    };

    try {
        const response = await fetch(RPC_URL, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify(payload)
        });

        const headers = {};
        for (const [key, value] of response.headers.entries()) {
            headers[key] = value;
        }

        const data = await response.json();
        return { success: true, status: response.status, headers, data };
    } catch (error) {
        return { success: false, error: error.message };
    }
}

// メイン処理
async function main() {
    console.log('RPCエンドポイントレート制限テスト開始...');
    console.log(`エンドポイント: ${RPC_URL}`);

    // リクエスト間隔 (ミリ秒)
    const interval = 100;
    // テスト期間 (ミリ秒)
    const testDuration = 30000; // 30秒

    const startTime = Date.now();
    let requestCount = 0;
    let successCount = 0;
    let errorCount = 0;

    // 一定間隔でリクエストを送信
    const intervalId = setInterval(async () => {
        requestCount++;
        const result = await sendRequest();

        if (result.success) {
            successCount++;
        } else {
            errorCount++;
            console.error(`エラー: ${result.error}`);
        }

        // 最初のリクエストのヘッダー情報を表示
        if (requestCount === 1 && result.headers) {
            console.log('レスポンスヘッダー:');
            console.log(result.headers);
        }

        // テスト期間が終了したら停止
        if (Date.now() - startTime >= testDuration) {
            clearInterval(intervalId);
            printResults(startTime, requestCount, successCount, errorCount);
        }
    }, interval);
}

// 結果を表示する関数
function printResults(startTime, requestCount, successCount, errorCount) {
    const duration = (Date.now() - startTime) / 1000;
    console.log('\nテスト結果:');
    console.log(`テスト時間: ${duration.toFixed(2)}秒`);
    console.log(`総リクエスト数: ${requestCount}`);
    console.log(`成功: ${successCount}`);
    console.log(`エラー: ${errorCount}`);
    console.log(`毎秒リクエスト数: ${(requestCount / duration).toFixed(2)}`);

    if (errorCount > 0) {
        console.log(`エラー率: ${((errorCount / requestCount) * 100).toFixed(2)}%`);
        console.log('エラーが発生しました。RPCエンドポイントのレート制限に達した可能性があります。');
    } else {
        console.log('エラーなし。現在のリクエストレートではレート制限に達していません。');
    }
}

// スクリプト実行
main().catch(console.error); 