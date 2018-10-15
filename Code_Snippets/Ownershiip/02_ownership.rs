
fn borrow_sum(v:&Vec<i32>)->i32{ //reference
    let mut sum=0;
    for value in v{
        sum+=*value; //de reference
    }
    return sum;
}

fn main(){
    let values= vec![1,2,3,4,5];
    let sum = borrow_sum(&values);
    println!("sume of {} values is {}",values.len(),sum);
}