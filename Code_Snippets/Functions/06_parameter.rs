fn main(){
     
     let mut no:i32 = 5;
     mutate_no_to_zero(&mut no);
     println!("{}",no);
}

fn mutate_no_to_zero(param_no:&mut i32){
    *param_no =0; // de reference
}