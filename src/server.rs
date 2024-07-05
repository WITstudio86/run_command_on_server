use anyhow::{Error, Result};
use bytes::Bytes;
use futures::{SinkExt, StreamExt};
use std::env::args;
use tokio::{net::TcpListener, process};
use tokio_util::codec::{Framed, LengthDelimitedCodec};

#[tokio::main]
async fn main() -> Result<(), Error> {
    // read args
    let addrs = args().nth(1).unwrap_or_else(|| "127.0.0.1:8888".to_owned());
    println!("target address: {}", addrs);

    // creat linsterner
    let linstener = TcpListener::bind(&addrs)
        .await
        .expect("linterner binding error");
    println!("binded on {}", addrs);

    loop {
        // creat frame stream
        let (stream, _) = linstener.accept().await?;

        // creat framed stream
        let mut framed_stream = Framed::new(stream, LengthDelimitedCodec::new());

        tokio::spawn(async move{
            while let Some(msg) = framed_stream.next().await {
                match msg {
                    Ok(command) => {
                        // read command from bytes
                        let command = String::from_utf8(command.to_vec()).unwrap();
                        println!("command: {}", command);
                        let output = process(&command).await;
                        println!("output: {}", output);
                        _ = framed_stream.send(Bytes::from(output)).await;
                    }
                    Err(e) => {
                        println!("Error: {e}");
                    }
                }
            }
        });
    }
}

async fn process(command: &str) -> String {
    let output = process::Command::new(command).output().await;
    if let Ok(output) = output {
        String::from_utf8(output.stdout.to_vec()).unwrap()
    } else {
        "invalid command".parse().unwrap()
    }
}
