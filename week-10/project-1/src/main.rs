struct Laptop {
    brand:String,
    price:u32,
    quantity:u32
}

impl Laptop {
    fn cost(&self, purchase_qtty:u32)->u32 {
        self.price * purchase_qtty
    }
}

fn main() {
    let hp = Laptop {
        brand:String::from("HP"),
        price:650_000,
        quantity:10
    };

    let ibm = Laptop {
        brand:String::from("IBM"),
        price:755_000,
        quantity:6
    };

    let toshiba = Laptop {
        brand:String::from("TOSHIBA"),
        price:550_000,
        quantity:10
    };

    let dell = Laptop {
        brand:String::from("DELL"),
        price:850_000,
        quantity:4
    };

    println!("Available Laptops:");
    println!("{} - {} units at N{} each", hp.brand, hp.quantity, hp.price);
    println!("{} - {} units at N{} each", ibm.brand, ibm.quantity, ibm.price);
    println!("{} - {} units at N{} each", toshiba.brand, toshiba.quantity, toshiba.price);
    println!("{} - {} units at N{} each\n", dell.brand, dell.quantity, dell.price);

    let purchase_qtty = 3;

    println!("{} laptops were purchased from each brand\n", purchase_qtty);

    let hp_cost = hp.cost(purchase_qtty);
    let ibm_cost = ibm.cost(purchase_qtty);
    let toshiba_cost = toshiba.cost(purchase_qtty);
    let dell_cost = dell.cost(purchase_qtty);



    let total_cost = hp_cost + ibm_cost + toshiba_cost + dell_cost;

    println!("Total Cost = N{}", total_cost);
}