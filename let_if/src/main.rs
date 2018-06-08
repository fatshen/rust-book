fn main() {
    // if let
    {
        let option = Some(5);
        fn foo(x: i32) {
            println!("{}", x);
        }

        fn bar() {
            println!("others");
        }

        match option {
            Some(x) => { foo(x); },
            None => {},
        }

        if option.is_some() {
            let x = option.unwrap();
            foo(x);
        }

        if let Some(x) = option {
            foo(x);
        } else {
            bar();
        }
    }

    // while if 
    {
        let mut v1 = vec![1, 3, 5, 7, 9];
        loop {
            match v1.pop() {
                Some(x) => println!("{}", x),
                None => break,
            }
        }

        let mut v2 = vec![1, 3, 5, 7, 9];
        while let Some(x) = v2.pop() {
            println!("{}", x);
        }
    }
}
