
use std::fs;
use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;

fn main() {
    // 設置監聽接口及端口
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // 處理請求事件
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        // 取得 html 內容並 render 至畫面上
        handle_connection(stream);
    }
}

// 取得 html 內容並 render 至畫面上
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let contents = get_html_string();

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

// 取得 html 內容
fn get_html_string() -> String {
    let contents = match fs::read_to_string("hello.html") {
        Ok(contents) => contents,
        Err(_e) => String::from("")
    };

    return contents;
}