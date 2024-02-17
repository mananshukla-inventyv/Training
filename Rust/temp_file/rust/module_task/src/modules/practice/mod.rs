use std::rc::Rc;
use self::List::{Cons,Nil};
pub enum List {
   Cons(i32, Rc<List>),
   Nil,
}

pub fn practice(){
   // let  x=vec![1,2,3];
   // let mut  y=vec![1,2,3];
   // y.extend(x.iter());
   // println!("{:?}", x);
   // let  x="hello".to_string();
   // let  y="world".to_string();
   // let c=[x,y].concat();
//    let z = x.append(&mut y);
//    println!("{:?}",y);

   // let mut list=vec![];
   // let mut borrows_mutably= || list.push(2) ;
   // borrows_mutably();
   // println!("{:?}",list);

   let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
   println!("count after creating a = {}", Rc::strong_count(&a));
   let b = Cons(3, Rc::clone(&a));
   println!("count after creating b = {}", Rc::strong_count(&a));
   {
      let c = Cons(4, Rc::clone(&a));
      println!("count after creating c = {}", Rc::strong_count(&a));
   }
   drop(b);
   println!("count after c goes out of scope = {}", Rc::strong_count(&a));
   println!("count after c goes out of scope = {}", Rc::weak_count(&a));
   // println!("{:?}",c);
}