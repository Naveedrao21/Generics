trait AreaCalculator {
    fn area(&self) -> f64;
}
struct Circle {
    radius: f64,
}
 struct Rectangle {
    width: f64,
    hieght: f64,
 }
 impl AreaCalculator for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
 }
 impl AreaCalculator for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.hieght
    }
 }
 fn main() {
    // Circle ka object banaya
    let c = Circle {
        radius: 5.0,
    };

    // Rectangle ka object banaya
    let r = Rectangle {
        width: 10.0,
        hieght: 5.0,
    };

    // Area calculate karke print kiya
    println!("Area of circle is: {}", c.area());
    println!("Area of rectangle is: {}", r.area());
}