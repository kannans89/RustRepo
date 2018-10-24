fn main() {
    rule3();
 
    
}

fn rule3(){
  //program cannot create mutable references if
  //it already has more than one or immutable reference and vice versa
  
  //given scope only one is possible either mut or //immut
  
  let mut name = String::from("kannan");
  {
  let r2 = &name;
  }
  let r1 = &mut name;
  
  
}

fn rule2(){
    //only one mutable reference is valid in a scope
    
      let mut name = String::from("kannan");
      
      let ref1 = &mut name; // ref1 points to name
     // let ref2 = &mut name; //ERR:ref2 points to name
}

fn rule1(){
    let name = String::from("kannan");
    
    //multiple immutable references are valid
    let ref1 = &name; // ref1 points  to name
    let ref2 = &name; // ref2 points to name
    let ref3 = &name; //ref3 points to name
    
}