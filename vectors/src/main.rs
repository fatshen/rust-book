fn main() {

    // base
    {
        let v1 = vec![1, 2, 3, 4];
        println!("v1[0]: {}", v1[0]);

        let v2 = vec![0; 10];
        println!("v2[0]: {}", v2[0]);
    }

    // usize
    {
        let v = vec![1, 2, 3, 4, 5];
        let i: usize = 0;
        // let j: i32 = 0;

        // work
        println!("v[i]: {}", v[i]);

        // not work
        // println!("v[j]: {}", v[j]);
    }

    // Out-of-bounds Access
    {
        let v = vec![1, 2, 3];
        // panic
        // println!("v[7]: {}", v[7]);

        match v.get(7) {
            Some(x) => println!("Item 7 is {}", x),
            None => println!("Item 7 not exist.")
        }
    }

    // iterator
    {
        let mut v = vec![1, 2, 3, 4, 5];

        for i in &v {
            println!("A reference to {}", i);
        }

        for i in &mut v {
            println!("A mutable reference to {}", i);
        }

        for i in v {
            println!("Take ownership of the vector and its element {}", i);
        }

        // work
        // for i in v {
        //     println!("Take ownership of the vector and its element {}", i);
        // }


    }
}
