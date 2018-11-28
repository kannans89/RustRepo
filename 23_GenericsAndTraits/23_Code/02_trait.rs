
fn main(){

    let b1 = Book {
        id:1001,
        name:"Rust in Action"
    };

    b1.print();
    
}

struct Book {
  name:&'static str,
  id:u32
}

trait Print {
    fn print(&self);
}

impl Print for Book {
    fn print(&self){
        println!("Printing book with id:{} and name {}",self.id,self.name)
    }
}