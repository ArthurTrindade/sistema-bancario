mod cliente;
mod conta;
mod utilitario;

use colored::*;
use text_io::*;

use crate::cliente::menu_gerenciar_cliente;
use crate::conta::menu_gerenciar_conta;
use crate::utilitario::{clear, get_input, parse_i32};

use cliente::Cliente;

fn main() {

    // let mut pessoas: Vec<Cliente> = Vec::with_capacity(200);

    // let pessoa1: Cliente = Cliente::new(1, String::from("Arthur"), String::from("048.381.091-64"), String::from("62 991457490"), String::from("Rua Maranhão, 568"));

    // pessoa1.mostrar_cliente();



    menu_principal();
}

fn menu_principal() {
    clear();

    loop {
        println!("=========== Bem Vindo! ============");
        println!("Digite um comando para prosseguir: ");
        println!("C - Gerenciar Clientes");
        println!("T - Gerenciar Contas");
        println!("S - Sair");
        
        let opcao: &str = &get_input("\nDigite uma opção: ").to_uppercase();
        
        match opcao {
            "C" => menu_gerenciar_cliente(),
            "T" => menu_gerenciar_conta(),
            "S" => {
                clear();
                println!("Até main!!");
                break;
            }
            _ => {
                clear();
                println!("{}", "\n** Comando inválido digite uma opção válida!! **\n".red())
            }
        }
    }
}
