fn cap_values_owned(max:i32,mut v:Vec<i32>)->Vec<i32>{
    for index in 0..v.len(){
        if v[index]>max{
            v[index]=max;
        }
    }
    return v;
}

fn main(){
    let mut values = vec![1,2,3,4,5,200];
    values = cap_values_owned(10,values);

    for v in values{
        println!("{}",v);
    }
}