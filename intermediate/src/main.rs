//! intermediate: 中级 Rust 概念，从结构体和枚举开始。

mod collections;
mod database;
mod error_handling;
mod file_io;
mod generics;
mod lifetimes;
mod macros_intro;
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
        Some("file_io") => file_io::run(),
        Some("database") => database::run(),
        Some("macros_intro") => macros_intro::run(),
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
        "  structs_and_enums, patterns, option, error_handling, generics, traits,"
    );
    println!(
        "  collections, static_and_const, lifetimes, file_io, database, macros_intro"
    );
}
