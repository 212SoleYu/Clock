// read.rs
// Date: 2025-02-17
use std::fs::File;

pub fn hello_test() {
    println!("hello world!");
}



pub fn get_current_path() {
    let path = std::env::current_dir().unwrap();
    println!("The current directory is {}", path.display());
}