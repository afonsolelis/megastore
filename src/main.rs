use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

#[derive(Deserialize, Debug)]
struct Edge {
    to_id: i32,
    relationship: String,
}

#[derive(Deserialize, Debug)]
struct ProductData {
    price: f64,
}

#[derive(Deserialize, Debug)]
struct Product {
    id: i32,
    name: String,
    description: String,
    data: ProductData,
    edges: Vec<Edge>,
}

fn load_products(dir_path: &str) -> Result<HashMap<i32, Product>, Box<dyn std::error::Error>> {
    let mut products = HashMap::new();
    let path = Path::new(dir_path);

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let file_path = entry.path();
        if file_path.is_file() && file_path.extension().and_then(|s| s.to_str()) == Some("json") {
            let file_content = fs::read_to_string(&file_path)?;
            let product: Product = serde_json::from_str(&file_content)?;
            products.insert(product.id, product);
        }
    }

    Ok(products)
}

fn main() {
    println!("Carregando produtos...");
    let products_map = match load_products("./data") {
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
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            continue;
        }

        let parts: Vec<&str> = input.trim().splitn(2, ' ').collect();
        let command = parts.get(0).unwrap_or(&"");
        let args = parts.get(1).unwrap_or(&"");

        match *command {
            "search" => {
                if args.is_empty() {
                    println!("Por favor, forneça um termo para buscar.");
                    continue;
                }
                let search_term = args.to_lowercase();
                let mut found = false;

                for product in products_map.values() {
                    if product.name.to_lowercase().contains(&search_term)
                        || product.description.to_lowercase().contains(&search_term)
                    {
                        found = true;
                        println!(
                            "ID: {} | Nome: {} | Preço: {:.2}",
                            product.id, product.name, product.data.price
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

                if !found {
                    println!("Nenhum produto encontrado para '{}'.", args);
                }
            }
            "exit" => {
                println!("Saindo...");
                break;
            }
            "" => {
                // Ignore empty input
            }
            _ => {
                println!("Comando desconhecido: '{}'. Comandos disponíveis: search, exit", command);
            }
        }
    }
}