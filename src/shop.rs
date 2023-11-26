use crate::{users::{User, self, UserRole}, file_utils};
use std::io;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Item {
    quantity: u32,
    name: String,
    price: f64
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Shop {
    owner: User,
    name: String,
    items: Vec<Item>
}

impl Shop {
    fn new(user: User, name: &str) -> Self{
        Shop {
            owner: user,
            name: name.to_string(),
            items: Vec::new()
        }
    }

    fn add_item(mut self, item: Item) {
        self.items.push(item);
    }

    fn update_item_quantity(mut self, itemname:String, quantity: u32) {
        if let Some(item) = self.items.iter_mut().find(|i| i.name == itemname) {
            item.quantity = quantity;
        }
    }
}


pub fn create_shop(owner_phone_number: u64, name: &str) {
    
    if exist_shop_by_phone_number(owner_phone_number) {
        println!("You already have a shop...\n");
        return;
    }

    let owner: Option<User> = users::find_user_by_phone_number(owner_phone_number);
    match owner {
        Some(user) => {
            if user.role == UserRole::Seller {
                let new_shop = Shop::new(user, name);

                let mut shop_list = match read_all_shop() {
                    Ok(list) => list,
                    Err(_) => Vec::new()
                };
                shop_list.push(new_shop);

                if let Err(err) = file_utils::save_file("shop.json", &shop_list) {
                    eprintln!("Error saving shop list: {}", err);
                }

                println!("Shop created successfully!");
            }
            else {
                println!("Customer cannot create a shop. Please upgrade to Business Account.")
            }
        }
        None => {
            println!("Owner not found with given phone number")
        }
    }
}

pub fn add_item(phone_number: u64, item_name: &str, price: f64, quantity: u32) {
    let mut shop_list = match read_all_shop() {
        Ok(list) => list,
        Err(_) => Vec::new()
    };

    if let Some(shop) = shop_list.iter_mut().find(|sh| sh.owner.phone_number == phone_number) {
        let item = Item { quantity: quantity, name: item_name.to_string(), price: price };
        shop.items.push(item);

        if let Err(err) = file_utils::save_file("shop.json", &shop_list) {
            eprintln!("Error saving shop list: {}", err);
        }
    }
    else {
        println!("Shop not found with given phone number")
    }

    println!("Item added successfully!");
}

pub fn update_item_quantity(phone_number: u64, item_name: &str, quantity: u32) {
    
    let mut shop_list = match read_all_shop() {
        Ok(list) => list,
        Err(_) => Vec::new()
    };

    if let Some(shop) = shop_list.iter_mut().find(|sh| sh.owner.phone_number == phone_number) {
        if let Some(item) = shop.items.iter_mut().find(|i| i.name == item_name) {
            item.quantity = quantity;

            if let Err(err) = file_utils::save_file("shop.json", &shop_list) {
                eprintln!("Error saving shop list: {}", err);
            }
        }
        else {
            println!("item: {} not found", item_name);
        }
    }
    else {
        println!("Shop not found with given phone number")
    }
}

pub fn read_all_shop() -> io::Result<Vec<Shop>>{
    let contents = file_utils::read_file("shop.json");
    let shop_list: Vec<Shop> = serde_json::from_str(&contents).expect("Error deserializing file data");
    Ok(shop_list)
}

pub fn exist_shop_by_phone_number(phone_number: u64) -> bool {
    let shop_list = match read_all_shop() {
        Ok(list) => list,
        Err(_) => Vec::new()
    };

    if shop_list.into_iter().find(|shop| shop.owner.phone_number == phone_number).is_some() {
        true
    }
    else {
        false
    }
}

