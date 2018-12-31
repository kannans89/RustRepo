use std::fs;

fn main(){
    fs::remove_file("data.txt").expect("could not reomve file");
    println!("file is removed");
}