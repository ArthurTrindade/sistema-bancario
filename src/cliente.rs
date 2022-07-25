use colored::*;
use crate::utilitario::{clear, get_input, parse_i32};


#[derive(Debug)]
pub struct Cliente {
    pub codigo: i32,
    pub cpf: String,
    pub nome: String,
    pub telefone: String,
    pub endereco: String
}

impl Cliente {

    pub fn new(codigo: i32, nome: String, cpf: String, telefone: String, endereco: String) -> Self {
        Self { codigo, cpf, nome, telefone, endereco }
    }  
    
    pub fn mostrar_cliente(&self) {
        println!("CÓDIGO: {}\nNOME: {}\nCPF: {}\nTELEFONE: {}\nENDEREÇO: {}\n", self.codigo, self.nome, self.cpf, self.telefone, self.endereco)
    }

}

pub fn menu_gerenciar_cliente() {
    clear();
    loop {
        println!("=========== Gerenciar Clientes ============");
        println!("Digite um comando para prosseguir: ");
        println!("C - Cadastrar um cliente");
        println!("L - Listar todos os clientes cadastrados");
        println!("B - Buscar cliente já cadastrado");
        println!("A - Atualizar um cliente cadastrado");
        println!("E Excluir um cliente cadastrado");
        println!("S - Sair");

        let opcao: &str = &get_input("\nEscolha uma opção: ").to_uppercase();

        match opcao {
            "C" => println!("\nCadastrar cliente\n"),
            "L" => println!("\nListar todos os clientes cadastrados\n"),
            "B" => println!("\nBuscar cliente já cadastrado\n"),
            "A" => println!("\nAtualizar um cliente cadastrado\n"),
            "E" => println!("\nExcluir um cliente cadastrado\n"),
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
