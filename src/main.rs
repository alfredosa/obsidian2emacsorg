// main.rs
mod sync;
use std::process::exit;
use std::{env, path::PathBuf};
use sync::*;

fn usage() {
    println!("Usage:");
    println!("      obsidian2org <source> <destination>");
    println!("      copies the source dir (including dir) to the destination of choice");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Incorrect Usage. See Usage:");
        usage();
        exit(1);
    }

    let source = PathBuf::from(&args[1]);
    let destination = PathBuf::from(&args[2]);

    let params = SyncParams {
        source,
        destination,
    };

    start_sync(&params);
}
