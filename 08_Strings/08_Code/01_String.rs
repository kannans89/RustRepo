fn main(){

    let mut start = 'a' as u8;
    let mut alphabets=String::new();

    while start <= ('z' as u8){
        alphabets.push(start as char);
        if(start as char!='z'){
            alphabets.push_str(",");
        }
        start+=1;

    }

    println!("{}",alphabets);
}