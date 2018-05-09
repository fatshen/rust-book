fn main() {

    // Move semantics 1
    {
        let v = vec![1, 2, 3];
        let v2 = v;
        println!("v[0]: {}", v2[0]);
        //use of moved value
        //println!("v[0]: {}", v[0]);
    }

    // Move semantics 2
    {
        fn take(_v: Vec<i32>) {
            //do nothing
        }

        let v = vec![1, 2, 3];
        take(v);
        //println!("v[0]: {}", v[0]);
    }

    // copy
    {
        let a = 5;
        let _y = double(a);
        println!("_y: {}", _y);

        let b = true;
        let _z = change_truth(b);
        println!("_z: {}", _z);

        fn double(x: i32) -> i32 {
            x * 2
        }

        fn change_truth(x: bool) -> bool {
            !x
        }
    }

    // More than ownership
}
