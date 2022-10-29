struct Product {
    name: String,
    price: f32,
    is_available: bool,
    category: ProductCategory
}

enum ProductCategory { 
    Electronics,
    Cleaning,
    Farming
}

impl Product {
    fn new (name: String, price: f32, category: ProductCategory) -> Product {
        Product {
            name: name,
            price: price,
            is_available: true,
            category
        }
    }
    fn calculate_sales_tax (&self) -> f32 {
        self.price * 0.1
    }
    fn set_price (&mut self, new_price: f32) {
        self.price = new_price;
    }
    fn buy (self) -> i32 {
        let name = self.name;
        println!("{name} was bought");
        1234
    }
    fn get_sales_tax () -> f32 {
        0.1
    }
}

enum Command {
    Undo,
    Redo,
    AddText(String),
    MoveCursor(i32, i32),
    Replace {
        from: String,
        to: String,
    }
}

impl Command {
    fn serialize (&self) -> String {
        String::from("JSON string")
    }
}

fn main() {
    println!("Hello, world!");
    let category = ProductCategory::Electronics;
    let mut book = Product {
        name: String::from("Fire and blood"),
        price: 45.5,
        is_available: true,
        category: category
    };
    let sales_tax = book.calculate_sales_tax();
    println!("sales tax is: {sales_tax}");
    book.set_price(2.2);
    book.buy();

    let mut book2 = Product::new(String::from("winds of winter"), 20.2, ProductCategory::Electronics);
    let sales_tax2 = book2.calculate_sales_tax();
    println!("sales tax is: {sales_tax2}");
    book2.set_price(40.2);
    book2.buy();

    let command1 = Command::Undo;
    let command2 = Command::AddText(String::from("test"));
    let command3 = Command::MoveCursor(2, 4);
    let command4 = Command::Replace {
        from: String::from("a"),
        to: String::from("b"),
    };

    let json_string = command1.serialize();
    println!("{json_string}");
}