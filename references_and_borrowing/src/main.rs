fn main() {

    // references 1
    {
        fn foo(_v1: &Vec<i32>, _v2: &Vec<i32>) -> i32 {
            // return value
            42
        }

        let v1 = vec![1, 2, 3];
        let v2 = vec![1, 2, 3];
        let answer = foo(&v1, &v2);
        println!("answer: {}", answer);
    }

    // references 2
    {
        fn sum_vec(v: &Vec<i32>) -> i32 {
            return v.iter().fold(0, |a, &b| a + b);
        }

        fn foo(_v1: &Vec<i32>, _v2: &Vec<i32>) -> i32 {
            let s1 = sum_vec(&_v1);
            let s2 = sum_vec(&_v2);
            s1 + s2
        }

        let v1 = vec![1, 2, 3];
        let v2 = vec![4, 5, 6];

        let answer = foo(&v1, &v2);
        println!("answer: {}", answer);
    }

    // &mut references
    {
        let mut x = 5;
        {
            let y = &mut x;
            *y += 1;
        }
        println!("x: {}", x);
    }

    // Thinking in scopes
    {
        // cannot borrow `x` as immutable because it is also borrowed as mutable
        // let mut x = 5;
        // let y = &mut x;
        // *y += 1;
        // println!("x: {}", x);
    }

    // Iterator invalidation
    {
        let mut v = vec![1, 2, 3];
        for i in &v {
            println!("i: {}", i);
            // cannot borrow `v` as mutable because it is also borrowed as immutable
            // v.push(34);
        }
    }

    // release 1
    {
        // let y: &i32;
        // {
        //     let x = 5;
        //     y = &x;
        // }
        // println!("y: {}", y);
    }

    // release 2
    {
        let y: &i32;
        let x = 5;
        y = &x;
        println!("y: {}", y);

    }


}
