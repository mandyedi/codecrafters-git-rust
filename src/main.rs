use std::env;
use std::fs;
use std::io::Read;
use std::path::Path;
use flate2::read::ZlibDecoder;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        print_help();
        return;
    }

    match args[1].as_str() {
        "init" => init(),
        "cat-file" => cat_file(&args),
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

fn cat_file(args: &Vec<String>) {
    if args.len() < 4 {
        println!("usage: git cat-file -p <blob_sha>");
        return;
    }

    if args[2] == "-p" {
        let blob_sha= &args[3];
        let id = &blob_sha[..2];
        let blob_file_name = &blob_sha[2..];

        let blob = fs::read(format!(".git/objects/{}/{}", id, blob_file_name)).unwrap();
        let mut decompressed_content = ZlibDecoder::new(&blob[..]);
        let mut content_string = String::new();
        decompressed_content.read_to_string(&mut content_string).unwrap();
        
        let content = content_string.split('\0').nth(1).unwrap_or("default");
        print!("{content}");
    }
}

fn print_help() {
    println!("usage: git <command>");
    println!("Git commands to use:\n");
    println!("init                      Create an empty Git repository.");
    println!("cat-file -p <blob_sha>    Print blob content.")
}