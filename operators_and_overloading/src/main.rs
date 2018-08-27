fn main() {
    use std::ops::Add;

    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl Add for Point {
        type Output = Point;

        fn add(self, other: Point) -> Point {
            Point { x: self.x + other.x, y: self.y + other.y}
        }
    }

    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 4 };

    let p3 = p1 + p2;

    println!("x: {}, y: {}", p3.x, p3.y);

    impl Add<i32> for Point {
        type Output = f64;

        fn add(self, other: i32) -> f64 {
            0.7
        }
    }

    println!("add<i32>: {}", p3 + 333);


    use std::ops::Mul;

    trait HasArea<T> {
        fn area(&self) -> T;
    }

    struct Squre<T> {
        x: T,
        y: T,
        side: T,
    }

    impl<T> HasArea<T> for Squre<T>
    where T: Mul<Output = T> + Copy {
        fn area(&self) -> T {
            self.side * self.side
        }
    }

    let s = Squre {
        x: 0.0f64,
        y: 0.0f64,
        side: 12.0f64,
    };

    println!("area: {}", s.area());

}
