fn main() {

    // loop
    // loop {
    //     println!("loop!");
    // }

    // while
    println!("while: ");
    {
        let mut x = 5;
        let mut done = false;

        while !done {
            println!("x: {}", x);

            x = x - 1;
            if x == 0 {
                done = true;
            }

        }
    }

    // for
    println!("for: ");
    {
        for x in 0..10 {
            println!("x: {}", x)
        }
    }

    // Enumerate
    println!("enumerate:");
    {
        for (index, value) in (5..10).enumerate() {
            println!("index: {}, value: {}", index, value);
        }
    }

    // On iterators
    println!("On iterators");
    {
        let lines = "hello\nworld".lines();

        for (linenumber, line) in lines.enumerate() {
            println!("{}: {}", linenumber, line);
        }
    }

    // Ending iteration early
    println!("Ending iteration early");
    {
        let mut x = 5;

        loop {
            x += x - 3;

            println!("x: {}", x);
            if x % 5 == 0 {
                break
            }
        }

        for x in 0..10 {
            if x % 2 == 0 {
                continue
            }

            println!("x: {}", x);
        }
    }

    // Loop labels
    {
        'outer: for x in 0..10 {
            'inner: for y in 0..10 {
                if x % 2 == 0 { continue 'outer; }
                if y % 2 == 0 { continue 'inner; }

                println!("x: {}, y: {}", x, y);
            }
        }
    }

}
