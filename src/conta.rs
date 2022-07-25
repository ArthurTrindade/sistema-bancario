use colored::*;
use crate::utilitario::{clear, get_input};

pub fn menu_gerenciar_conta() {
    clear();

    loop {
        println!("=========== Gerenciar Contas ============");
        println!("Digite um comando para prosseguir: ");
        println!("R - Listagem de todas as contas cadastradas.");
        println!("C - Cadastrar uma conta para um cliente.");
        println!("L - Listar todas as contas de um cliente.");
        println!("W - Realizar um saque em uma conta.");
        println!("D - Realizar um depósito em uma conta.");
        println!("T - Realizar transferência entre contas.");
        println!("E - Exibir um extrato de uma conta.");
        println!("S - Sair");

        let opcao: &str = &get_input("\nEscolha uma opção: ").to_uppercase();

        match opcao {
            "R" => println!("Cadastrar cliente"),
            "C" => println!("Listar todos os clientes cadastrados"),
            "L" => println!("Buscar cliente já cadastrado"),
            "W" => println!("Atualizar um cliente cadastrado"),
            "D" => println!("Excluir um cliente cadastrado"),
            "T" => println!("Excluir um cliente cadastrado"),
            "E" => println!("Excluir um cliente cadastrado"),
            "S" => {
                clear();
                break;
            }
            _ => {
                clear();
                println!("{}", "\n** Comando inválido digite uma opção válida!! **\n".red())
            }
        }
    }
}
