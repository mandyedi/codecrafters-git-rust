use std::env;
use std::fs;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use flate2::{Compression, {read::ZlibDecoder, write::ZlibEncoder}};
use sha1::{Sha1, Digest};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        print_help();
        return;
    }

    match args[1].as_str() {
        "init" => init(),
        "cat-file" => cat_file(&args),
        "hash-object" => hash_object(&args),
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
        
        let content = content_string.split('\0').nth(1).unwrap_or("");
        print!("{content}");
    }
}

fn hash_object(args: &Vec<String>) {
    if args.len() < 4 {
        println!("usage: git hash-object -w <file>");
        return;
    }

    // read text file
    let file_content = fs::read_to_string(&args[3]).unwrap_or("".to_string());

    // create blob
    let length = file_content.len();
    let blob = format!("blob {}\0{}", length, file_content);

    // hash blob and print
    let hasher = Sha1::new_with_prefix(&blob);
    let hash = format!("{:x}", hasher.finalize());
    print!("{hash}");

    // zip blob
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(blob.as_bytes()).unwrap();
    let compressed_blob = encoder.finish().unwrap();

    // write blob to file
    let folder_name = &hash[..2];
    let remaining = &hash[2..];
    fs::create_dir(format!(".git/objects/{}", folder_name)).unwrap();
    fs::write(format!(".git/objects/{}/{}", folder_name, remaining), compressed_blob).unwrap();
}

fn print_help() {
    println!("usage: git <command>");
    println!("Git commands to use:\n");
    println!("init                      Create an empty Git repository.");
    println!("cat-file -p <blob_sha>    Print blob content.");
    println!("hash-object -w <file>     Computes the SHA hash of <file> and writes it to blob.")
}