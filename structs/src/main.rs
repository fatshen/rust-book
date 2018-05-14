
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    // struct 1
    {
        let mut p = Point { x: 0, y: 0 };
        p.x = 60;
        println!("The point x: {}, y: {}", p.x, p.y);
    }

    // struct mutability
    {
        let mut p = Point { x: 0, y: 0 };
        p.x = 60;
        println!("The point x: {}, y: {}", p.x, p.y);

        let p = p;
        //p.y = 90;
        println!("The point x: {}, y: {}", p.x, p.y);
    }

    // struct references
    {
        struct Point {
            x: i32,
            y: i32,
        }

        struct PointRef<'a> {
            x: &'a mut i32,
            y: &'a mut i32,
        }

        let mut point = Point { x: 0, y: 0 };
        println!("x: {}, y: {}", point.x, point.y);
        {
            let r = PointRef { x: &mut point.x, y: &mut point.y };
            println!("x: {}, y: {}", r.x, r.y);
            *r.x = 10;
            *r.y = 20;
            println!("x: {}, y: {}", r.x, r.y);
        }
        assert_eq!(10, point.x);
        assert_eq!(20, point.y);
        println!("x: {}, y: {}", point.x, point.y);
    }

    // Update syntax
    {
        struct Point3d {
            x: i32,
            y: i32,
            z: i32,
        }

        let mut point = Point3d { x:0, y:0, z:0 };
        println!("x: {}, y: {}, z: {}", point.x, point.y, point.z);
        point = Point3d { y:10, .. point };
        println!("x: {}, y: {}, z: {}", point.x, point.y, point.z);
    }

    // struct tuple
    {
        struct Color(i32, i32, i32);
        struct Point(i32, i32, i32);

        let black = Color(1, 2, 3);
        let point = Point(10, 20, 30);

        println!("{}, {}, {}", black.0, black.1, black.2);
        println!("{}, {}, {}", point.0, point.1, point.2);

        let Point(_, origin_y, origin_z) = point;
        println!("origin_y: {}, origin_z: {}", origin_y, origin_z);

        struct Inches(i32);
        let Inches(length) = Inches(10);
        println!("length: {}", length);
    }

    // Unit-like structs
    {
        struct Electron{};
        struct Proton;

        let _x = Electron{};
        let _y = Proton;
        // error
        // let z = Electron;
    }
}


