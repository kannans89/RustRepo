
fn main(){

    let a = (110,true,10.8);
    println!("{:?}",a);

    let b:(i32,bool,f64) = (110,true,10.9);
    println!("{:?}",b);

    println!("{:?}",b.1);
    print(b);
}

fn print(x:(i32,bool,f64)){
    println!("Inside print method");
    println!("{:?}",x);
}