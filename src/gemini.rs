use crate::models::{Content, GeminiRequest, GeminiResponse, Part, SystemInstruction};
use reqwest::Client;

// models/ プレフィックスを明記（すべてハイフン区切り）
const TARGET_MODEL: &str = "gemini-flash-latest";

pub async fn ask_question(
    client: &Client,
    api_key: &str,
    topic: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let body = GeminiRequest {
        system_instruction: SystemInstruction {
            parts: vec![Part {
                text: "あなたはライブ配信の司会者・アシスタントAIです。配信者が楽しくトークできるように、配信者へ向けたシンプルで答えやすい質問（20〜30文字程度）を1つ投げかけてください。記号や絵文字は含めないでください。".to_string(),
            }],
        },
        contents: vec![Content {
            parts: vec![Part {
                text: format!("トークのテーマ/状況: {}\n配信者への質問を作成してください。", topic),
            }],
        }],
    };

    // URLの {}:generateContent の部分に TARGET_MODEL ("models/gemini-2.5-flash-lite") が直接入ります
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
        TARGET_MODEL, api_key
    );

    // リクエスト送信（1度だけ試し、失敗したらすぐスキップ）
    let res = client.post(&url).json(&body).send().await?;
    let status = res.status();
    let text_body = res.text().await?;

    if status.is_success() {
        if let Ok(res_data) = serde_json::from_str::<GeminiResponse>(&text_body) {
            if let Some(candidates) = res_data.candidates {
                if let Some(first_candidate) = candidates.get(0) {
                    if let Some(first_part) = first_candidate.content.parts.get(0) {
                        return Ok(first_part.text.trim().to_string());
                    }
                }
            }
        }
    }

    // 429や503、404等のエラーが発生した場合は待たずに即スキップ
    Err(format!("APIエラー ({}) によりスキップします", status).into())
}
