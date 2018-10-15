//multiple borrows at same time

fn main(){
    let mut values = [1,2,3,4,5];
    let a = &values;
    let b = &values;
    println!("{:?}",*a);//de reference a (follow where a is pointing)
    println!("{:?}",*b);// de reference b (follow where b is pointing)
    println!("{:?}",values);

    let c = &b;
     println!("Following c {:?}",*c);// follow to end 

     //values[0]=100;//Error:cannot mutate value which are borrowed
     


}