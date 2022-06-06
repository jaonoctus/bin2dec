use clap::{arg, Command};

fn main() {
    let matches = Command::new("bin2dec")
        .version("1.0.0")
        .author("João Dias <jaonoctus@protonmail.com>")
        .about("Binary-to-Decimal number converter")
        .arg(
            arg!([BINARY_NUMBER] "a number including 0-1")
                .required(true) // TODO: add validator
        )
        .get_matches();

    let user_input = matches.value_of("BINARY_NUMBER").unwrap();
    let decimal = convert_binary_string_to_decimal(user_input);
    println!("{}", decimal);
}

fn convert_binary_string_to_decimal (input: &str) -> i32 {
    i32::from_str_radix(&input.trim(), 2).expect("YOU MUST TYPE ONLY 0 or 1")
}
