
fn main(){
 f4();
}

fn f4(){
     //String + String

  let name1 = "Tutorials".to_string();
  let name2 =   "Point".to_string();
  let name3 = name1 + &name2; // dereference conversion
  println!("{}",name3);

}

fn f3(){
     //String + String

  let name1 = String::from("Tutorials");
  let name2 =   String::from("Point");
  let name3 = name1 + &name2; // dereference conversion
  println!("{}",name3);

}
fn f2(){
  let name1 = String::from("Tutorials");
  // String + &str
  let name2 = name1 + "Point";
  println!("{}",name2);
}

fn f1() {
     //Conversion between String and &str
    let name1 = String::from("Tutorials");
    // &str = &String
    let name2:&str = &name1; // deref conversion
    println!("{}",name1);
    println!("{}",name2);
}