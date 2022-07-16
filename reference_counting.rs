//Used when we want to have multiple shares of a pointer

use std::rc::Rc;
use crate::List::{Cons,Nil};

enum List{
    Cons(i32,Rc<List>),
    Nil
}


fn main() {
    let a = Rc::new(Cons(3,Rc::new(Cons(4,Rc::new(Cons(5,Rc::new(Nil)))))));

    println!("")

    let b = Cons(1,Rc::clone(&a));
    let c = Cons(2,Rc::clone(&a));
    
}
