fn main() {

    // simple 1
    {
        let x = 1;
        let y = "c";
        match x {
            y => { println!("x: {}, y: {}", x, y); }
        }
        println!("y: {}", y);
    }

    // Multiple patterns
    {
        let x = 1;
        match x {
            1 | 2 => { println!("1 or 2"); },
            _ => { println!("something else"); },
        }
    }

    // Destructuring 1
    {
        struct Point {
            x: i32,
            y: i32,
        }

        let point = Point { x: 10, y: 20 };
        match point {
            Point { x, y } => { println!("x: {}, y: {}", x, y); },
        }
    }

    // Destructuring 2
    {
        struct Point {
            x: i32,
            y: i32,
        }

        let point = Point { x: 10, y: 20 };
        match point {
            Point { x: x1, y: y1 } => { println!("x1: {}, y1: {}", x1, y1); },
        }
    }

    // Destructuring 3
    {
        struct Point {
            x: i32,
            y: i32,
        }

        let point = Point { x: 10, y: 20 };
        match point {
            Point { x, .. } => { println!("x: {}", x); },
        }
    }

    // Destructuring 4
    {
        struct Point {
            x: i32,
            y: i32,
        }

        let point = Point { x: 10, y: 20 };
        match point {
            Point { y, .. } => { println!("y: {}", y); },
        }
    }

    // Ignoring bindings
    {
        let some_value : Result<i32, &'static str> = Err("There was an error.");
        match some_value {
            Ok(value) => println!("got an value: {}", value),
            Err(_) => println!("an error occured"),
        }

        fn coorinate() -> (i32, i32, i32) {
            (10, 20, 30)
        }

        let (_, a, _) = coorinate();
        println!("a: {}", a);

        {
            let tuple: (u32, String) = (5, String::from("five"));
            let (x, s) = tuple;
            //println!("Tuple is: {:?}", tuple);
        }

        {
            let tuple: (u32, String) = (5, String::from("five"));
            let (x, _) = tuple;
            println!("Tuple is: {:?}", tuple);
        }

        enum OptionalTuple {
            Value(i32, i32, i32),
            Missing,
        }

        let x = OptionalTuple::Value(1, 2, 3);

        match x {
            OptionalTuple::Value(..) => { println!("Got value"); },
            OptionalTuple::Missing => { println!("Missing"); },
        }

    }

    // ref
    {
        let x = 5;

        match x {
            ref r => { println!("r: {}", r); },
        }

        let mut y = 7;
        match y {
            ref mut mr => { println!("mr: {}", mr); },
        }
    }

    // ranges
    {
        let x = 1;
        match x {
            1 ... 5 => { println!("1 ... 5"); },
            _ => { println!("anything"); },
        }

        let y = '💅';
        match y {
            'a' ... 'j' => println!("a .. j"),
            'k' ... 'z' => println!("k .. z"),
            _ => println!("others"),

        }

        let z = 1;
        match z {
            e @ 1 ... 5 => { println!("e: {}", e); },
            _ => { println!("anything"); },
        }
    }

    // sample
    {
        #[derive(Debug)]
        struct Person {
            name: Option<String>,
        }

        let name = "Steve".to_string();
        let x: Option<Person> = Some(Person { name: Some(name) });
        match x {
            Some(Person { name: ref a @ Some(_), .. }) => println!("{:?}", a),
            _ => println!("anything"),
        }
    }

    // Guards 1
    {
        enum OptionalInt {
            Value(i32),
            Messing,
        }

        let x = OptionalInt::Value(5);
        match x {
            OptionalInt::Value(i) if i > 5 => println!("i > 5"),
            OptionalInt::Value(..) => println!("Got int"),
            OptionalInt::Messing => println!("Messing"),

        }
    }


    // Guards 2
    {
        let x = 4;
        let y = false;
        match x {
            4 | 5 if y => println!("4 | 5 and True"),
            _ => println!("no"),
        }
    }

}
