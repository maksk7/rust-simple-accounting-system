pub enum ProductType {
    Good,
    Service,
}

pub struct Product {
    name: String,
    description: String,
    sales_price: f32,
    purchase_price: f32,
    product_type: ProductType,
}

impl Product {
    fn new(
        name: String,
        description: String,
        sales_price: f32,
        purchase_price: f32,
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

    pub fn new_good(name: String, description: String, sales_price: f32, purchase_price: f32) -> Self {
        Self::new(
            name,
            description,
            sales_price,
            purchase_price,
            ProductType::Good,
        )
    }

    pub fn new_service(name: String, description: String, sales_price: f32, purchase_price: f32) -> Self {
        Self::new(
            name,
            description,
            sales_price,
            purchase_price,
            ProductType::Service,
        )
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name
    }

    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }

    pub fn set_sales_price(&mut self, sales_price: f32) {
        self.sales_price = sales_price;
    }

    pub fn set_purchase_price(&mut self, purchase_price: f32) {
        self.purchase_price = purchase_price;
    }
}
