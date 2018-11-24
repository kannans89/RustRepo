
fn main(){

    let mut a = 40;
    //let c = &mut a; // compilation error
    
    {
        let b = &mut a;
        *b += 2;
        

    }
      println!("a={}", a);

      let mut z = vec![10,20,30];

      for i in &z {
           print!("i={}\t",i);
           z.push(40);// compilation error
      }

}