use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Product {
    pub id: u32,
    pub name: String,
    pub category: String,
}

impl Product {
    pub fn new(id: u32, name: &str, category: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            category: category.to_string(),
        }
    }
}

pub struct ProductSearch {
    index: HashMap<String, Vec<u32>>,
    products: HashMap<u32, Product>,
}

impl ProductSearch {
    pub fn new() -> Self {
        Self {
            index: HashMap::new(),
            products: HashMap::new(),
        }
    }

    pub fn add_product(&mut self, product: Product) {
        for word in product.name.split_whitespace().map(|w| w.to_lowercase()) {
            self.index.entry(word).or_default().push(product.id);
        }
        self.products.insert(product.id, product);
    }

    pub fn search(&self, term: &str) -> Vec<&Product> {
        if let Some(ids) = self.index.get(&term.to_lowercase()) {
            ids.iter().filter_map(|id| self.products.get(id)).collect()
        } else {
            vec![]
        }
    }
}
