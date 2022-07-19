use std::io;

pub fn get_input(prompt: &str) -> String {
    println!("{}",prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Falha!");
    input.trim().to_string()
}

pub fn parse_i32(string: String) -> i32 {
    string.trim().parse().unwrap_or(0)
}

// pub fn parse_f64(string: String) -> f64 {
//     string.trim().parse().unwrap_or(0.0)
// }

pub fn clear() {
    print!("\x1B[2J\x1B[1;1H");
}
