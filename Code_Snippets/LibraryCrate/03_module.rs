
mod tutorials_point{
       pub fn sayHello(name:&str){
           println!("Hello says {}",name);
       }
}

fn main(){
    tutorials_point::sayHello("kannan");
}