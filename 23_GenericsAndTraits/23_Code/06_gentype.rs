fn main(){
    
    let mut vector_integer: Vec<i32> = vec![20,30];
    let mut vector_string_slice:Vec<&'static str> = vec!["Tutorials","Point"];

   vector_string_slice.push("Mohtahsim");
   vector_integer.push(40);

   println!("{:?}",vector_string_slice);
   println!("{:?}",vector_integer);
    
}