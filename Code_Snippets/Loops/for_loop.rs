fn main(){
     for x in 1..11{ // 11 is not inclusive
        if x==5 {
            continue;
        }
         println!("x is  {}",x);
     }

         //   println!("x outside is  {}",x);
     //iterate wil position and vlaue

     for (index,value)  in (10..21).enumerate() {
         println!("index is {} and value is {}",index,value);
     }
}