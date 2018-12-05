use std::ops::Deref;

struct ProBox<T>(T);

impl<T> ProBox<T> {
    fn new(x:T)->ProBox<T>{
        ProBox(x)
    }
}

impl<T> Deref for ProBox<T> {
  type Target = T;

    fn deref(&self) -> &T {
      &self.0
    }
}


impl<T> Drop for ProBox<T>{
   fn drop(&mut self){
    
       println!("dropping Probox object from memory ");
   }    
}

fn main() {
    let x = 50;
    ProBox::new(x);
    ProBox::new("Hello");
    
}