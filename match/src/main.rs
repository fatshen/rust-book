fn main() {
    // simple
    {
        let x = 5;
        match x {
            1 => { println!("one"); },
            2 => { println!("two"); },
            3 => { println!("three"); },
            4 => { println!("four"); },
            5 => { println!("five"); },
            6 => { println!("six"); },
            7 => { println!("seven"); },
            8 => { println!("eight"); },
            _ => { println!("something else"); },
        }
    }

    // simple 2
    {
        let x = 5;
        let y = match x {
            1 => "one",
            2 => "two",
            3 => "three",
            4 => "four",
            5 => "five",
            6 => "six",
            7 => "seven",
            8 => "eight",
            _ => "something else",
        };

        println!("y: {}", y);
    }

}
