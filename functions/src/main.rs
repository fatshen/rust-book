
fn main() {
    print_number(5);
    print_sum(1, 2);

    //error: expected one of `:` or `@` here
    //print_sum_error(3, 4);
    {
        let x: i32 = 12;
        let y: i32 = add_one(x);
        println!("{} add one : {}", x, y);
    }

    //error: help: consider removing this semicolon
    {
        let x: i32 = 12;
        let y: i32 = add_one(x);
        println!("{} add one : {}", x, y);
    }

    //Early returns
    {
        let x = early_return(11);
        println!("Early returns");
        println!("x : {}", x);
    }

    //Diverging functions
    {
        //let x = diverging();
        //println!("x : {}", x);
    }


    //function pointer
    {
        //let f: fn(i32) -> i32;
        let f = add_one;
        let six = f(5);
        println!("six: {}", six);
    }
}

fn print_number(x: i32) {
    println!("x is {}", x);
}

fn print_sum(x: i32, y: i32) {
    println!("sum is {}", x + y);
}

// fn print_sum_error(x, y) {
//     println!("sum is {}", x + y);
// }

fn add_one(x: i32) -> i32 {
    x + 1
}

//wrong way to return value
// fn add_one_error(x: i32) -> i32 {
//     x + 1;
// }

fn early_return(x: i32) -> i32 {
    return x;

    //warning: unreachable_code
    //x + 1
}

//Diverging functions
fn diverging() -> ! {
    panic!("Diverging functions");
}

