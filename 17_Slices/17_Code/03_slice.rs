
fn main(){

      f4();

    
}

fn f1(){
    let s:&'static str ="Hello Rust"; //string slice
    println!("{}",s);

}

fn f2(){
    let s:&'static str ="Hello Rust"; //string slice
    println!("{}",s);

    for c in s.chars(){
        println!("{}",c);
    }
}

fn f3(){

    let s:&'static str ="Hello Rust"; //string slice

    for c in s.chars().rev(){
        println!("{}",c);
    }
     println!("{}",s);
}


fn f4(){

    let s:&'static str ="Hello Rust"; //string slice

    for c in s.chars().rev(){
        println!("{}",c);
    }

    let r =s.chars().nth(0);
    println!("{:?}",r);
      
    println!("{}",s);
}