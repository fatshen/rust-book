pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    //The test attribute
    // pub fn add_two(a: i32) -> i32 {
    //     a + 2
    // }

    // #[test]
    // fn it_works() {
    //     assert_eq!(4, add_two(2));
    // }

    // ignore
    // #[test]
    // #[should_panic(expected = "assertion failed")]
    // fn it_works() {
    //     assert!(false);
    // }

    // #[test]
    // #[ignore]
    // fn expensive_test() {
    //     assert!(false);
    // }

    // use super::add_two;
    use super::*;

    #[test]
    fn it_works(){
        assert_eq!(4, add_two(2));
    }
}
