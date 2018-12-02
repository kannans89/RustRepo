
 use std::io::Write;
fn main(){
    
    let mut file = std::fs::File::create("data.txt").unwrap();
    file.write_all("Hello World".as_bytes()).unwrap();
     file.write_all("\nTutorialsPoint".as_bytes()).unwrap();
}