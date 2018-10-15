// std is standard library crate
// collection is a module in std library crate
    use std::collections::LinkedList;
    
    //https://www.safaribooksonline.com/videos/learning-rust/9781788477918/9781788477918-video1_4

    fn main(){
        //create a mutable binding

        let mut list = LinkedList::new();
        //linked list is a strut
        list.push_back(10);
        list.push_back(20);
        list.push_back(30);

        for item in list {
            println!("{}",item);
        }


    
    }