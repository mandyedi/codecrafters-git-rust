#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        print_help();
        return;
    }

    match args[1].as_str() {
        "init" => init(),
        _ => print_help(),
    }
}

fn init() {
    if !Path::new(".git").is_dir() {
        fs::create_dir(".git").unwrap();
        fs::create_dir(".git/objects").unwrap();
        fs::create_dir(".git/refs").unwrap();
        fs::write(".git/HEAD", "ref: refs/heads/main\n").unwrap();
        println!("Initialized git directory.")
    }
    else {
        println!("Git repository already initialized.");
    }
}

fn print_help() {
    println!("usage: git <command>");
    println!("Git commands to use:\n");
    println!("init    Create an empty Git repository.");
}