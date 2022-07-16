//
// -> When you have a type whose 
//      size can't be known at compile time
//      and you wnat to use a value of that 
//      type in a context that requires an exact size
//->When you have a large amount of data and 
//      you want to transfer ownership but
//       ensure the data won't be copied when you do so
//->When you want to own a value and you care only
//       that it's a type that implements a particular 
//      trait rather than being of a specific type
use crate::List::{Cons,Nil};
use std::rc::Rc;
use std::ops::Deref;

enum List{
    Cons(i32, Rc<List>),
    Nil
}

//I have changed the implementation of Box to use Rc, 
//Box in the end is nothing specific, points to a 
//specific data pointer in the heap.
//This allows you to "Box" recursive size at compile time

struct TestBox<T>(T);
impl<T> TestBox<T>{
    fn new(test: T)->TestBox<T>{
        TestBox(test)
    }
}
impl<T> Deref for TestBox<T>{
    type Target = T;
    fn deref(&self)->&Self::Target{
        &self.0
    }
}

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10,Rc::new(Nil))))); //befor it used a simple Box instead of Rc

    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));

    let z = Box::new(5);
    println!("{}",*z *5); // deref the pointer

    let test = TestBox::new(8);
    println!("Implementing borrow{}",5+ *test);
    
    let pass = &(*test);
    println!("Borrowed before and stil works{}",10+ *test);
}
