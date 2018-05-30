fn main() {

    // String
    {
        let greeting = "Hello there!";
        println!("{}", greeting);

        let s1 = "foo\
                  bar";
        println!("{}", s1);


        let s2 = "foo
              bar";
        println!("{}", s2);

        let mut s3 = "Hello".to_string();
        println!("{}", s3);
        s3.push_str(", world!");
        println!("{}", s3);

        fn takes_slice(slice: &str) {
            println!("Got {}", slice);
        }

        let s4 = "Hello".to_string();
        takes_slice(&s4);
    }

    // Indexing
    {
        let hachiko = "忠犬ハチ公";

        for b in hachiko.as_bytes() {
            print!("{} ", b);
        }
        println!("");

        for c in hachiko.chars() {
            print!("{} ", c);
        }
        println!("");
    }

    // Slicing
    {
        let dog = "hachiko";
        let hachi = &dog[0..5];
        println!("{}", hachi);
    }

    // Concatenation
    {
        let s1 = "Hello ".to_string();
        let s2 = "world!";
        let s3 = s1 + s2;
        println!("{}", s3);

        let a1 = "Hello ".to_string();
        let a2 = "world!".to_string();
        let a3 = a1 + &a2;
        println!("{}", a3);
    }
}
