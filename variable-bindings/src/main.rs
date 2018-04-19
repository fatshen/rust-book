fn main() {
    // 1. Patterns
    println!("1. Patterns");
    {
        let (x, y) = (1, 2);
        println!("{} {}", x, y);
    }

    // 2. Type annotations
    println!("2. Type annotations");
    {
        let z: i32 = 3;
        println!("{}", z);
    }

    // 3. Mutability
    println!("3. Mutability");
    {
        // error: cannot assign twice to immutable variable
        // let m = 4;
        // m = 5;
    }

    // 4. Initializing bindings
    println!("4. Initializing bindings");
    {
        // error: use of possibly uninitialized
        // let x4: i32;
        // println!("{}", x4);
    }

    // 5. Scope and shadowing
    println!("5. Scope and shadowing");
    {
        let x: i32 = 17;
        {
            let y: i32 = 3;
            println!("x:{}, y:{}", x, y);
        }
        // error: did you mean x
        // println!("x:{}, y:{}", x, y);
    }

    {
        let x: i32 = 8;
        {
            println!("x:{}", x);
            let x: i32 = 12;
            println!("x:{}", x);
        }
        println!("x:{}", x);
        let x = 42;
        println!("x:{}", x);
    }

    {
        let mut x: i32 = 1;
        println!("x:{}", x);
        x = 7;
        let x = x;
        // x = 1; x is immutable now
        println!("x:{}", x);

        let y = 4;
        println!("y:{}", y);
        let y = "I can also be bound to text!";
        println!("y:{}", y);
    }
}
