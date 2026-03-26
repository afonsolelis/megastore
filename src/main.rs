mod file_loader;
mod models;
mod search;

use models::Product;
use std::collections::HashMap;
use std::io::{self, Write};

fn print_help() {
    println!("Comandos disponíveis:");
    println!("  search <termo>  Busca por nome ou descrição");
    println!("  help            Exibe esta ajuda");
    println!("  exit            Encerra o programa");
}

fn main() {
    println!("Carregando produtos...");
    let products_map: HashMap<i32, Product> = match file_loader::load_products("./data") {
        Ok(map) => {
            println!("{} produtos carregados. Motor de busca pronto.", map.len());
            map
        }
        Err(e) => {
            eprintln!("Erro ao carregar produtos: {}", e);
            return;
        }
    };

    loop {
        print!("> ");
        if let Err(error) = io::stdout().flush() {
            eprintln!("Erro ao atualizar o terminal: {}", error);
            break;
        }

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(0) => {
                println!();
                println!("Saindo...");
                break;
            }
            Ok(_) => {}
            Err(error) => {
                eprintln!("Erro ao ler comando: {}", error);
                continue;
            }
        }

        let trimmed_input = input.trim();
        let parts: Vec<&str> = trimmed_input.splitn(2, char::is_whitespace).collect();
        let command = parts.get(0).unwrap_or(&"");
        let args = parts.get(1).map(|arg| arg.trim()).unwrap_or("");

        match *command {
            "search" => {
                if args.is_empty() {
                    println!("Uso: search <termo>");
                    continue;
                }

                let results = search::search_products(args, &products_map);

                if results.is_empty() {
                    println!("Nenhum produto encontrado para '{}'.", args);
                } else {
                    for product in results {
                        println!(
                            "ID: {} | Nome: {} | Categoria: {} | Preço: {:.2}",
                            product.id, product.name, product.data.category, product.data.price
                        );

                        if !product.edges.is_empty() {
                            println!("  -> Relações:");
                            for edge in &product.edges {
                                if let Some(related_product) = products_map.get(&edge.to_id) {
                                    println!(
                                        "     - {}: {} (ID: {})",
                                        edge.relationship, related_product.name, related_product.id
                                    );
                                } else {
                                    println!(
                                        "     - {}: Produto relacionado não encontrado (ID: {})",
                                        edge.relationship, edge.to_id
                                    );
                                }
                            }
                        }
                    }
                }
            }
            "exit" => {
                println!("Saindo...");
                break;
            }
            "help" => {
                print_help();
            }
            "" => {
                // Ignore empty input
            }
            _ => {
                println!(
                    "Comando desconhecido: '{}'. Use 'help' para listar os comandos.",
                    command
                );
            }
        }
    }
}
