use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SizeOption {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InitOptions {
    pub frame_duration: u32,
    pub size: SizeOption,
}
