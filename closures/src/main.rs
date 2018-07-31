fn main() {
    // Grama
    {
        let plus_one = |x: i32| x + 1;
        println!("{}", plus_one(1));
    }

    // environment
    {
        let mut num = 5;
        {
            let plus_num = |x: i32| x + num;
        }
        let y = &mut num;
    }

    // move
    {
        let mut num = 5;
        let plus_num = move |x: i32| num += x;
        let y = &mut num;
    }

    // Taking closures as arguments
    {
        fn call_with_one<F>(some_closure: F) -> i32
        where F: Fn(i32) -> i32
        {
            some_closure(2)
        }

        let answer = call_with_one(|x: i32| x + 2);
        println!("{}", answer);
    }

    // Closures and pointers
    {
        fn call_with_one(some_closure: &Fn(i32) -> i32) -> i32
        {
            some_closure(1)
        }

        fn add_one(i: i32) -> i32
        {
            i + 1
        }

        let answer = call_with_one(&add_one);
        println!("answer: {}", answer);
    }

    // Return closure
    {
        fn factory() -> Box<Fn(i32) -> i32>
        {
            let num = 5;

            Box::new(move |x| x + num)
        }

        let f = factory();
        let answer = f(1);
        println!("answer: {}", answer);
    }
}
