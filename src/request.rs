use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use crate::response::Response;

#[derive(Deserialize, Debug)]
pub struct Request<Body> {
    pub src: String,
    pub dest: String,
    pub body: Body,
}

impl<Body> Request<Body> {
    pub fn reply<ResponseBody: Serialize + std::fmt::Debug>(&self, body: ResponseBody) {
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
    Generate(GenerateBody),
}

#[derive(Deserialize, Debug, Clone)]

pub struct InitBody {
    pub r#type: String,
    pub msg_id: usize,
    pub node_id: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct EchoBody {
    pub r#type: String,
    pub msg_id: usize,
    pub echo: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct GenerateBody {
    pub r#type: String,
    pub msg_id : usize,
}
