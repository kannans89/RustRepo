fn main()
{
   

    f1();

}

fn f2(){
         
    let mut iterator = (1..10).into_iter();
    
    let mut taken = iterator.take(2); 
    println!("{:?}",taken.next()); // Some(1)
    println!("{:?}",taken.next()); // Some(2)
    println!("{:?}",taken.next()); //None
   
}

fn f1(){
     let mut iterator = (1..10).into_iter();
    for arg in iterator.skip(5) {
     println!("{:?}",arg);
   }
}