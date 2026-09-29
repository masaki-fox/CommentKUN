//Gemini API用のデータ構造定義

use serde::{Deserialize, Serialize};

// --- リクエスト用 ---
#[derive(Serialize)]
pub struct GeminiRequest {
    pub contents: Vec<Content>,
    pub system_instruction: SystemInstruction,
}

#[derive(Serialize)]
pub struct SystemInstruction {
    pub parts: Vec<Part>,
}

#[derive(Serialize)]
pub struct Content {
    pub parts: Vec<Part>,
}

// --- レスポンス用 ---
#[derive(Deserialize)]
pub struct GeminiResponse {
    pub candidates: Option<Vec<Candidate>>,
}

#[derive(Deserialize)]
pub struct Candidate {
    pub content: ContentResponse,
}

#[derive(Deserialize)]
pub struct ContentResponse {
    pub parts: Vec<Part>,
}

// 共通パーツ
#[derive(Serialize, Deserialize, Debug)]
pub struct Part {
    pub text: String,
}
