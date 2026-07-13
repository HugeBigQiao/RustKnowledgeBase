//! 程序入口: 只负责 CLI 派发, 不包含任何业务逻辑.

fn main() {
    let arg = std::env::args().nth(1);

    match arg.as_deref() {
        Some("demo") | None => intermediate_example::service::demo::run(),
        Some(other) => {
            println!("未知命令: {}", other);
            print_help();
        }
    }
}

fn print_help() {
    println!("用法: cargo run -- [命令]");
    println!("命令:");
    println!("  demo (默认) : 运行完整演示");
    println!();
    println!("也可以作为库在其他项目中使用:");
    println!("  intermediate_example = {{ path = \"../intermediate_example\" }}");
    println!("  use intermediate_example::models::library::Library;");
}
