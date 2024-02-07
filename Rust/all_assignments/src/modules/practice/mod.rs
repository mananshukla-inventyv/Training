pub fn practice() {
    let x = vec![1, 2, 3];
    let mut y = vec![1, 2, 3];
    y.extend(x.iter());
    println!("{:?}", x);
    // let  x="hello".to_string();
    // let  y="world".to_string();
    // let c=[x,y].concat();
    //    let z = x.append(&mut y);
    //    println!("{:?}",y);

    let mut list = vec![];
    let mut borrows_mutably = || list.push(2);
    borrows_mutably();
    println!("{:?}", list);
}
