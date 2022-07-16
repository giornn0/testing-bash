use std::ops::Drop;
use std::mem::drop;
struct CustomSmartPointer{
    data: String
}
impl Drop for CustomSmartPointer{
    fn drop(&mut self){
        println!("Here the pointer with {} will be dropped after her lifetime expires.", self.data);
    }
}

fn main(){
    let longer =CustomSmartPointer{data:"Longer lifetime ".to_owned()};
    {
        let shorter =CustomSmartPointer{data:"Shorter lifetime".to_owned()};
    }
    //to force the drop (in one use case maybe), 
    //we can use std::mem::drop function
    //this is because we cant call drop manually
    //or disable the automatic call of drop after the scops end

    // drop(longer);
    
    for i in 0..20{
        println!("testing {}",i);
        //As you wil see the longer pointer is dropped
        //when his scope ends, so you can use him
        //in all this loop, but not the shorter one
    }
}