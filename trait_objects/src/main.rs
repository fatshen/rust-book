fn main() {
    // background
    {
        trait Foo {
            fn method(&self);
        }

        impl Foo for u8 {
            fn method(&self) { println!("u8: {}", *self) }
        }

        impl Foo for String {
            fn method(&self) { println!("string: {}", *self) }
        }

        let a: String = "abc".to_string();
        a.method();
    }

    // static
    {
        trait Foo {
            fn method(&self);
        }

        impl Foo for u8 {
            fn method(&self) { println!("u8: {}", *self) }
        }

        impl Foo for String {
            fn method(&self) { println!("string: {}", *self) }
        }

        fn do_something<T: Foo>(x: T) {
            x.method();
        }

        let a: String = "dfg".to_string();
        do_something(a);

    }

    // dynamic
    {
        trait Foo {
            fn method(&self);
        }

        impl Foo for u8 {
            fn method(&self) { println!("u8: {}", *self) }
        }

        impl Foo for String {
            fn method(&self) { println!("string: {}", *self) }
        }

        fn do_something(x: &Foo) {
            x.method();
        }

        let a: String = "dfg".to_string();
        do_something(&a);
        do_something(&a as &Foo);

    }

}
