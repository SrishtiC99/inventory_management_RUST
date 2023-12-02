use serde;
use crate::file_utils;

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq)]
pub enum UserRole {
    Seller,
    Customer
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct User {
    pub phone_number: u64,
    pub name: String,
    pub address: String,
    pub role: UserRole
}

impl User {
    
    fn new(phone_number: u64, name: String, address: String) -> Self {
        User {
            phone_number,
            name,
            address,
            role: UserRole::Customer
        }
    }

    fn update_role(&mut self) {
        self.role = UserRole::Seller;
    }
}

pub fn register_user(phone_number: u64, name: String, address: String) {
    let new_user = User::new(phone_number, name, address);
    let mut user_list = get_all_users();
    user_list.push(new_user);

    if let Err(err) = file_utils::save_file(String::from("users.json"), &user_list) {
        eprintln!("Error saving user list: {}", err);
    }
    println!("User has been registered successfully!!\n");
}

pub fn make_business_account(phone_number: u64) {
    let mut user_list = get_all_users();

    if let Some(user) = user_list.iter_mut().find(|u| u.phone_number == phone_number) {
        user.update_role();
    }
    else {
        println!("User not found");
    }
    
    if let Err(err) = file_utils::save_file(String::from("users.json"), &user_list) {
        eprintln!("Error saving user list: {}", err);
    }
    println!("Thank you for subscribing to bussiness account\n!");
}

pub fn get_all_users() -> Vec<User> {
    let contents = file_utils::read_file(String::from("users.json"));
    let user_list: Vec<User> = serde_json::from_str(&contents).expect("Error deserializing file data");
    user_list
}

pub fn find_user_by_phone_number(phone_number: u64) -> Option<User> {
    get_all_users().into_iter().find(|u| u.phone_number == phone_number)
}

