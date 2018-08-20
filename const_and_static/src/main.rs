fn main() {
    // const
    {
        const N: i32 = 5;
        println!("N: {}", N);
    }

    // static
    {
        static N: i32 = 5;
        println!("N: {}", N);
    }

    // mutable
    {
        static mut N: i32 = 5;
        unsafe {
            N += 1;
            println!("N: {}", N);
        }
    }
}
