use std::io;

fn main() {
    println!("Welcome! Please Take Your Order!");

    let mut total_cost: f32 = 0.0;

    loop {
        println!("\n FOOD ITEMS MENU:");
        println!(" FOOD ITEMS                        Price (₦)");
        println!("P =    Poundo Yam/Edinkaiko Soup   - 3,200");
        println!("F =    Fried Rice & Chicken        - 3,000");
        println!("A =    Amala & Ewedu Soup          - 2,500");
        println!("E =    Eba & Egusi Soup            - 2,000");
        println!("W =    White Rice & Stew           - 2,500");

        // Get food item
        println!("\nWhat would you like? (P, F, A, E, or W):");
        let mut foodtype = String::new();
        io::stdin()
            .read_line(&mut foodtype)
            .expect("Failed to read food type");
        let foodtype = foodtype.trim().to_uppercase();
        

        // To get price
        let price:f32;

        if foodtype == "P" {
            price = 3_200.0;
        } else if foodtype == "F" {
            price = 3_000.0;
        } else if foodtype == "A" {
            price = 2_500.0;
        } else if foodtype == "E" {
            price = 2_000.0;
        } else if foodtype == "W" {
            price = 2_500.0;
        } else {
            println!("Sorry, that's not on the menu! Please enter P, F, A, E, or W.");
            continue;
        }

        // Get quantity
        println!("How many portions would you like: ");
        let mut q_input = String::new();
        io::stdin()
            .read_line(&mut q_input)
            .expect("Failed to read quantity");
        let quantity:f32 = q_input.trim().parse().expect("Invalid input for quantity");

        
        // Calculate total cost
        let cost:f32 = price * quantity;
        println!("Your order has been saved!");
        println!("The cost for this is ₦{:.2}", cost);
        total_cost += cost;
        println!("Your total is ₦{:.2} currently.", total_cost);

        // Ask if user wants to continue
        println!("\nWould you like to add anything else? (Y or N): ");
        let mut again_input = String::new();
        io::stdin()
            .read_line(&mut again_input)
            .expect("Failed to read input");
        let again = again_input.trim();

        if again == "N" || again == "n" || again == "No" || again == "NO"{
            break;
        }
    }

    // Apply discount if necessary
    if total_cost > 10_000.0 {
        let discount = total_cost * 0.05;
        total_cost -= discount;
        println!("\nCongrats! You have earned a 5% discount!");
    }

    println!("Final total charge: ₦{:.2}", total_cost);

    println!("Thank you for ordering from us!")
}
