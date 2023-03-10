use std::{collections::HashMap, fmt::Debug};

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
#[serde(tag = "type")]
#[allow(non_camel_case_types)]
pub enum RequestBody {
    init(InitBody),
    echo(EchoBody),
    generate(GenerateBody),
    broadcast(BroadcastBody),
    read(ReadBody),
    topology(TopologyBody),
}

#[derive(Deserialize, Debug, Clone)]

pub struct InitBody {
    pub msg_id: usize,
    pub node_id: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct EchoBody {
    pub msg_id: usize,
    pub echo: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct GenerateBody {
    pub msg_id: usize,
}

#[derive(Deserialize, Debug, Clone)]
pub struct BroadcastBody {
    pub msg_id: usize,
    pub message: usize,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ReadBody {
    pub msg_id: usize,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TopologyBody {
    pub msg_id: usize,
    pub topology: HashMap<String, Vec<String>>,
}
