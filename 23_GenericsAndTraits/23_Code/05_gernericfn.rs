use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

struct Book{
    id:u32
}
impl Display for Book {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Book id {}", self.id)
    }
}

fn main(){
    print_pro(10);
    print_pro("Hello TutorialsPoint");
    print_pro(Book{id:1001});
}

fn print_pro<T:Display>(t:T){
    println!("Inside print_pro generic function:");
    println!("{}",t);
}
