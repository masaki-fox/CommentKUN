//棒読み連携

use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

pub async fn speak(text: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = TcpStream::connect("127.0.0.1:50001").await?;

    let command: u16 = 0x0001; // 読み上げ
    let speed: i16 = -1; // 速度 (デフォルト: -1)
    let tone: i16 = -1; // 音程 (デフォルト: -1)
    let volume: i16 = -1; // 音量 (デフォルト: -1)
    let voice: u16 = 0; // 声質 (デフォルト: 0)
    let encoding: u8 = 0; // 文字コード (0: UTF-8)

    let text_bytes = text.as_bytes();
    let length = text_bytes.len() as u32;

    let mut packet = Vec::new();
    packet.extend_from_slice(&command.to_le_bytes());
    packet.extend_from_slice(&speed.to_le_bytes());
    packet.extend_from_slice(&tone.to_le_bytes());
    packet.extend_from_slice(&volume.to_le_bytes());
    packet.extend_from_slice(&voice.to_le_bytes());
    packet.push(encoding);
    packet.extend_from_slice(&length.to_le_bytes());
    packet.extend_from_slice(text_bytes);

    stream.write_all(&packet).await?;
    Ok(())
}
