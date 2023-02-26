use serde::Deserialize;

use crate::response::{Response, ResponseBody};

#[derive(Deserialize, Debug)]
pub struct Request {
    src: String,
    dest: String,
    pub body: RequestBody,
}

impl Request {
    pub fn reply(&self, body: ResponseBody) {
        let response = Response {
            src: self.dest.clone(),
            dest: self.src.clone(),
            body: body,
        };

        let response_in_json = serde_json::to_string(&response).unwrap();
        println!("{response_in_json}")
    }
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum RequestBody {
    Init(InitBody),
    Echo(EchoBody),
}

impl RequestBody {
    pub fn get_msg_id(&self) -> usize {
        match self {
            RequestBody::Init(init_body) => init_body.msg_id,
            RequestBody::Echo(echo_body) => echo_body.msg_id,
        }
    }
}

#[derive(Deserialize, Debug)]

pub struct InitBody {
    pub r#type: String,
    pub msg_id: usize,
    pub node_id: String,
}

#[derive(Deserialize, Debug)]
pub struct EchoBody {
    pub r#type: String,
    pub msg_id: usize,
    pub echo: String,
}
