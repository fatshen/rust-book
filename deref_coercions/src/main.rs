fn main() {
    use std::ops::Deref;

    struct DerefExample<T> {
        value: T,
    }

    impl<T> Deref for DerefExample<T> {
        type Target = T;

        fn deref(&self) -> &T {
            &self.value
        }
    }

    let x = DerefExample { value: 'a' };
    println!("*x: {}", *x);


    use std::rc::Rc;

    fn foo(s: &str) {
        println!("s: {}", s);
    }

    let owned = "Hello".to_string();
    let counted = Rc::new(owned);
    foo(&counted);

    struct Bar;

    impl Bar {
        fn bar(&self) {
            println!("bar");
        }
    }

    let b = &&Bar;
    b.bar();

}
