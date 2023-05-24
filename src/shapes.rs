use std::f64::consts::PI;

pub trait Shape {
    fn area(&self);
    fn perimeter(&self);
    fn draw(&self);
}

pub struct Circle {
    pub r: f64,
}

impl Shape for Circle {
    fn area(&self) {
        println!("Area do cirulo é {}", (self.r * self.r) * PI)
    }
    fn perimeter(&self) {
        println!("Perimetro do cirulo é {}", 2.0 * PI * self.r)
    }

    fn draw(&self) {
        todo!()
    }
}
