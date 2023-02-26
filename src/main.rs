use crate::{
    request::Request,
    response::{EchoOkBody, InitOkBody, ResponseBody},
};
use request::{EchoBody, InitBody};
use tokio::io::{stdin, AsyncBufReadExt, BufReader};
mod request;
mod response;

#[tokio::main]
async fn main() {
    let stdin_buf_reader = BufReader::new(stdin());

    let mut stdin_lines_stream = stdin_buf_reader.lines();

    while let Some(line) = stdin_lines_stream.next_line().await.unwrap() {
        eprintln!("Processing stdin line: \n {line}");
        let request: Request = serde_json::from_str(&line).unwrap();

        tokio::task::spawn(async move {
            match &request.body {
                request::RequestBody::Init(init_body) => {
                    handle_init(&request, init_body).await;
                }
                request::RequestBody::Echo(echo_body) => {
                    handle_echo(&request, echo_body).await;
                }
            }
        });
    }
}

async fn handle_init(request: &Request, init_body: &InitBody) {
    eprintln!("Processing Init request from the maelstrom, with body {init_body:?}");

    let init_ok_body = InitOkBody {
        r#type: "init_ok".to_string(),
        in_reply_to: request.body.get_msg_id(),
    };

    request.reply(ResponseBody::InitOk(init_ok_body));
    eprintln!("Successfully replied with init_ok response");
}

async fn handle_echo(request: &Request, echo_body: &EchoBody) {
    eprintln!("Processing Echo request from the maelstrom, with body {echo_body:?}");

    let echo_ok_body = EchoOkBody {
        r#type: "echo_ok".to_string(),
        in_reply_to: request.body.get_msg_id(),
        echo: echo_body.echo.clone(),
    };

    request.reply(ResponseBody::EchoOk(echo_ok_body));
    eprintln!("Successfully replied with echo_ok response");
}
