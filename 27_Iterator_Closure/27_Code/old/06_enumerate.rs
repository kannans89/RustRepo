fn main() {
    let mut iterator = vec!["A","B","C"].into_iter();
    let mut enumerated = iterator.enumerate();

    println!("{:?}",enumerated.next());
    println!("{:?}",enumerated.next());
    println!("{:?}",enumerated.next());
}