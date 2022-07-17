mod cliente;
mod conta;
mod utilitario;

use crate::cliente::menu_gerenciar_cliente;
use crate::conta::menu_gerenciar_conta;
use crate::utilitario::{clear, get_input, parse_i32};

fn main() {

    menu_principal();
}

fn menu_principal() {
    clear();

    loop {
        println!("=========== Bem Vindo! ============");
        println!("Digite um comando para prosseguir: ");
        println!("1 - Gerenciar Clientes");
        println!("2 - Gerenciar Contas");
        println!("3 - Sair");
        println!("Escolha: ");

        let opcao: i32 = parse_i32(get_input());

        match opcao {
            1 => menu_gerenciar_cliente(),
            2 => menu_gerenciar_conta(),
            3 => {
                clear();
                println!("Até main!!");
                break;
            }
            _ => println!("** Comando inválido digite uma opção válida!! **")
        }
    }
}
