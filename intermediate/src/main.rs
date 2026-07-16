//! intermediate: 中级 Rust 概念，从结构体和枚举开始。

mod collections;
mod error_handling;
mod generics;
mod lifetimes;
mod option;
mod patterns;
mod sandbox;
mod static_and_const;
mod structs_and_enums;
mod traits;

fn main() {
    let arg = std::env::args().nth(1);

    match arg.as_deref() {
        Some("structs_and_enums") => structs_and_enums::run(),
        Some("patterns") => patterns::run(),
        Some("option") => option::run(),
        Some("error_handling") => error_handling::run(),
        Some("generics") => generics::run(),
        Some("traits") => traits::run(),
        Some("collections") => collections::run(),
        Some("static_and_const") => static_and_const::run(),
        Some("lifetimes") => lifetimes::run(),
        Some("sandbox") => sandbox::run(),
        Some(other) => {
            println!("未知模块: {}", other);
            print_help();
        }
        None => print_help(),
    }
}

fn print_help() {
    println!("用法: cargo run -- <模块名>");
    println!("可用模块:");
    println!(
        "  structs_and_enums, patterns, option, error_handling, generics, traits, vec_advanced, collections, static_and_const, lifetimes"
    );
}
