fn main() {
    let a = [10,20,30];
    let mut iter = a.iter();
    println!("{:?}",iter);

    // a call to next returns next value

    println!("{:?}",iter.next());
    println!("{:?}",iter.next());
    println!("{:?}",iter.next());
    println!("{:?}",iter.next());
}