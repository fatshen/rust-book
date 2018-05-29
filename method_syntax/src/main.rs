fn main() {
    // Circle
    {
        struct Circle {
            x: f64,
            y: f64,
            radius: f64,
        }

        impl Circle {
            fn area(&self) -> f64 {
                std::f64::consts::PI * (self.radius * self.radius)
            }

            fn grow(&self, increment: f64) -> Circle {
                Circle { x: self.x, y: self.y, radius: self.radius + increment }
            }

            fn reference(&self) {
                println!("taking self by reference!");
            }

            fn mutable_reference(&mut self) {
                println!("taking self by mutable reference!");
            }

            fn takes_ownership(self) {
                println!("taking ownership of self!");
            }
        }

        let mut c = Circle { x: 0.0, y: 0.0, radius: 2.0 };
        println!("c.area(): {}", c.area());

        c.reference();
        c.mutable_reference();
        c.takes_ownership();

        let a = Circle { x: 0.0, y: 0.0, radius: 2.0 };
        println!("a: {}", a.area());

        let b = a.grow(2.0).area();
        println!("b: {}", b);
    }

    // Associated functions
    {
        struct Circle {
            x: f64,
            y: f64,
            radius: f64,
        }

        impl Circle {
            fn new(x: f64, y: f64, radius: f64) -> Circle {
                Circle {
                    x: x,
                    y: y,
                    radius: radius,
                }
            }
        }

        let c = Circle::new(1.0, 2.0, 2.0);
        println!("c.x: {}", c.x);
    }

    // Builder Pattern
    {
        struct Circle {
            x: f64,
            y: f64,
            radius: f64,
        }

        impl Circle {
            fn area(&self) -> f64 {
                std::f64::consts::PI * (self.radius * self.radius)
            }
        }

        struct CircleBuilder {
            x: f64,
            y: f64,
            radius: f64,
        }

        impl CircleBuilder {
            fn new() -> CircleBuilder {
                CircleBuilder { x: 0.0, y: 0.0, radius: 1.0, }
            }

            fn x(&mut self, coordinate: f64) -> &mut CircleBuilder {
                self.x = coordinate;
                self
            }

            fn y(&mut self, coordinate: f64) -> &mut CircleBuilder {
                self.y = coordinate;
                self
            }

            fn radius(&mut self, coordinate: f64) -> &mut CircleBuilder {
                self.radius = coordinate;
                self
            }

            fn finalize(&self) -> Circle {
                Circle { x: self.x, y: self.y, radius: self.radius }
            }
        }

        let c = CircleBuilder::new()
            .x(1.0)
            .y(2.0)
            .radius(2.0)
            .finalize();

        println!("c.area(): {}", c.area());
        println!("c.x: {}", c.x);
        println!("c.y: {}", c.y);
    }
}
