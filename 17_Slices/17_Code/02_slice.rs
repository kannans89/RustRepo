
fn main(){

    let mut data = [10,20,30,40,50];
    use_slice(&mut data[1..4]);//this is effectively borrowing elements for a while
    println!("{:?}",data);
}

fn use_slice(slice:&mut [i32]){ // is takign a slice or borrowing a part of an array of i32s

  println!("length of slice is {:?}",slice.len());
  println!("{:?}",slice);
  slice[0]=1010;

}