use std::io::stdin;

fn ask_for_binary_number () -> String {
    println!("Type a binary number:");
    let mut string_input = String::new();
    stdin().read_line(&mut string_input).expect("Error to read");
    string_input
}

fn convert_binary_string_to_decimal (input: String) -> i32 {
    i32::from_str_radix(&input.trim(), 2).expect("YOU MUST TYPE ONLY 0 or 1")
}

fn main() {
    let user_input = ask_for_binary_number();
    let decimal = convert_binary_string_to_decimal(user_input);
    println!("decimal={}", decimal)
}
