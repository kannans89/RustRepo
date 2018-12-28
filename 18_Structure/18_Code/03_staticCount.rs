


struct Point {
    x: i32,
    y: i32,
  
}

impl Point {

    fn getInstance(x: i32, y: i32) -> Point {
        Point { x: x, y: y }
        POINT_COUNT+=1;
    }

    fn display(&self){
        println!("x ={} y={}",self.x,self.y );
    }
}


fn main(){
    let p1 = Point::getInstance(10,20);
    p1.display();
    let p1 = Point::getInstance(10,20);
    p1.display();

    println!("{}",POINT_COUNT);

}