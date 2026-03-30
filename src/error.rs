#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("IO错误 {0}")]
    IO(#[from] std::io::Error),

    #[error("CMPP错误 错误码：{0} 说明：{1}")]
    CMPP(i32, String),
}
