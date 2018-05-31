fn main() {

    // generics
    {
//        enum Option<T> {
//            Some(T),
//            None
//        }
//
//        let x: Option<i32> = Some(5);
//        println!("{:?}", x);
    }

    // Generic structs
    {
        struct Point<T> {
            x: T,
            y: T,
        }

        let mut int_point = Point { x: 1, y: 2 };
        let float_point = Point { x: 1.2, y:3.2 };

        println!("x: {}, y: {}", int_point.x, int_point.y);
        println!("x: {}, y: {}", float_point.x, float_point.y);

        impl<T> Point<T> {
            fn swap(&mut self) {
                std::mem::swap(&mut self.x, &mut self.y);
            }
        }

        int_point.swap();
        println!("x: {}, y: {}", int_point.x, int_point.y);
    }

    // Resolving ambiguities
    {
        let mut v1 = Vec::new();
        v1.push(true);
        println!("{:?}", v1);

        let v2 = Vec::<bool>::new();
        println!("{:?}", v2);

    }
}
