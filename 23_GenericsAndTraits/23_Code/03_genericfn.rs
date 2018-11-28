fn print_u8(val:u8){
    println!("u8 value is :{}",val);
}

fn print_u16(val:u16){
    println!("u16 value is:{}",val);
}

fn main(){
    print_u8(10 as u8);
    print_u16(20 as u16);
}