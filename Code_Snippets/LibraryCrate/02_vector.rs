
//heap allocated dynamic array

//no need for this, its default as its very commonly used
//user std::collections::Vec;

fn main(){
    let mut vector = Vec::new(); // by default available

    vector.push('x');
    vector.push('y');

    for item in vector{
        println!("{}",item);
    }
}