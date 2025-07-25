use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct VersionInfo {
    pub ResourceVersion: String,
    pub CodeVersion: String,
    pub DataTableVersion: String,
    pub AudioVersion: String,
    pub InternalGameVersion: i32,
    pub InternalResourceVersion: i32,
    pub InternalCodeVersion: i32,
    pub InternalDataTableVersion: i32,
    pub AppleReview: bool,
    pub OssPath: String,
    pub AppUrl: String,
    pub RootPath: String,
    pub ParadoxVersion: String,
    pub InternalParadoxVersion: i32,
    pub Code: i32,
}