#[derive(serde::Serialize, Debug)]
pub struct Response {
    pub src: String,
    pub dest: String,
    pub body: ResponseBody,
}

#[derive(serde::Serialize, Debug)]
#[serde(untagged)]
pub enum ResponseBody {
    InitOk(InitOkBody),
    EchoOk(EchoOkBody),
}

#[derive(serde::Serialize, Debug)]
pub struct InitOkBody {
    pub r#type: String,
    pub in_reply_to: usize,
}

#[derive(serde::Serialize, Debug)]
pub struct EchoOkBody {
    pub r#type: String,
    pub in_reply_to: usize,
    pub echo: String,
}
