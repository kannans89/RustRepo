fn main(){

    let v = vec![10,20,30];
    let itr= v.iter();
    for val in itr {
        println!("{}",val);
    }
}