fn main() {
    #[cfg(target_os = "linux")]
    fn foo() {
        println!("Linux");
    }

    foo();
}
