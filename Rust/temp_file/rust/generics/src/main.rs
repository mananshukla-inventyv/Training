struct add<T,U>{
    num1:T,
    num2:U
}


pub trait summary {
    fn summmarize(&a:add<T,U>)
}
fn main() {
    let a=add{
        num1:10.0,
        num2:20
    };
}
