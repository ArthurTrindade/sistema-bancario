use crate::utilitario::{clear, get_input, parse_i32};

pub struct Cliente {
    pub codigo: u16,
    pub nome: String,
    pub cpf: String,
    pub telefone: String,
    pub endereco: String
}

impl Cliente {
    //pub fn new()
    
}

pub fn menu_gerenciar_cliente() {
    clear();

    loop {
        println!("=========== Gerenciar Clientes ============");
        println!("Digite um comando para prosseguir: ");
        println!("1 - Cadastrar um cliente");
        println!("2 - Listar todos os clientes cadastrados");
        println!("3 - Buscar cliente já cadastrado");
        println!("4 - Atualizar um cliente cadastrado");
        println!("5 Excluir um cliente cadastrado");
        println!("6 - Sair");

        let opcao: i32 = parse_i32(get_input());

        match opcao {
            1 => println!("Cadastrar cliente"),
            2 => println!("Listar todos os clientes cadastrados"),
            3 => println!("Buscar cliente já cadastrado"),
            4 => println!("Atualizar um cliente cadastrado"),
            5 => println!("Excluir um cliente cadastrado"),
            6 => {
                clear();
                break;
            }
            _ => println!("** Comando inválido digite uma opção válida!! **")
        }
    }
}
