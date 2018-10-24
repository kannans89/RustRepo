fn main(){
     
     let  name:String = String::from("TutorialsPoint");
     display(name);
    // println!("The value of name is:{}",name); //Error
}

fn display(param_name:String){
    
    println!("param_name value is :{}",param_name);
}