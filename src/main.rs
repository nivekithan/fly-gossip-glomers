use crate::{
    request::Request,
    response::{InitOkBody, ResponseBody},
};
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

        match &request.body {
            request::RequestBody::Init(init_body) => {
                eprintln!("Processing Init request from the maelstrom, with body {init_body:?}");

                let init_ok_body = InitOkBody {
                    r#type: "init_ok".to_string(),
                    in_reply_to: init_body.msg_id,
                };

                request.reply(ResponseBody::InitOk(init_ok_body));
                eprintln!("Successfully replied with init_ok response");
            }
        }
    }
}

