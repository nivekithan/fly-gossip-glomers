use serde::Serialize;

#[derive(serde::Serialize, Debug)]
pub struct Response<Body: Serialize + std::fmt::Debug> {
    pub src: String,
    pub dest: String,
    pub body: Body,
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

#[derive(serde::Serialize, Debug)]
pub struct GenerateOkBody {
    pub r#type: String,
    pub id: String,
    pub in_reply_to: usize,
}

#[derive(serde::Serialize, Debug)]
pub struct BrodcastOkBody {
    pub r#type: String,
    pub in_reply_to: usize,
}

#[derive(serde::Serialize, Debug)]
pub struct ReadOkBody {
    pub r#type: String,
    pub in_reply_to: usize,
    pub messages: Vec<usize>,
}

#[derive(serde::Serialize, Debug)]
pub struct TopologyOkBody {
    pub r#type: String,
    pub in_reply_to: usize,
}
