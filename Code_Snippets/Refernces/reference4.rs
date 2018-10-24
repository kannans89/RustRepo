fn main() {
    let x = 5;
    let y = & x; // y points to x
    
    println!("{}",y);
    println!("{}",*y);
    
    let b:i32=*y;
    //let a:i32=y; //Error a expects concrete data
}