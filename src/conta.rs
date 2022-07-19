use crate::utilitario::{clear, get_input, parse_i32};

pub fn menu_gerenciar_conta() {
    clear();

    loop {
        println!("=========== Gerenciar Contas ============");
        println!("Digite um comando para prosseguir: ");
        println!("1 - Listagem de todas as contas cadastradas.");
        println!("2 - Cadastrar uma conta para um cliente.");
        println!("3 - Listar todas as contas de um cliente.");
        println!("4 - Realizar um saque em uma conta.");
        println!("5 - Realizar um depósito em uma conta.");
        println!("6 - Realizar transferência entre contas.");
        println!("7 - Exibir um extrato de uma conta.");
        println!("8 - Sair");

        let opcao: i32 = parse_i32(get_input("\nEscolha uma opção: "));

        match opcao {
            1 => println!("Cadastrar cliente"),
            2 => println!("Listar todos os clientes cadastrados"),
            3 => println!("Buscar cliente já cadastrado"),
            4 => println!("Atualizar um cliente cadastrado"),
            5 => println!("Excluir um cliente cadastrado"),
            6 => println!("Excluir um cliente cadastrado"),
            7 => println!("Excluir um cliente cadastrado"),
            8 => {
                clear();
                break;
            }
            _ => println!("** Comando inválido digite uma opção válida!! **")
        }
    }
}
