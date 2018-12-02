
fn main(){
    let mut line = String::new();
    println!("Enter your name :");
    std::io::stdin().read_line(&mut line);
    println!("Hello Dear, {}", line);
}