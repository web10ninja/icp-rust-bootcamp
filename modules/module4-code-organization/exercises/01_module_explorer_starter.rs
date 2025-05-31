mod products {
    #[derive(Debug, Clone)]
    pub struct Product {
        pub id: u32,
        pub name: String,
        pub price: f64,
    }

    impl Product {
        pub fn new(id: u32, name: &str, price: f64) -> Self {
            Product {
                id,
                name: name.to_string(),
                price,
            }
        }
    }
}

mod users {
    #[derive(Debug)]
    pub struct User {
        pub id: u32,
        pub name: String,
    }

    impl User {
        pub fn new(id: u32, name: &str) -> Self {
            User {
                id,
                name: name.to_string(),
            }
        }
    }
}

mod inventory {
    use super::products::Product;

    #[derive(Debug)]
    pub struct Inventory {
        pub items: Vec<Product>,
    }

    impl Inventory {
        pub fn new() -> Self {
            Inventory { items: Vec::new() }
        }

        pub fn add_product(&mut self, product: Product) {
            self.items.push(product);
        }
    }
}

mod orders {
    use super::products::Product;
    use super::users::User;

    #[derive(Debug)]
    pub struct Order {
        pub id: u32,
        pub user: User,
        pub products: Vec<Product>,
    }

    impl Order {
        pub fn new(id: u32, user: User, products: Vec<Product>) -> Self {
            Order { id, user, products }
        }

        pub fn print_details(&self) {
            println!("Order #{} for user: {}", self.id, self.user.name);
            println!("Products:");
            for p in &self.products {
                println!("  - {} (${:.2})", p.name, p.price);
            }
        }
    }
}

// Bring types into scope for main
use products::Product;
use users::User;
use inventory::Inventory;
use orders::Order;

fn main() {
    // 1. Create some products
    let p1 = Product::new(1, "Rust Book", 39.99);
    let p2 = Product::new(2, "T-shirt", 19.99);

    // 2. Add products to inventory (clone so we can use p1/p2 again)
    let mut inv = Inventory::new();
    inv.add_product(p1.clone());
    inv.add_product(p2.clone());

    // 3. Create a user
    let user = User::new(1, "Alice");

    // 4. Create an order for the user with some products
    let order = Order::new(1, user, vec![p1, p2]);

    // 5. Print order details
    order.print_details();
}