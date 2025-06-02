#!/usr/bin/env python3
# -*- coding: utf-8 -*-

import matplotlib.pyplot as plt
import pandas as pd
import re
import numpy as np
import seaborn as sns
from datetime import datetime
import os

# スタイル設定
plt.style.use('ggplot')
sns.set_palette("Blues_r")

def parse_log_file(file_path):
    """
    ログファイルを解析し、タイムスタンプとラグ量を抽出する関数
    """
    timestamps = []
    lag_amounts = []
    
    pattern = r'([\d\-T:.Z]+) ERROR artemis_core::engine: error receiving event: channel lagged by (\d+)'
    
    with open(file_path, 'r') as file:
        for line in file:
            match = re.search(pattern, line)
            if match:
                timestamp_str = match.group(1)
                lag_amount = int(match.group(2))
                
                # タイムスタンプをdatetimeオブジェクトに変換
                timestamp = datetime.strptime(timestamp_str, '%Y-%m-%dT%H:%M:%S.%fZ')
                
                timestamps.append(timestamp)
                lag_amounts.append(lag_amount)
    
    return pd.DataFrame({'timestamp': timestamps, 'lag': lag_amounts})

def analyze_lag_data(df):
    """
    ラグデータを分析して基本的な統計情報を出力する関数
    """
    if df.empty:
        print("データがありません。")
        return
    
    # 基本統計量の計算
    total_events = len(df)
    max_lag = df['lag'].max()
    min_lag = df['lag'].min()
    mean_lag = df['lag'].mean()
    median_lag = df['lag'].median()
    
    # ラグの重大度によるカテゴリ分け
    critical_lags = df[df['lag'] > 1000].shape[0]  # 1000以上のラグを重大とする
    high_lags = df[(df['lag'] > 100) & (df['lag'] <= 1000)].shape[0]
    medium_lags = df[(df['lag'] > 50) & (df['lag'] <= 100)].shape[0]
    low_lags = df[(df['lag'] > 10) & (df['lag'] <= 50)].shape[0]
    minimal_lags = df[df['lag'] <= 10].shape[0]
    
    # 結果出力
    print("=== チャネルラグ分析レポート ===")
    print(f"総イベント数: {total_events}")
    print(f"最大ラグ: {max_lag}")
    print(f"最小ラグ: {min_lag}")
    print(f"平均ラグ: {mean_lag:.2f}")
    print(f"中央値ラグ: {median_lag}")
    print("\n=== ラグの重大度分布 ===")
    print(f"重大 (>1000): {critical_lags} ({critical_lags / total_events * 100:.1f}%)")
    print(f"高 (101-1000): {high_lags} ({high_lags / total_events * 100:.1f}%)")
    print(f"中 (51-100): {medium_lags} ({medium_lags / total_events * 100:.1f}%)")
    print(f"低 (11-50): {low_lags} ({low_lags / total_events * 100:.1f}%)")
    print(f"最小 (1-10): {minimal_lags} ({minimal_lags / total_events * 100:.1f}%)")
    
    return {
        'total_events': total_events,
        'max_lag': max_lag,
        'min_lag': min_lag,
        'mean_lag': mean_lag,
        'median_lag': median_lag,
        'critical_lags': critical_lags,
        'high_lags': high_lags,
        'medium_lags': medium_lags,
        'low_lags': low_lags,
        'minimal_lags': minimal_lags
    }

def plot_lag_distribution(df, output_dir=None):
    """
    ラグの分布を可視化する関数
    """
    if df.empty:
        print("データがありません。グラフは作成されません。")
        return
    
    # ラグ量のヒストグラム（対数スケール）
    plt.figure(figsize=(12, 6))
    plt.hist(df['lag'], bins=50, alpha=0.7, log=True)
    plt.title('チャネルラグ分布 (対数スケール)', fontsize=16)
    plt.xlabel('ラグ量', fontsize=14)
    plt.ylabel('発生回数 (対数)', fontsize=14)
    plt.grid(True, alpha=0.3)
    plt.tight_layout()
    
    if output_dir:
        plt.savefig(os.path.join(output_dir, 'lag_distribution_log.png'), dpi=300)
    
    plt.figure(figsize=(12, 6))
    plt.hist(df['lag'][df['lag'] < 100], bins=50, alpha=0.7)
    plt.title('チャネルラグ分布 (ラグ < 100)', fontsize=16)
    plt.xlabel('ラグ量', fontsize=14)
    plt.ylabel('発生回数', fontsize=14)
    plt.grid(True, alpha=0.3)
    plt.tight_layout()
    
    if output_dir:
        plt.savefig(os.path.join(output_dir, 'lag_distribution_small.png'), dpi=300)
    
    # 時系列でのラグ量の変化
    plt.figure(figsize=(15, 6))
    plt.plot(df['timestamp'], df['lag'], marker='o', linestyle='-', markersize=4, alpha=0.6)
    plt.title('時間経過によるチャネルラグの変化', fontsize=16)
    plt.xlabel('時間', fontsize=14)
    plt.ylabel('ラグ量', fontsize=14)
    plt.grid(True, alpha=0.3)
    plt.tight_layout()
    
    if output_dir:
        plt.savefig(os.path.join(output_dir, 'lag_time_series.png'), dpi=300)
    
    # 時系列でのラグ量の変化（対数スケール）
    plt.figure(figsize=(15, 6))
    plt.semilogy(df['timestamp'], df['lag'], marker='o', linestyle='-', markersize=4, alpha=0.6)
    plt.title('時間経過によるチャネルラグの変化 (対数スケール)', fontsize=16)
    plt.xlabel('時間', fontsize=14)
    plt.ylabel('ラグ量 (対数)', fontsize=14)
    plt.grid(True, alpha=0.3)
    plt.tight_layout()
    
    if output_dir:
        plt.savefig(os.path.join(output_dir, 'lag_time_series_log.png'), dpi=300)
    
    # ラグ分布の円グラフ
    plt.figure(figsize=(10, 10))
    lag_categories = {
        '重大 (>1000)': df[df['lag'] > 1000].shape[0],
        '高 (101-1000)': df[(df['lag'] > 100) & (df['lag'] <= 1000)].shape[0],
        '中 (51-100)': df[(df['lag'] > 50) & (df['lag'] <= 100)].shape[0],
        '低 (11-50)': df[(df['lag'] > 10) & (df['lag'] <= 50)].shape[0],
        '最小 (1-10)': df[df['lag'] <= 10].shape[0]
    }
    
    colors = sns.color_palette("Blues_r", len(lag_categories))
    plt.pie(lag_categories.values(), labels=lag_categories.keys(), autopct='%1.1f%%',
            startangle=90, colors=colors, shadow=True, explode=[0.1 if k == '重大 (>1000)' else 0 for k in lag_categories.keys()])
    plt.title('ラグの重大度分布', fontsize=16)
    plt.axis('equal')
    
    if output_dir:
        plt.savefig(os.path.join(output_dir, 'lag_severity_pie.png'), dpi=300)
    
    if not output_dir:
        plt.show()

def generate_recommendations(stats):
    """
    分析結果に基づいて対策を提案する関数
    """
    recommendations = []
    
    if stats['critical_lags'] > 0:
        recommendations.append("重大なチャネルラグが検出されました。イベントチャネルのバッファサイズの大幅な増加が必要です。")
        recommendations.append("イベント処理のバッチ化と並列処理の実装を検討してください。")
    
    if stats['mean_lag'] > 50:
        recommendations.append("平均ラグが高すぎます。イベント処理のパフォーマンスを最適化する必要があります。")
    
    if stats['high_lags'] / stats['total_events'] > 0.2:  # 20%以上が高ラグの場合
        recommendations.append("高いラグが全体の20%以上を占めています。イベントの優先度付けと選択的処理を実装してください。")
    
    # 一般的な推奨事項
    recommendations.append("統合アプローチの導入を検討してください：")
    recommendations.append("1. イベント駆動型+優先度キュー: 重要なイベントを先に処理")
    recommendations.append("2. マルチレベルスキャン+バッチ処理: リスクレベルに基づく効率的なスキャン")
    recommendations.append("3. インクリメンタルスキャン+キャッシュ戦略: ヘルスファクター変化率に基づく動的TTL設定")
    
    return recommendations

def main():
    """
    メイン実行関数
    """
    log_file = 'channel_lag_diagnosis.log'
    output_dir = 'diagnosis_output'
    
    # 出力ディレクトリの作成
    if not os.path.exists(output_dir):
        os.makedirs(output_dir)
    
    # ログファイルの解析
    print(f"ログファイル '{log_file}' を解析中...")
    df = parse_log_file(log_file)
    
    if df.empty:
        print("ログファイルからデータを抽出できませんでした。")
        return
    
    # データの分析
    stats = analyze_lag_data(df)
    
    # グラフの作成
    print("\nグラフを生成中...")
    plot_lag_distribution(df, output_dir)
    
    # 推奨事項の生成
    recommendations = generate_recommendations(stats)
    
    # 推奨事項の出力
    print("\n=== 推奨対策 ===")
    for i, rec in enumerate(recommendations, 1):
        print(f"{i}. {rec}")
    
    # レポートファイルへの書き込み
    with open(os.path.join(output_dir, 'diagnosis_report.txt'), 'w') as f:
        f.write("=== Hyperliquid清算ボット（Liquidator）チャネルラグ診断レポート ===\n\n")
        
        f.write("=== 基本統計情報 ===\n")
        f.write(f"総イベント数: {stats['total_events']}\n")
        f.write(f"最大ラグ: {stats['max_lag']}\n")
        f.write(f"最小ラグ: {stats['min_lag']}\n")
        f.write(f"平均ラグ: {stats['mean_lag']:.2f}\n")
        f.write(f"中央値ラグ: {stats['median_lag']}\n\n")
        
        f.write("=== ラグの重大度分布 ===\n")
        f.write(f"重大 (>1000): {stats['critical_lags']} ({stats['critical_lags'] / stats['total_events'] * 100:.1f}%)\n")
        f.write(f"高 (101-1000): {stats['high_lags']} ({stats['high_lags'] / stats['total_events'] * 100:.1f}%)\n")
        f.write(f"中 (51-100): {stats['medium_lags']} ({stats['medium_lags'] / stats['total_events'] * 100:.1f}%)\n")
        f.write(f"低 (11-50): {stats['low_lags']} ({stats['low_lags'] / stats['total_events'] * 100:.1f}%)\n")
        f.write(f"最小 (1-10): {stats['minimal_lags']} ({stats['minimal_lags'] / stats['total_events'] * 100:.1f}%)\n\n")
        
        f.write("=== 推奨対策 ===\n")
        for i, rec in enumerate(recommendations, 1):
            f.write(f"{i}. {rec}\n")
        
        f.write("\n=== 実装推奨事項 ===\n")
        f.write("1. イベント駆動型+優先度キューアプローチ:\n")
        f.write("   - イベントの種類に応じた優先度付け\n")
        f.write("   - リアルタイムでの優先度更新\n")
        f.write("   - 優先度に基づくバッチ処理\n\n")
        
        f.write("2. マルチレベルスキャン+バッチ処理:\n")
        f.write("   - リスクレベルに基づく借り手の分類\n")
        f.write("   - リスクレベル別の並列処理\n")
        f.write("   - 動的バッチサイズの調整\n\n")
        
        f.write("3. インクリメンタルスキャン+キャッシュ戦略:\n")
        f.write("   - ヘルスファクター変化率による分類\n")
        f.write("   - 変化率に応じた差分更新\n")
        f.write("   - 動的TTL設定によるキャッシュ最適化\n")
    
    print(f"\nすべての診断結果は '{output_dir}' ディレクトリに保存されました。")

if __name__ == "__main__":
    main() 