macro_rules! foo {
    (x => $e:expr) => (println!("mode X: {}", $e));
    (y => $e:expr) => (println!("mode Y: {}", $e));
}

macro_rules! o_0 {
    (
        $(
            $x:expr; [ $( $y:expr ),* ]
        );*
    ) => {
        &[ $($( $x + $y ),*),* ]
    }
}

macro_rules! five_times {
    ($x:expr) => (5 * $x);
}

fn get_log_state() -> i32 {
    3
}

macro_rules! log {
    ($msg:expr) => {{
        let state = get_log_state();
        if state > 0 {
            println!("log({}): {}", state, $msg);
        }
    }};
}

macro_rules! write_html {
    ($w:expr, ) => (());

    ($w:expr, $e:tt) => (write!($w, "{}", $e));

    ($w:expr, $tag:ident [ $($inner:tt)* ] $($rest:tt)*) => {{
        write!($w, "<{}>", stringify!($tag));
        write_html!($w, $($inner)*);
        write!($w, "</{}>", stringify!($tag));
        write_html!($w, $($rest)*);
    }};
}


fn main() {

    // match
    {
        foo!(y => 3);

        //error: no rules expected the token `z`
        //foo!(z => 3);
    }


    // Repetition
    {
        let a: &[i32] = o_0!(10; [1, 2, 3]; 20; [4, 5, 6]);
        assert_eq!(a, [11, 12, 13, 24, 25, 26]);
    }

    // Hygiene
    {
        println!("5 * 7: {}", five_times!(7));

        let state: &str = "reticulating splines";
        log!(state);
    }

    // recursion
    {
        use std::fmt::Write;
        let mut out = String::new();
        write_html!(&mut out,
                    html[
                        head[title["Macros guide"]]
                            body[h1["Macros are the best!"]]
                    ]);
        println!("out: {}", out);
    }

}
