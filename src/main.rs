use std::{
    sync::{Arc, Mutex},
    time::{Duration, SystemTime},
};

use crate::{
    request::{BroadcastBody, ReadBody, Request, RequestBody, TopologyBody},
    response::{EchoOkBody, GenerateOkBody, InitOkBody, ReadOkBody, TopologyOkBody},
};
use request::{EchoBody, GenerateBody, InitBody};
use response::BrodcastOkBody;
use tokio::io::{stdin, AsyncBufReadExt, BufReader};
mod request;
mod response;

#[tokio::main]
async fn main() {
    let stdin_buf_reader = BufReader::new(stdin());
    let mut stdin_lines_stream = stdin_buf_reader.lines();
    let machine_config: SharedMachineConfig = Arc::new(tokio::sync::RwLock::new(None));
    let sequence: SharedSequenceNum = Arc::new(Mutex::new(0));
    let messages: SharedMessages = Arc::new(Mutex::new(Vec::new()));

    while let Some(line) = stdin_lines_stream.next_line().await.unwrap() {
        eprintln!("Processing stdin line: \n {line}");
        let request: Request<RequestBody> = serde_json::from_str(&line).unwrap();
        let machine_config = Arc::clone(&machine_config);
        let sequence = Arc::clone(&sequence);
        let messages = Arc::clone(&messages);

        tokio::task::spawn(async move {
            match &request.body {
                request::RequestBody::init(init_body) => {
                    let init_request: Request<InitBody> = Request {
                        src: request.src,
                        dest: request.dest,
                        body: init_body.clone(),
                    };

                    handle_init(&init_request, machine_config).await;
                }
                request::RequestBody::echo(echo_body) => {
                    let echo_request: Request<EchoBody> = Request {
                        src: request.src,
                        dest: request.dest,
                        body: echo_body.clone(),
                    };
                    handle_echo(&echo_request).await;
                }
                request::RequestBody::generate(generate_body) => {
                    let generate_request: Request<GenerateBody> = Request {
                        src: request.src,
                        dest: request.dest,
                        body: generate_body.clone(),
                    };
                    handle_generate(&generate_request, machine_config, sequence).await;
                }
                request::RequestBody::broadcast(brodcast_body) => {
                    let broadcast_request: Request<BroadcastBody> = Request {
                        src: request.src,
                        dest: request.dest,
                        body: brodcast_body.clone(),
                    };

                    handle_broadcast(broadcast_request, messages).await;
                }
                request::RequestBody::read(read_body) => {
                    let read_request: Request<ReadBody> = Request {
                        src: request.src,
                        dest: request.dest,
                        body: read_body.clone(),
                    };

                    handle_read(read_request, messages).await;
                }
                request::RequestBody::topology(topolody_body) => {
                    let toplogy_request: Request<TopologyBody> = Request {
                        src: request.src,
                        dest: request.dest,
                        body: topolody_body.clone(),
                    };
                    
                    handle_topology(toplogy_request).await;
                }
            }
        });
    }
}

async fn handle_init(request: &Request<InitBody>, machine_config: SharedMachineConfig) {
    eprintln!(
        "Processing Init request from the maelstrom, with body {request_body:?}",
        request_body = request.body
    );

    let init_ok_body = InitOkBody {
        r#type: "init_ok".to_string(),
        in_reply_to: request.body.msg_id,
    };
    let mut machine_config = machine_config.write().await;

    *machine_config = Some(MachineConfig {
        node_id: request.body.node_id.clone(),
    });

    request.reply(init_ok_body);
    eprintln!("Successfully replied with init_ok response");
}

async fn handle_echo(request: &Request<EchoBody>) {
    eprintln!(
        "Processing Echo request from the maelstrom, with body {echo_body:?}",
        echo_body = request.body
    );

    let echo_ok_body = EchoOkBody {
        r#type: "echo_ok".to_string(),
        in_reply_to: request.body.msg_id,
        echo: request.body.echo.clone(),
    };

    request.reply(echo_ok_body);
    eprintln!("Successfully replied with echo_ok response");
}

async fn handle_generate(
    request: &Request<GenerateBody>,
    machine_config: SharedMachineConfig,
    sequence: SharedSequenceNum,
) {
    eprintln!(
        "Processing Generate request from maelstrom, with body {generate_body:?}",
        generate_body = request.body
    );

    let unix_timestamp_for_2023 = 1672531200; // This timestamp species Jan 1, 2023 00:00:00 UTC
    let snowflake = SystemTime::UNIX_EPOCH + Duration::new(unix_timestamp_for_2023, 0);

    let node_id = {
        let machine_config = machine_config.read().await;

        match &*machine_config {
            Some(machine_config) => machine_config.node_id.clone(),
            None => unreachable!(),
        }
    };

    let timestamp = SystemTime::now()
        .duration_since(snowflake)
        .unwrap()
        .as_millis();

    let unique_id = {
        let mut sequence = sequence.lock().unwrap();
        let unique_id = format!("{timestamp}{node_id}{sequence}",);
        *sequence += 1;
        unique_id
    };

    let generate_ok_body = GenerateOkBody {
        id: unique_id,
        r#type: String::from("generate_ok"),
        in_reply_to: request.body.msg_id,
    };
    request.reply(generate_ok_body);
    eprintln!("Sucessfully replied with generate_ok response");
}

async fn handle_broadcast(request: Request<BroadcastBody>, messages: SharedMessages) {
    eprintln!(
        "Handling broadcast message from maelstrom with body {broadcast_body:?}",
        broadcast_body = request.body
    );
    let mut messages = messages.lock().unwrap();

    messages.push(request.body.message);

    request.reply(BrodcastOkBody {
        r#type: "broadcast_ok".to_string(),
        in_reply_to: request.body.msg_id,
    });

    eprintln!("Successfully replied with broadcast_ok message");
}

async fn handle_read(request: Request<ReadBody>, messages: SharedMessages) {
    eprintln!(
        "Handling read message from maelstrom with body {read_body:?}",
        read_body = request.body
    );

    let messages = messages.lock().unwrap();

    request.reply(ReadOkBody {
        r#type: "read_ok".to_string(),
        in_reply_to: request.body.msg_id,
        messages: messages.clone(),
    });

    eprintln!("Successfully replied with read_ok message");
}

async fn handle_topology(request: Request<TopologyBody>) {
    eprintln!(
        "Handling read message from maelstrom with body {topology_body:?}",
        topology_body = request.body
    );

    request.reply(TopologyOkBody {
        r#type: "topology_ok".to_string(),
        in_reply_to: request.body.msg_id,
    });

    eprintln!("Sucessfully replied with topology_ok message");
}

struct MachineConfig {
    pub node_id: String,
}

type SharedMachineConfig = Arc<tokio::sync::RwLock<Option<MachineConfig>>>;
type SharedSequenceNum = Arc<Mutex<usize>>;
type SharedMessages = Arc<Mutex<Vec<usize>>>;
