use bytes::Bytes;
use clap::Parser;
use futures::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_util::codec::{Framed, LengthDelimitedCodec};
use colored::Colorize;

#[derive(Parser, Debug)]
struct Args {
    #[clap(short, long)]
    command: String,
    #[clap(short, long,default_value = "127.0.0.1:8888")]
    address: String,
}

#[tokio::main]
async fn main() {
    // get address
    let args = Args::parse();
    let addrs = args.address;
    let command = args.command;

    // creat client
    let stream = TcpStream::connect(&addrs).await.expect("stream error");

    // creat frame stream
    let mut framed_stream = Framed::new(stream, LengthDelimitedCodec::new());

    // send command
    _ = framed_stream.send(Bytes::from(command)).await;

    // get response and print output
    if let Some(msg) = framed_stream.next().await {
        match msg {
            Ok(output) => {
                let output = String::from_utf8(output.to_vec()).unwrap();
                println!("{}{}","ouput:\n".purple() ,output.green());
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}
