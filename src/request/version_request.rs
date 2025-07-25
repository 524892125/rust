use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct FormParams {
    pub clientVersion: String,
    pub channel: String,
    pub deviceNo: String, // 可选参数
}