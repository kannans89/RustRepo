
fn main(){

    let mut iterator = (1..5).into_iter();
    println!("{:?}",iterator.next()); // Some(1)
    println!("{:?}",iterator.next()); // Some(2)
    println!("{:?}",iterator.next()); // Some(3)
    println!("{:?}",iterator.next()); // Some(4)
    println!("{:?}",iterator.next()); // None
    println!("{:?}",iterator.next()); // None
}