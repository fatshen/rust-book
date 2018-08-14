fn main() {
    // ufcs
    {
        trait Foo {
            fn f(&self);
        }

        trait Bar {
            fn f(&self);
        }

        struct Baz;

        impl Foo for Baz {
            fn f(&self) { println!("Baz's impl of Foo"); }
        }

        impl Bar for Baz {
            fn f(&self) { println!("Baz's impl of Bar"); }
        }

        let b = Baz;
        Foo::f(&b);
        Bar::f(&b);
    }

    // Angle-bracket Form
    {
        trait Foo {
            fn foo() -> i32;
        }

        struct Bar;

        impl Bar {
            fn foo() -> i32 {
                20
            }
        }

        impl Foo for Bar {
            fn foo() -> i32 {
                10
            }
        }

        println!("<Bar as Foo>::foo(): {}", <Bar as Foo>::foo());
        println!("Bar::foo(): {}", Bar::foo());
    }
}
