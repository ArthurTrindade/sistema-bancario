use colored::*;
use crate::utilitario::{clear, get_input};

pub const QTD_MAX_CLIENTES: usize = 200;

pub static mut CLIENTES: Vec<Cliente> = Vec::new();

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

    pub fn ler_dados_cliente() -> Cliente {
        let codigo: i32 = 1;
        let nome: String = get_input("Digite um nome: ");
        let cpf: String = get_input("Digite um cpf: ");
        let telefone: String = get_input("Digite um telefone: ");
        let endereco: String = get_input("Digite um endereco: ");
    
        let cliente: Cliente = Cliente::new(codigo, nome, cpf, telefone, endereco);
        cliente
    }

    pub fn cadastrar_cliente(self) {
        unsafe { CLIENTES.push(self) }
    }
    
    pub fn menu_cadastrar_cliente() {
       print!("\n=========== Cadastro de Clientes ===========\n");
       if unsafe { CLIENTES.len() } == QTD_MAX_CLIENTES {
            println!("Limite de Cliente atingidos!");
       }
       
       let cliente: Cliente = Cliente::ler_dados_cliente();
       
       cliente.cadastrar_cliente();
    }

    pub fn mostrar_cliente(&self) {
        println!();
        println!("Código  : {}", self.codigo);
        println!("Nome    : {}", self.nome);
        println!("CPF/CNPJ: {}", self.cpf);
        println!("Telefone: {}", self.telefone);
        println!("Endereço: {}", self.endereco);
        println!();
    }

    pub fn menu_listar_clientes() {    
        println!("\n=========== Listagem de Clientes ============");
        if unsafe { CLIENTES.len() } <= 0 {
            println!("{}", "** Nenhume Cliente Cadastrado **\n".red());
        }

        for cliente in unsafe { CLIENTES.iter() } {
            cliente.mostrar_cliente();
        }

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
        println!("E - Excluir um cliente cadastrado");
        println!("S - Sair");

        let opcao: &str = &get_input("\nEscolha uma opção: ").to_uppercase();

        match opcao {
            "C" => Cliente::menu_cadastrar_cliente(),
            "L" => Cliente::menu_listar_clientes(),
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
