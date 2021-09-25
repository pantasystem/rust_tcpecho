use std::error::Error;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::{env, str, thread};

fn main() -> Result<(), Box<dyn Error>>{
    let args: Vec<String> = env::args().collect();
    let addr = &args[1];
    echo_server(addr)?;
    return Ok(());
}

fn echo_server(address: &str) -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind(address)?;
    loop {
        // 接続待ち受け
        let (mut stream, _) = listener.accept()?;

        // 接続を受けると新たにスレッドを作成して入力を受け付ける
        thread::spawn(move || {
           let mut buffer = [0u8; 1024];
           loop {
               // 接続したstreamから入力を受け付ける
               let nbytes = stream.read(&mut buffer).unwrap();
               if nbytes == 0 {
                   return;
               }
               print!("{}", str::from_utf8(&buffer[..nbytes]).unwrap());

               // クライアントから受けた入力を返す
               stream.write_all(&buffer[..nbytes]).unwrap();
           } 
        });
    }
}