use std::mem;

fn main() {
    let x: i32 = 5;
    let y = x as i64;
    println!("y: {}", y);

    let one = true as u8;
    println!("one: {}", one);

    let at_sign = 64 as char;
    println!("at_sign: {}", at_sign);

    let two_hundred = -56i8 as u8;
    println!("two_hundred: {}", two_hundred);

    let a = 300 as *const char;
    let b = a as u32;
    println!("b: {}", b);

    unsafe{
        let a = [0u8, 1u8, 0u8, 0u8];
        let b = mem::transmute::<[u8; 4], u32>(a);
        println!("b: {}", b);

        let c: u32 = mem::transmute(a);
        println!("c: {}", c);
    }
}
