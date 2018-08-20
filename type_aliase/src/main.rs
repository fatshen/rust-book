fn main() {
    type Name = String;
    let x: Name = "Hello".to_string();
    println!("x: {}", x);

    type Num = i32;
    let x_num: i32 = 5;
    let y_num: Num = 5;
    if x_num == y_num {
        println!("x_num == y_num");
    }
}
