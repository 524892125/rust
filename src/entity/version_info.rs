use serde::{Deserialize, Deserializer, Serialize};

fn bool_from_int<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let i = i32::deserialize(deserializer)?;
    Ok(i != 0)
}


#[derive(Debug, Serialize, Deserialize)]
pub struct VersionInfo {
    pub ResourceVersion: String,
    pub CodeVersion: String,
    pub DataTableVersion: String,
    pub AudioVersion: String,
    pub InternalGameVersion: String,
    pub InternalResourceVersion: String,
    pub InternalCodeVersion: String,
    pub InternalDataTableVersion: String,
    #[serde(deserialize_with = "bool_from_int")]
    pub AppleReview: bool,
    pub OssPath: String,
    pub AppUrl: String,
    #[serde(rename = "PathRoot")] // 修正 JSON 字段和结构体字段名不一致的问题
    pub RootPath: String,
    pub ParadoxVersion: String,
    pub InternalParadoxVersion: String,
    #[serde(default)]
    pub Code: i32,
}