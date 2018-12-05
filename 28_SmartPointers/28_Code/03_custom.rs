
struct ProBox<T>(T);

impl<T> ProBox<T> {
    fn new(x:T)->ProBox<T>{
        ProBox(x)
    }
}

fn main() {
    let x = 5;
    let y = ProBox::new(x);

    println!("{}",5==x);
    println!("{}",5==*y);
}