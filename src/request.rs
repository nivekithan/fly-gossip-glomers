use serde::Deserialize;

use crate::response::{Response, ResponseBody};

#[derive(Deserialize, Debug)]
pub struct Request {
    src: String,
    dest: String,
    pub body: RequestBody,
}

impl Request {
    pub fn reply(self, body: ResponseBody) {
        let response = Response {
            src: self.dest,
            dest: self.src,
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
}

#[derive(Deserialize, Debug)]

pub struct InitBody {
    pub r#type: String,
    pub msg_id: usize,
    pub node_id: String,
}
