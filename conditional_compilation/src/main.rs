#[cfg(any(unix, windows))]
fn foo() -> i32 {
    2
}

fn main() {
    println!("Hello, world! {}", foo());
}
