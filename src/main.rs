use anyhow::Result;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> Result<()> {

    let mut listener = TcpListener::bind("127.0.0.1:6379").await?;

    let ping_response = b"+PONG\r\n";

    loop {
        let incoming = listener.accept().await;
        match incoming {
            ok(mut strem, _) => {
                tokio::spawn(async move {
                    handle_connection(&mut stream).await.unwrap();
                })
            },
            Err(e) => {
                println!("Error: {}", e)
            }
        }
    }
}

async fn handle_connection(stream: &mut TcpStream) -> Result<()> {
    let mut buf = [0;512];

    let response = b"+PONG\r\n";

    loop {
        let b_read = stream.read(&mut buf).await?;
        if b_read == 0 {
            println!("Client closed the connection");
            break;
        }

        stream.write(response).await?
    }

    Ok(())
}
