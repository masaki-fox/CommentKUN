mod bouyomi;
mod gemini;
mod models;

use reqwest::Client;
use std::env;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let api_key = env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY が設定されていません");
    let http_client = Client::new();

    println!("🎙️ あなたに質問するAIアナウンサーBotが起動しました！");

    // トークテーマの候補（ランダムに選んだり切り替えたりできます）
    let topics = [
        "雑談・日常の話題",
        "今やっているゲームの感想や状況",
        "最近買ってよかったものやハマっていること",
        "今日の出来事や食べたもの",
    ];

    let mut index = 0;

    loop {
        let current_topic = topics[index % topics.len()];
        println!("\n[質問生成中...] テーマ: {}", current_topic);

        trigger_question(&http_client, &api_key, current_topic).await;

        index += 1;

        // 次の質問までの間隔（例: 45秒間隔。お好みで変更してください）
        tokio::time::sleep(Duration::from_secs(45)).await;
    }
}
async fn trigger_question(client: &Client, api_key: &str, topic: &str) {
    match gemini::ask_question(client, api_key, topic).await {
        Ok(question) => {
            println!("🤖 AI: {}", question);
            if let Err(e) = bouyomi::speak(&question).await {
                eprintln!("棒読みちゃん送信エラー: {}", e);
            }
        }
        Err(e) => {
            // エラーが出てもログだけ軽く出して、そのまま次の周期へスキップ！
            println!("⚠️ スキップしました: {}", e);
        }
    }
}
