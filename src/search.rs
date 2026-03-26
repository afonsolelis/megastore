use crate::models::Product;
use std::collections::HashMap;

fn normalize_text(text: &str) -> String {
    text.chars()
        .flat_map(|c| match c.to_ascii_lowercase() {
            'á' | 'à' | 'â' | 'ã' | 'ä' => ['a'].into_iter().collect::<Vec<_>>(),
            'é' | 'è' | 'ê' | 'ë' => ['e'].into_iter().collect::<Vec<_>>(),
            'í' | 'ì' | 'î' | 'ï' => ['i'].into_iter().collect::<Vec<_>>(),
            'ó' | 'ò' | 'ô' | 'õ' | 'ö' => ['o'].into_iter().collect::<Vec<_>>(),
            'ú' | 'ù' | 'û' | 'ü' => ['u'].into_iter().collect::<Vec<_>>(),
            'ç' => ['c'].into_iter().collect::<Vec<_>>(),
            other => [other].into_iter().collect::<Vec<_>>(),
        })
        .collect()
}

pub fn search_products<'a>(
    term: &str,
    products_map: &'a HashMap<i32, Product>,
) -> Vec<&'a Product> {
    let search_term = normalize_text(term);
    let mut results = Vec::new();

    for product in products_map.values() {
        let normalized_name = normalize_text(&product.name);
        let normalized_description = normalize_text(&product.description);

        if normalized_name.contains(&search_term) || normalized_description.contains(&search_term) {
            results.push(product);
        }
    }

    results.sort_by_key(|product| product.id);
    results
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Product, ProductData};

    fn create_mock_products() -> HashMap<i32, Product> {
        let mut products = HashMap::new();
        products.insert(
            1,
            Product {
                id: 1,
                name: "Laptop".to_string(),
                description: "A powerful laptop".to_string(),
                data: ProductData {
                    price: 1200.0,
                    category: "computers".to_string(),
                },
                edges: vec![],
            },
        );
        products.insert(
            2,
            Product {
                id: 2,
                name: "Mouse".to_string(),
                description: "A wireless mouse".to_string(),
                data: ProductData {
                    price: 25.0,
                    category: "accessories".to_string(),
                },
                edges: vec![],
            },
        );
        products.insert(
            3,
            Product {
                id: 3,
                name: "Keyboard".to_string(),
                description: "A mechanical keyboard".to_string(),
                data: ProductData {
                    price: 75.0,
                    category: "accessories".to_string(),
                },
                edges: vec![],
            },
        );
        products
    }

    #[test]
    fn test_search_found() {
        let products = create_mock_products();
        let results = search_products("laptop", &products);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Laptop");
    }

    #[test]
    fn test_search_not_found() {
        let products = create_mock_products();
        let results = search_products("projector", &products);
        assert!(results.is_empty());
    }

    #[test]
    fn test_search_multiple_results() {
        let products = create_mock_products();
        let results = search_products("a", &products);
        assert_eq!(results.len(), 3);
    }

    #[test]
    fn test_search_case_insensitivity() {
        let products = create_mock_products();
        let results = search_products("LaPtOp", &products);
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_search_ignores_accents() {
        let mut products = HashMap::new();
        products.insert(
            1,
            Product {
                id: 1,
                name: "Câmera A7S".to_string(),
                description: "Ótima para vídeos".to_string(),
                data: ProductData {
                    price: 15000.0,
                    category: "cameras".to_string(),
                },
                edges: vec![],
            },
        );

        let results = search_products("camera", &products);
        assert_eq!(results.len(), 1);
    }
}
