#[derive(serde::Serialize)]
pub struct Response {
    pub src: String,
    pub dest: String,
    pub body: ResponseBody,
}

#[derive(serde::Serialize)]
#[serde(untagged)]
pub enum ResponseBody {
    InitOk(InitOkBody),
}

#[derive(serde::Serialize)]
pub struct InitOkBody {
    pub r#type: String,
    pub in_reply_to: usize,
}
