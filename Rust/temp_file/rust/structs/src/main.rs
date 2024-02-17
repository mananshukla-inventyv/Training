
struct User{
    user_name:String,
    email:String,
    sign_in_count:u64,
    active:bool,
}
#[derive(Debug)]
struct Rectangle{
    width:u32,
    height:u32
}
impl Rectangle {
    fn area(&self)->u32 {
        self.height*self.width
    }
}

fn main() {
    let mut u1=User{
        user_name:String::from("Manan"),
        email:String::from("manan@gmail.com"),
        sign_in_count:12,
        active:true,
    };
    u1.user_name=String::from("Xyz");
    let u2=build_user(
        String::from("ABC"),
        String::from("ABC@gmail.com"));

    let u3=User{
        user_name:String::from("PQR"),
        ..u2
    };
    let rect=&Rectangle{
        width:30,
        height:50
    };
    println!("{:?}", rect);
    let area=area(rect);
    println!("Area is: {}",area);
    println!("Area is: {}",rect.area());
}
fn build_user(user_name:String, email:String)->User{
    User{
        user_name,
        email,
        sign_in_count:1,
        active:true
    }
}
fn area(rect:&Rectangle)->u32 {
    rect.width*rect.height
}