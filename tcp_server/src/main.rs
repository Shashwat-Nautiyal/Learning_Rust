// use std::io::{Read, Write};
// use std::net::{TcpListener, TcpStream};


// synchronous tcp-server with multi-threading

// fn handle_client(mut stream: TcpStream) {
//     // Buffer Array
//     let mut buffer = [0; 1024];
//     // Reading raw bytes into the buffer
//     if let Ok(bytes_read) = stream.read(&mut buffer) {
//         if bytes_read == 0 {
//             return; // connection closed
//         }
//         let req = String::from_utf8_lossy(&buffer[..bytes_read]);
//         println!("Request: {}", req);
//         // Proper HTTP response
//         let body = "Hey!! Shashwat";
//         let response = format!(
//             "HTTP/1.1 200 OK\r\n\
//              Content-Length: {}\r\n\
//              Content-Type: text/plain\r\n\
//              Connection: close\r\n\
//              \r\n\
//              {}",
//             body.len(),
//             body
//         );
//         stream.write_all(response.as_bytes()).unwrap();
//         stream.flush().unwrap();
//     }
// }

// fn main(){
//     let listener = TcpListener::bind("127.0.0.1:8080").expect("failed to bind to the localhost");
//     println!("Server listening at: 127.0.0.1:8080");
//     for stream in listener.incoming(){
//         match stream{
//             Ok(stream) => {
//                 std::thread::spawn(|| handle_client(stream));
//             }
//             Err(e) => {
//                 eprintln!("failed to establish connection: {}", e);
//             }
//         }
//     }
// }


// asynchronous tcp-server

use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

async fn handle_client(mut stream: TcpStream){
    let mut buffer = [0;1024];

    loop{
        match stream.read(&mut buffer).await {
            Ok(0) => {
                println!("client discoonected\n");
                return;
            }
            Ok(_) =>{
                let body = "Hello from Rust HTTP Server!\n";
                // let response = format!(
                //     "HTTP/1.1 200 OK\r\n\
                //     Content-Length: {}\r\n\
                //     Content-Type: text/plain\r\n\
                //     Connection: close\r\n\
                //     \r\n\
                //     {}",
                //     body.len(),
                //     body
                // );

                if let Err(e) = stream.write_all(body.as_bytes()).await {
                    eprintln!("Failed to send response: {}", e);
                }
            }
            Err(e) => {
                eprintln!("Failed: {}", e);
            }
        }
    }
    
}

#[tokio::main]
async fn main()-> Result<(), Box<dyn std::error::Error>>{
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server listening on 127.0.0.1:8080");

    loop {
        let (stream, addr) = listener.accept().await?;
        println!("New client: {}", addr);

        tokio::spawn(async move {
            handle_client(stream).await; 
        });


    }
}

// use tokio::net::{TcpListener, TcpStream};
// use tokio::io::{AsyncReadExt, AsyncWriteExt};

// async fn handle_client(mut stream: TcpStream) {
//     let mut buffer = [0u8; 1024];

//     loop {
//         match stream.read(&mut buffer).await {
//             Ok(0) => {
//                 println!("Client disconnected");
//                 return;
//             }
//             Ok(n) => {
//                 if let Err(e) = stream.write_all(&buffer[..n]).await {
//                     eprintln!("Failed to write: {}", e);
//                     return;
//                 }
//             }
//             Err(e) => {
//                 eprintln!("Read error: {}", e);
//                 return;
//             }
//         }
//     }
// }

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let listener = TcpListener::bind("127.0.0.1:8080").await?;
//     println!("Server listening on 127.0.0.1:8080");

//     loop {
//         let (stream, addr) = listener.accept().await?;
//         println!("New client: {}", addr);

//         tokio::spawn(async move {
//             handle_client(stream).await;
//         });
//     }
// }
