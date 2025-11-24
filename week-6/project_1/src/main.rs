use std::io;

// Food item structure
struct FoodItem {
    code: char,
    name: String,
    price: f64,
}

impl FoodItem {
    fn new(code: char, name: &str, price: f64) -> Self {
        FoodItem {
            code,
            name: name.to_string(),
            price,
        }
    }
}

// Order structure
struct Order {
    item: FoodItem,
    quantity: u32,
}

impl Order {
    fn new(item: FoodItem, quantity: u32) -> Self {
        Order { item, quantity }
    }

    fn calculate_total(&self) -> f64 {
        self.item.price * self.quantity as f64
    }
}

fn main() {
    println!("{}", "=".repeat(60));
    println!("{:^60}", "WELCOME TO OUR RESTAURANT");
    println!("{}", "=".repeat(60));

    // Initialize menu items
    let menu_items = vec![
        FoodItem::new('P', "Poundo Yam/Edinkaiko Soup", 3200.0),
        FoodItem::new('F', "Fried Rice & Chicken", 3000.0),
        FoodItem::new('A', "Amala & Ewedu Soup", 2500.0),
        FoodItem::new('E', "Eba & Egusi Soup", 2000.0),
        FoodItem::new('W', "White Rice & Stew", 2500.0),
    ];

    let mut orders: Vec<Order> = Vec::new();
    let mut total_amount = 0.0;

    loop {
        display_menu(&menu_items);
        
        // Get food type input
        let food_code = get_food_code();
        
        if food_code == 'Q' {
            break;
        }

        // Find the selected food item
        let selected_item = match menu_items.iter().find(|item| item.code == food_code) {
            Some(item) => item,
            None => {
                println!("❌ Invalid food code! Please try again.");
                continue;
            }
        };

        // Get quantity input
        let quantity = get_quantity_input(&selected_item.name);
        
        // Create and add order
        let order = Order::new(FoodItem::new(
            selected_item.code,
            &selected_item.name,
            selected_item.price
        ), quantity);
        
        let order_total = order.calculate_total();
        orders.push(order);
        total_amount += order_total;

        println!("✅ Added {} x {} = N{:.2}", quantity, selected_item.name, order_total);
        println!("📊 Current Total: N{:.2}\n", total_amount);
    }

    // Process final bill
    if total_amount > 0.0 {
        generate_receipt(&orders, total_amount);
    } else {
        println!("\n👋 Thank you for visiting! No orders were placed.");
    }
}

fn display_menu(menu_items: &[FoodItem]) {
    println!("\n{}", "🍽️".repeat(20));
    println!("{:^40}", "MENU");
    println!("{}", "🍽️".repeat(20));
    
    for item in menu_items {
        println!("{} = {} - N{:.2}", item.code, item.name, item.price);
    }
    println!("{}", "-".repeat(40));
    println!("Q = Quit and Generate Bill");
    println!();
}

fn get_food_code() -> char {
    loop {
        print!("Enter food code (P, F, A, E, W) or 'Q' to quit: ");
        io::Write::flush(&mut io::stdout()).expect("Flush failed");
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        
        let input = input.trim().to_uppercase();
        
        if input.is_empty() {
            println!("❌ Please enter a food code!");
            continue;
        }
        
        let ch = input.chars().next().unwrap();
        
        if ch == 'Q' || ch == 'P' || ch == 'F' || ch == 'A' || ch == 'E' || ch == 'W' {
            return ch;
        } else {
            println!("❌ Invalid code! Please enter P, F, A, E, W, or Q.");
        }
    }
}

fn get_quantity_input(item_name: &str) -> u32 {
    loop {
        print!("Enter quantity for {}: ", item_name);
        io::Write::flush(&mut io::stdout()).expect("Flush failed");
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        
        match input.trim().parse::<u32>() {
            Ok(quantity) if quantity > 0 => {
                return quantity;
            }
            Ok(_) => {
                println!("❌ Quantity must be greater than 0!");
            }
            Err(_) => {
                println!("❌ Please enter a valid number!");
            }
        }
    }
}

fn generate_receipt(orders: &[Order], mut total_amount: f64) {
    println!("\n{}", "=".repeat(60));
    println!("{:^60}", "ORDER RECEIPT");
    println!("{}", "=".repeat(60));
    
    // Display order details
    println!("\n{:<30} {:<10} {:<10} {:<10}", 
             "ITEM", "QTY", "PRICE", "SUBTOTAL");
    println!("{}", "-".repeat(60));
    
    for order in orders {
        let subtotal = order.calculate_total();
        println!("{:<30} {:<10} N{:<8.2} N{:<8.2}", 
                 order.item.name, 
                 order.quantity, 
                 order.item.price, 
                 subtotal);
    }
    
    println!("{}", "-".repeat(60));
    
    // Calculate discount
    let discount = if total_amount > 10000.0 {
        let discount_amount = total_amount * 0.05;
        println!("{:<40} N{:<8.2}", "Subtotal:", total_amount);
        println!("{:<40} N{:<8.2} (5%)", "Discount:", discount_amount);
        total_amount -= discount_amount;
        discount_amount
    } else {
        0.0
    };
    
    println!("{}", "=".repeat(60));
    println!("{:<40} N{:<8.2}", "TOTAL AMOUNT:", total_amount);
    println!("{}", "=".repeat(60));
    
    // Additional information
    if discount > 0.0 {
        println!("🎉 Congratulations! You saved N{:.2} with our 5% discount!", discount);
    }
    
    println!("\n{}", "❤️".repeat(20));
    println!("{:^40}", "THANK YOU FOR YOUR ORDER!");
    println!("{:^40}", "COME BACK SOON!");
    println!("{}", "❤️".repeat(20));
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_calculation() {
        let item = FoodItem::new('P', "Test Item", 3200.0);
        let order = Order::new(item, 2);
        assert_eq!(order.calculate_total(), 6400.0);
    }

    #[test]
    fn test_discount_eligibility() {
        // Test case where discount should apply
        let total = 15000.0;
        let discount = if total > 10000.0 { total * 0.05 } else { 0.0 };
        assert_eq!(discount, 750.0);
        
        // Test case where discount should not apply
        let total = 5000.0;
        let discount = if total > 10000.0 { total * 0.05 } else { 0.0 };
        assert_eq!(discount, 0.0);
    }

    #[test]
    fn test_food_item_creation() {
        let item = FoodItem::new('A', "Amala", 2500.0);
        assert_eq!(item.code, 'A');
        assert_eq!(item.name, "Amala");
        assert_eq!(item.price, 2500.0);
    }
}

// Additional function to demonstrate sample order
fn demonstrate_sample_order() {
    println!("\n{}", "📋".repeat(30));
    println!("{:^60}", "SAMPLE ORDER DEMONSTRATION");
    println!("{}", "📋".repeat(30));
    
    let sample_orders = vec![
        Order::new(FoodItem::new('P', "Poundo Yam/Edinkaiko Soup", 3200.0), 2),
        Order::new(FoodItem::new('F', "Fried Rice & Chicken", 3000.0), 1),
        Order::new(FoodItem::new('W', "White Rice & Stew", 2500.0), 3),
    ];
    
    let mut total = 0.0;
    for order in &sample_orders {
        let order_total = order.calculate_total();
        total += order_total;
        println!("{} x {} = N{:.2}", order.quantity, order.item.name, order_total);
    }
    
    println!("Total before discount: N{:.2}", total);
    
    if total > 10000.0 {
        let discount = total * 0.05;
        let final_total = total - discount;
        println!("Discount (5%): N{:.2}", discount);
        println!("Final Total: N{:.2}", final_total);
    }
}