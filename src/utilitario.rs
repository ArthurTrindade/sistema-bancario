use std::io;

pub const MSG_ERROR_INPUT: &'static str = "\n** Comando inválido digite uma opção válida!! **\n";

pub fn get_input(prompt: &str) -> String {
    println!("{}",prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Falha!");
    input.trim().to_string()
}

pub fn clear() {
    print!("\x1B[2J\x1B[1;1H");
}
