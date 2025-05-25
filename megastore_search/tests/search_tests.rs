use megastore_search::{Product, ProductSearch};

#[test]
fn test_search_existing_product() {
    let mut engine = ProductSearch::new();
    engine.add_product(Product::new(1, "Smartphone Samsung", "Eletrônicos"));

    let result = engine.search("Samsung");
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].id, 1);
}

#[test]
fn test_search_non_existing_product() {
    let engine = ProductSearch::new();
    let result = engine.search("Inexistente");
    assert_eq!(result.len(), 0);
}
