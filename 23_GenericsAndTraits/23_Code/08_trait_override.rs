fn main(){

    let b1 = Book {
        id:1001,
        name:"Rust in Action"
    };

    b1.print();
    b1.hello();
}

struct Book {
  name:&'static str,
  id:u32
}

trait Print {
    fn print(&self);
    fn hello(&self){
        println!("hello");
    }
}

impl Print for Book {
    fn print(&self){
        println!("Printing book with id:{} and name {}",self.id,self.name)
    }
    fn hello(&self){
        println!("hello overriden to howdy!!");
    }
}
