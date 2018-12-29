
//https://doc.rust-lang.org/rust-by-example/flow_control/for.html?highlight=into_iter#for-and-iterators

fn main(){
//  iter_1();

 //into_iter_2();

  iter_mut3();
}

fn iter_mut3() {
      let mut names = vec!["Kannan", "Mohtashim", "Kiran"];

    for name in names.iter_mut() {
        match name {
            &mut "Mohtashim" => println!("There is a rustacean among us!"),
            _ =>  println!("Hello {}", name),
        }
    
    }

    println!("{:?}",names);
}



fn into_iter_2(){
    let names = vec!["Kannan", "Mohtashim", "Kiran"];

    for name in names.into_iter() {
        match name {
            "Mohtashim" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }

   // println!("{:?}",names); //Error:Cannot access after ownership move
}

fn iter_1() {
     let names = vec!["Kannan", "Mohtashim", "Kiran"];

    for name in names.iter() {
        match name {
            &"Mohtashim" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }

    println!("{:?}",names);
}
