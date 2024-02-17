// use std::cmp::min;
struct Shapes {
    width: f32,
    height: f32,
}

impl Shapes {
    fn rect(&self) -> f32 {
        self.width * self.height
    }
    fn circle(&self) -> f32 {
        // let radius=min(self.width ,self.height);
        if self.width <= self.height {
            3.14 * ((self.width * self.width) / 4.0)
        } else {
            3.14 * (self.height * self.height) / 4.0
        }
        // (self.width*self.height)/2.0
    }
    fn square(&self) -> f32 {
        if self.width <= self.height {
            self.width * self.width
        } else {
            self.height * self.height
        }
    }
}
fn main() {
    let s = Shapes {
        width: 10.0,
        height: 10.0,
    };
    println!("Area of Rectangle is: {:?}", s.rect());
    println!("Area of Square is: {:?}", s.square());
    println!("Area of Circle is: {:?}", s.circle());
}
