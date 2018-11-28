use std::ops::Fn;
use std::boxed::Box;

fn main(){
   let f= f2();
   let r = f(4);
}

// fn <T:Fn(i32)->i32> f1->T {
//     let f = |x| {x*x};
//     return f;
// }

fn f2()->Box<dyn Fn(i32)->i32> {
    let f = |x| {x*x*x};
    return f;
}