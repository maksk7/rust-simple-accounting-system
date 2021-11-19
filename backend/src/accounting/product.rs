pub enum ProductType {
    Good,
    Service,
}

pub struct Product {
    name: String,
    description: String,
    sales_price: Option<f32>,
    purchase_price: Option<f32>,
    product_type: ProductType,
}

impl Product {
    async fn new(
        name: String,
        description: String,
        sales_price: Option<f32>,
        purchase_price: Option<f32>,
        product_type: ProductType,
    ) -> Self {
        Product {
            name,
            description,
            sales_price,
            purchase_price,
            product_type,
        }
    }
}
