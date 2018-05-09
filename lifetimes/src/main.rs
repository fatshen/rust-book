fn main() {

    // lifetimes 1
    // this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `line` or `prefix`
    {
        // fn skip_prefix(line: &str, prefix: &str) -> &str {
        //     line
        // }

        // let line = "lang:en=Hello world!";
        // let lang = "en";

        // let v;
        // {
        //     let p = format!("lang={}", lang);
        //     v = skip_prefix(line, p.as_str());
        // }
        // println!("v: {}", v);
    }

    // lifetimes 2
    {
        fn skip_prefix<'a, 'b>(_line: &'a str, _prefix: &'b str) -> &'a str {
            _line
        }

        let line = "lang:en=Hello world!";
        let lang = "en";

        let v;
        {
            let p = format!("lang={}", lang);
            v = skip_prefix(line, p.as_str());
        }
        println!("v: {}", v);
    }

    // struct
    {
        struct Foo<'a> {
            x: &'a i32,
        }

        let y = &5;
        let f = Foo{ x: y};
        println!("f.x: {}", f.x);
    }

    // impl
    {
        struct Foo<'a> {
            x: &'a i32,
        }

        impl<'a> Foo<'a> {
            fn x(&self) -> &'a i32 { self.x }
        }

        let y = &5;
        let f = Foo{ x: y};
        println!("f.x(): {}", f.x());

    }

    // struct 2
    // `f.x` does not live long enough
    {
        // struct Foo<'a> {
        //     x: &'a i32,
        // }

        // let x;
        // {
        //     let y = &5;
        //     let f = Foo{ x: y};
        //     x = &f.x;
        // }
        // println!("x: {}", x);
    }


}
