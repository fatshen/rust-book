fn main() {
    // HasArea
    {
        struct Circle {
            x: f64,
            y: f64,
            radius: f64,
        }

        trait HasArea {
            fn area(&self) -> f64;
            fn is_larger(&self, &Self) -> bool;
        }

        impl HasArea for Circle {
            fn area(&self) -> f64 {
                std::f64::consts::PI * (self.radius * self.radius)
            }

            fn is_larger(&self, other: &Self) -> bool {
                self.area() > other.area()
            }
        }

        let c = Circle { x: 1.0, y: 2.0, radius: 3.0 };
        let e = Circle { x: 2.0, y: 3.0, radius: 1.0 };
        println!("c.area: {}", c.area());
        println!("c.is_larger(): {}", c.is_larger(&e));
    }

    // Trait bounds on generic functions
    {
        struct Circle {
            x: f64,
            y: f64,
            radius: f64,
        }

        trait HasArea {
            fn Area(&self) -> f64;
        }

        impl HasArea for Circle {
            fn Area(&self) -> f64 {
                std::f64::consts::PI * (self.radius * self.radius)
            }
        }

        struct Square {
            x: f64,
            y: f64,
            side: f64,
        }

        impl HasArea for Square {
            fn Area(&self) -> f64 {
                self.side * self.side
            }
        }

        fn print_area<T: HasArea>(shape: T) {
            println!("shape.Area: {}", shape.Area());
        }

        let c = Circle { x: 1.0, y: 2.0, radius: 3.0 };
        let s = Square { x: 1.0, y: 2.0, side: 3.0 };
        print_area(c);
        print_area(s);
    }

    // Trait bounds on generic structs
    {
        struct Rectangle<T> {
            x: T,
            y: T,
            width: T,
            height: T,
        }

        impl<T: PartialEq> Rectangle<T> {
            fn is_square(&self) -> bool {
                self.width == self.height
            }
        }

        let mut r = Rectangle {
            x: 0,
            y: 0,
            width: 42,
            height: 42,
        };

        println!("r.is_square(): {}", r.is_square());

        r.width = 1;
        println!("r.is_square(): {}", r.is_square());
    }

    // Rules for implementing traits
    {
        trait ApproxEqual {
            fn approx_equal(&self, other: &Self) -> bool;
        }

        impl ApproxEqual for f32 {
            fn approx_equal(&self, other: &Self) -> bool {
                (self - other).abs() <= std::f32::EPSILON
            }
        }

        println!("1.0 approx 1.1.00000001: {}", 1.0.approx_equal(&1.00000001));
    }

    // Multiple trait bounds
    {
        use std::fmt::Debug;

        fn foo<T: Clone + Debug>(x: T) {
            x.clone();
            println!("{:?}", x);
        }
    }

    // Where clause
    {
        use std::fmt::Debug;

        fn foo<T: Clone, K: Clone + Debug>(x: T, y: K) {
            x.clone();
            y.clone();
            println!("{:?}", y);
        }

        fn bar<T, K>(x: T, y: K) where T: Clone, K: Clone + Debug {
            x.clone();
            y.clone();
            println!("{:?}", y);
        }

        foo("hello", "world");
        bar("hello", "world");

        trait ConvertTo<Output> {
            fn convert(&self) -> Output;
        }

        impl ConvertTo<i64> for i32 {
            fn convert(&self) -> i64 {
                *self as i64
            }
        }

        fn normal<T: ConvertTo<i64>>(x: &T) -> i64 {
            x.convert()
        }

        fn inverse<T>(x: i32) -> T
        where i32: ConvertTo<T> {
            x.convert()
        }

    }

    // Default methods
    {
        trait Foo {
            fn is_valid(&self) -> bool;
            fn is_invalid(&self) -> bool { !self.is_valid() }
        }

        struct UserDefault;

        impl Foo for UserDefault {
            fn is_valid(&self) -> bool {
                println!("Call UserDefault.is_valid.");
                true
            }
        }

        struct OverrideDefault;

        impl Foo for OverrideDefault {
            fn is_valid(&self) -> bool {
                println!("Call OverrideDefault.is_valid.");
                true
            }

            fn is_invalid(&self) -> bool {
                println!("Call OverrideDefault.is_invalid.");
                true
            }
        }

        let default = UserDefault;
        default.is_invalid();

        let over = OverrideDefault;
        over.is_invalid();
    }

    // Inheritance
    {
        trait Foo {
            fn foo(&self);
        }

        trait FooBar : Foo {
            fn foobar(&self);
        }

        struct Baz;

        impl Foo for Baz {
            fn foo(&self) { println!("foo") }
        }

        impl FooBar for Baz {
            fn foobar(&self) { println!("foobar"); }
        }

        let b = Baz;
        b.foo();
        b.foobar();
    }


    // Deriving
    {
        #[derive(Debug)]
        struct Foo;

        println!("{:?}", Foo);
    }





}
