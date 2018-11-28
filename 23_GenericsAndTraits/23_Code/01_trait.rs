
trait AnimalTrait{
    fn get_name(&self)->&'static str;
    fn walk(&self){
        println!("{} is walking",self.get_name());
    }
}

struct Human{
    name:&'static str
}

impl AnimalTrait for Human{
    fn get_name(&self)->&'static str {
          self.name
     }
}

fn main(){
    let h1 = Human{name:"Mohtashim"};
    println!("Human name is :{}",h1.get_name());
}