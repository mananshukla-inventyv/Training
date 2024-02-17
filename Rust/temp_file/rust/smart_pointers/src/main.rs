use std::{ borrow::BorrowMut, fmt::Debug, ops::Deref, vec};

#[derive(Debug)]
enum List {
    Cons(i32,Box<List>),
    None
}
struct MyBox<T:Debug,U>(
    T,
    U
);
impl<T:Debug,U:Debug+PartialEq> MyBox<T,U> {
    fn new(x:T,y:U) -> MyBox<T,U>{
        MyBox(x,y)
    }
    fn trial(b:MyBox<T,U>) {
        println!("{:?}",b.1);
    }
}
use List::{Cons,None};

impl <T:Debug,U> Deref for MyBox<T,U> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl <T:Debug,U> Drop for MyBox<T,U> {
    fn drop(&mut self) {
        println!("The lock is removed!! from {:?}", self.0);
    }
}

fn main() {
    let sll=Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Cons(4, Box::new(Cons(5,Box::new(None))))))))));
    let x=5;
    let z="A";
    let y=MyBox::new(x,z);

    // y.drop(); error cant explicitly use the destructor

    
    // MyBox::trial(y);
    assert_eq!(x,5);
    assert_eq!(x,*(y.deref()));
    let mut a=Box::new(5);
    let mut b:&mut Box<i32>=a.borrow_mut();

    let mut c:&mut Box<i32>=a.borrow_mut();
    

    
    // String::from("value");
    let mut x: Box<vec::IntoIter<i32>>=Box::new(vec![1,2,3,4,5].into_iter());
    
    println!("{:?}",x.next());
    println!("{:?}",x.next());
    println!("{:?}",x.next());

    drop(y);
     // std::mem::drop(x)
    println!("Val waas dropped before end of func");
}
