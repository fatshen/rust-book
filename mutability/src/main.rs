fn main() {
    // mutability 1
    {
        //error
        // let x = 5;
        // x = 6;
        // println!("x: {}", x);

        let mut x = 5;
        println!("x: {}", x);
        x = 6;
        println!("x: {}", x);

        let y = &mut x;
        println!("y: {}", y);
        *y = 7;
        println!("y: {}", y);
    }

    // mutability 2
    {
        let mut x = 5;
        println!("x: {}", x);

        let mut z = 8;
        println!("z: {}", z);

        let mut y = &mut x;
        println!("y: {}", y);
        *y = 7;
        println!("y: {}", y);

        y = &mut z;
        println!("y: {}", y);
    }

    // Field-level mutability 1
    {
        struct Point {
            x: i32,
            y: i32,
        }

        let mut a = Point { x: 5, y: 6 };
        println!("a.x: {}, a.y: {}", a.x, a.y);
        a.x = 10;
        a.y = 20;
        println!("a.x: {}, a.y: {}", a.x, a.y);
    }

    // Field-level mutability 2
    {
        use std::cell::Cell;

        struct Point {
            x: i32,
            y: Cell<i32>,
        }

        let p = Point { x: 1, y: Cell::new(2) };
        p.y.set(7);
        println!("p.x: {}, p.y: {:?}", p.x, p.y);


    }





}
