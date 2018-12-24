// 1 байт - 8 бит
// 1 бит самая маленькая единица исчислении информации может быть или 0 или 1

struct Point {
    x: f32,
    y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Point {
        Point { x, y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    pub fn new(p1: Point, p2: Point) -> Rectangle {
        Rectangle { p1, p2 }
    }
    pub fn rect_area(&self) -> f32 {
        let Point { x: x_p1, y: y_p1 } = self.p1;
        let Point { x: x_p2, y: y_p2 } = self.p2;
        let mut area: f32 = 0.0;
        // some point from x or y is ueqal to other one
        //
        // y
        // ^
        // 5
        // |          * p1 {2.2x, 4.5y}
        // |
        // 4
        // |
        // |          * p1 {2.2x, 3.3y}
        // 3
        // |
        // |
        // 2
        // |
        // |
        // 1
        // |
        // |____1___2___3___4___5___6___7___8___9____> x
        if x_p1 == x_p2 {
            area = if y_p1 > y_p2 {
                // from highter point substract lower
                y_p1 - y_p2 * y_p1 - y_p2
            } else {
                y_p2 - y_p1 * y_p2 - y_p1
            };
        } else if y_p1 == y_p2 {
            area = if x_p1 > x_p2 {
                // from highter point substract lower
                x_p1 - x_p2 * x_p1 - x_p2
            } else {
                x_p2 - x_p1 * x_p2 - x_p1
            };
        } else {

        }
        area
        // 32.32
    }
}

fn main() {
    let p1 = Point::new(2.2, 3.3);
    let p2 = Point::new(2.2, 4.5);
    let rectangle = Rectangle::new(p1, p2);
}
