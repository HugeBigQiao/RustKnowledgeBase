//! 网络通信: TcpListener / TcpStream / 简单 HTTP.

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

// ===== 1. TCP Echo 服务器 =====

/// 启动一个简单的 echo 服务器, 在后台线程运行.
fn demo_echo_server() {
    println!("--- TCP Echo 服务器 ---");

    // 绑定本地端口
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("  服务器监听: 127.0.0.1:8080");

    // 在单独线程中运行, 避免阻塞主线程
    thread::spawn(move || {
        // accept() 阻塞等待连接
        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    println!("  [服务端] 收到连接");
                    let mut buf = [0u8; 1024];
                    // 读取客户端数据
                    let n = stream.read(&mut buf).unwrap();
                    let received = String::from_utf8_lossy(&buf[..n]);
                    println!("  [服务端] 收到: {}", received.trim());
                    // 原样返回
                    stream.write_all(b"ECHO: ").unwrap();
                    stream.write_all(&buf[..n]).unwrap();
                    break; // 只处理一次请求
                }
                Err(e) => eprintln!("  连接错误: {}", e),
            }
        }
    });

    // 给服务器一点启动时间
    thread::sleep(std::time::Duration::from_millis(50));
}

// ===== 2. TCP 客户端 =====

/// 连接到 echo 服务器并发送消息.
fn demo_echo_client() {
    println!("\n--- TCP 客户端 ---");

    // 连接到服务器
    match TcpStream::connect("127.0.0.1:8080") {
        Ok(mut stream) => {
            println!("  已连接服务器");

            // 发送消息
            let msg = "Hello, Rust TCP!";
            stream.write_all(msg.as_bytes()).unwrap();
            println!("  发送: {}", msg);

            // 读取回复
            let mut buf = [0u8; 1024];
            let n = stream.read(&mut buf).unwrap();
            let response = String::from_utf8_lossy(&buf[..n]);
            println!("  收到: {}", response.trim());
        }
        Err(e) => {
            // 如果服务器线程还没准备好, 这里就会出错
            // 实际项目需要重试机制
            println!("  连接失败: {} (服务器可能尚未就绪)", e);
        }
    }
}

// ===== 3. UDP 基本用法 =====

use std::net::UdpSocket;

/// UDP 是面向数据报的无连接协议.
fn demo_udp() {
    println!("\n--- UDP 数据报 ---");

    // 绑定本地端口
    let socket = UdpSocket::bind("127.0.0.1:0").unwrap(); // 0 = 随机端口
    println!("  UDP socket 已绑定: {}", socket.local_addr().unwrap());

    // 发送数据报到本地
    socket.send_to(b"Hello UDP", "127.0.0.1:9090").unwrap();
    println!("  已发送数据报到 127.0.0.1:9090");

    // UDP 不保证送达、不保证顺序、不保证不重复
    // 适用于: 实时音视频、DNS 查询、游戏状态同步
}

// ===== 4. DNS 解析 =====

use std::net::ToSocketAddrs;

/// 域名 → IP 地址解析.
fn demo_dns() {
    println!("\n--- DNS 解析 ---");

    // ToSocketAddrs trait: 将字符串解析为 SocketAddr 列表
    let addrs: Vec<_> = "rust-lang.org:443"
        .to_socket_addrs()
        .unwrap()
        .collect();

    for addr in &addrs {
        println!("  rust-lang.org:443 → {}", addr);
    }

    // 也支持解析本机
    if let Ok(addrs) = "localhost:80".to_socket_addrs() {
        println!("  localhost:80 → {:?}", addrs.collect::<Vec<_>>());
    }
}

// ===== 5. 简单 HTTP GET =====

/// 用裸 TCP 发送 HTTP/1.0 请求 (教学目的, 真实项目用 reqwest).
fn demo_http_get() {
    println!("\n--- 简单 HTTP GET ---");

    // 连接到 rust-lang.org 的 80 端口
    match TcpStream::connect("rust-lang.org:80") {
        Ok(mut stream) => {
            // 手工构造 HTTP 请求
            let request = "GET / HTTP/1.0\r\nHost: rust-lang.org\r\nConnection: close\r\n\r\n";
            stream.write_all(request.as_bytes()).unwrap();

            // 读取响应
            let mut response = String::new();
            stream.read_to_string(&mut response).unwrap();

            // 只打印状态行和前几行响应头
            let first_lines: Vec<&str> = response.lines().take(8).collect();
            for line in &first_lines {
                println!("  {}", line);
            }
            if response.lines().count() > 8 {
                println!("  ... (省略剩余 {} 行)", response.lines().count() - 8);
            }
        }
        Err(e) => {
            println!("  请求失败: {} (可能需要网络连接)", e);
        }
    }

    // 注意: 这是最原始的 HTTP 客户端, 没有:
    // - HTTPS/TLS 支持 (需要 443 端口 + TLS 握手)
    // - 自动重定向、Cookie、Keep-Alive
    // - 连接池、超时控制
    // 实际项目建议: reqwest (HTTP 客户端库)
}

pub fn run() {
    demo_echo_server();
    // 给服务器启动时间
    thread::sleep(std::time::Duration::from_millis(100));
    demo_echo_client();
    demo_udp();
    demo_dns();
    demo_http_get();
    // 等待后台服务器线程结束
    thread::sleep(std::time::Duration::from_millis(200));
}
