
fn main(){

    match f1(11){
        Ok(result)=>{
            match f2(result){
                Ok(result)=>{
                    println!("final result is {}",(result+1));
                }
                Err(msg)=>{
                    println!("{}",msg);
                }

            }

        }
        Err(err_msg)=>{
            println!("{}",err_msg);
        }
    }
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