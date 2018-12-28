
fn main(){
   match f3() {
       Ok(r)=> {
           println!("result in main is {}",r);
       }
       Err(e) =>{
           println!("error in main {}",e);
       }
   }
}
fn f3()->Result<i32,String>{

   f2(f1(11)?) //f3propagate error to main
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