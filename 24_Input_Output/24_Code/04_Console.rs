 use std::io::Write;

fn main(){

        std::io::stdout().write("Tutorials ".as_bytes()).unwrap();
        std::io::stdout().write(String::from("Point").as_bytes()).unwrap();
}