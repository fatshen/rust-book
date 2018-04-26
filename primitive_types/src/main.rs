fn main() {
    // boolean
    {
        let x = true;
        let y: bool = false;
        println!("x: {}", x);
        println!("y: {}", y);
    }

    // char
    {
        let x = 'x';
        let two_char = "[]";
        println!("x: {}", x);
        println!("two_char: {}", two_char);
    }

    // integer
    {
        let x = 42;
        let y = 0.1;
        println!("x: {}", x);
        println!("y: {}", y);
    }

    // array
    {
        let a = [1, 2, 3];
        let mut m = [5, 6, 7];
        println!("a.len(): {}", a.len());
        println!("m.len(): {}", m.len());

        let list = ["aaa", "bbb", "cccc"];
        println!("list[1]: {}", list[1]);
    }

    // Slicing syntax
    {
        let a = [0, 1, 2, 3, 4];
        let complete = &a[..];
        let middle = &a[1..4];

        println!("compete[0]: {}", complete[0]);
        println!("middle[0]: {}", middle[0]);
    }

    // Tuples
    {
        //let x = (1, "hello");
        let x: (i32, &str) = (1, "hello");
        println!("x.0: {}", x.0);
        println!("x.1: {}", x.1);

        let mut m = (1, 2);
        let y = (4, 5);
        m = y;
        println!("m.0: {}", m.0);
        println!("m.1: {}", m.1);

        let (a, b, c) = ('a', 7, "aaaaa");
        println!("a: {}", a);
        println!("b: {}", b);
        println!("c: {}", c);
    }

    // Tuple Indexing
    {
        let tuple = (1, 2, 3);
        let x = tuple.0;
        let y = tuple.1;
        let z = tuple.2;

        println!("x: {}", x);
        println!("y: {}", y);
        println!("z: {}", z);
    }


}
