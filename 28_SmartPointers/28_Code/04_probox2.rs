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

fn main() {
    let x = 5;
    let y = ProBox::new(x);

    println!("5==x is {}",5==x);
    println!("5==*y is {}",5==*y);
     println!("x==*y is {}",x==*y);
}