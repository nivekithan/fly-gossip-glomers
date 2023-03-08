use crate::{
    request::{Request, RequestBody},
    response::{EchoOkBody, InitOkBody},
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
        let request: Request<RequestBody> = serde_json::from_str(&line).unwrap();

        tokio::task::spawn(async move {
            match &request.body {
                request::RequestBody::Init(init_body) => {
                    let init_request : Request<InitBody> = Request {
                        src: request.src,
                        dest: request.dest,
                        body: init_body.clone(),
                    };

                    handle_init(&init_request).await;
                }
                request::RequestBody::Echo(echo_body) => {
                    let echo_request : Request<EchoBody> = Request {
                        src: request.src,
                        dest: request.dest,
                        body: echo_body.clone(),
                    };
                    handle_echo(&echo_request).await;
                }
            }
        });
    }
}

async fn handle_init(request: &Request<InitBody>) {

    eprintln!("Processing Init request from the maelstrom, with body {request_body:?}", request_body = request.body);

    let init_ok_body = InitOkBody {
        r#type: "init_ok".to_string(),
        in_reply_to: request.body.msg_id,
    };


    request.reply(init_ok_body);
    eprintln!("Successfully replied with init_ok response");
}

async fn handle_echo(request: &Request<EchoBody>) {
    eprintln!("Processing Echo request from the maelstrom, with body {echo_body:?}", echo_body =  request.body);

    let echo_ok_body = EchoOkBody {
        r#type: "echo_ok".to_string(),
        in_reply_to: request.body.msg_id,
        echo: request.body.echo.clone(),
    };

    request.reply(echo_ok_body);
    eprintln!("Successfully replied with echo_ok response");
}
