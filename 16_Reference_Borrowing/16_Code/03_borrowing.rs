
fn main(){
      let mut a = 100;
      {
      let b = &mut a;
      *b  +=2;
      }
      println!("a={}",a);
      //you cannot use a untill you unborrow a from its control

}
