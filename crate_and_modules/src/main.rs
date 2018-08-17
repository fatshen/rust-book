extern crate crate_and_modules;

fn main() {
    println!("Hello in English: {}", crate_and_modules::english::greetings::hello());
    println!("Hello in English: {}", crate_and_modules::english::farewells::goodbye());

    println!("Hello in Japanese: {}", crate_and_modules::japanese::greetings::hello());
    println!("Hello in Japanese: {}", crate_and_modules::japanese::farewells::goodbye());
}
