fn main() {

    {
        let x = 5;

        if x == 5 {
            println!("x is five");
        }
    }

    {
        let x = 5;

        if x == 5 {
            println!("x is five");
        } else {
            println!("x is not five");
        }
    }

    {
        let x = 6;

        if x == 5 {
            println!("x is five");
        } else if x == 6 {
            println!("x is six");
        }
    }

    {
        let x = 5;
        let y = if x == 5 {
            50
        } else {
            60
        };
        println!("y: {}", y);
    }

    {
        let x = 6;
        let y = if x == 5 { 50 } else { 60 };
        println!("y: {}", y);
    }

}
