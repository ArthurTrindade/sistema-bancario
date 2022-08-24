use sistema_bancario::*;

use crate::utilitario::*;
use crate::cliente::*;
use crate::conta::*;

use colored::*;

fn main() {
    
    // let pessoa1: Cliente = Cliente { codigo: (1), cpf: String::from("Arthur"), nome: String::from("Arthur"), telefone: String::from("Arthur"), endereco: String::from("Arthur") };

    // let pessoa2: Cliente = Cliente::new(2, String::from("Trindade"), String::from("048381"), String::from("62 991457490"), String::from("Rua"));
    
    // unsafe { CLIENTES.push(pessoa1) };
    // pessoa2.cadastrar_cliente();
  
    // for cliente in unsafe { CLIENTES.iter() } {
    //     cliente.mostrar_cliente();
    // }

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
                println!("{}", MSG_ERROR_INPUT.red());
            }
        }
    }
}
