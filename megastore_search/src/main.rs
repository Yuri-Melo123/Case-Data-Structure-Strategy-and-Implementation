use megastore_search::{Product, ProductSearch};
use std::io::{self, Write};

fn main() {
    let mut engine = ProductSearch::new();

    engine.add_product(Product::new(1, "Smartphone Samsung", "Eletrônicos"));
    engine.add_product(Product::new(2, "Notebook Dell", "Informática"));
    engine.add_product(Product::new(3, "Camiseta Nike", "Vestuário"));
    engine.add_product(Product::new(4, "Geladeira Brastemp", "Eletrodomésticos"));

    println!("Sistema de Busca MegaStore - Digite um termo ou 'sair' para encerrar.");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut entrada = String::new();
        io::stdin().read_line(&mut entrada).unwrap();
        let entrada = entrada.trim();

        if entrada.eq_ignore_ascii_case("sair") {
            println!("Encerrando...");
            break;
        }

        let resultados = engine.search(entrada);

        if resultados.is_empty() {
            println!("Nenhum produto encontrado para '{}'", entrada);
        } else {
            println!("Produtos encontrados:");
            for produto in resultados {
                println!("- [{}] {} ({})", produto.id, produto.name, produto.category);
            }
        }
    }
}
