
fn main(){

    let result1 = f1(10);
    if result1.is_err() {
        panic!("Error for result1");
    }
    //passing result1 to f2
    let result2 =  f2(result1.unwrap());//taking result
     if result2.is_err() {
         panic!("Error for result2");
     }

     println!("final result is {}",result2.unwrap());


}

fn f1(input:i32)->Result<i32,String>{
    println!("inside f1 {}",input);
      if input > 10 {
          Err(format!("Error:input > 10")) //no semicolon (this is returned)
      }
      else {
          Ok(input) //no semicolon (this is returned)
      }

}

fn f2(input:i32)->Result<i32,String>{
    println!("inside f2 {}",input);
     if input >20 {
         Err(format!("Error:input>20"))
     }
     else {
         Ok(input)
     }
}