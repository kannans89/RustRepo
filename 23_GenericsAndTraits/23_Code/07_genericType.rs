
struct Tagged<T> {
    value:T,
    tag_name:String
}

impl<T> Tagged<T> {
    fn tag(&self)->String {
        self.tag_name.clone()
    }
}

fn main(){
    let t:Tagged<i32> = Tagged{value:350,tag_name:"Kannan".to_string()};
    println!("value is :{} name is:{}",t.value,t.tag());
    
    //
    let t2:Tagged<String> = Tagged{value:"Sudhakaran".to_string(),tag_name:"Kannan".to_string()};
    println!("value is :{} name is:{}",t2.value,t2.tag());
    
}