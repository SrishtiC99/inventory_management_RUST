use serde;
use std::io;
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
    
    fn new(phone_number: u64, name: &str, address: &str, role: UserRole) -> Self {
        User {
            phone_number,
            name: name.to_string(),
            address: address.to_string(),
            role
        }
    }
}

pub fn register_user(phone_number: u64, name: &str, address: &str) {
    let new_user = User::new(phone_number, name, address, UserRole::Customer);
    let mut user_list = match read_all_users() {
        Ok(list) => list,
        Err(_) => Vec::new()
    };
    user_list.push(new_user);

    if let Err(err) = file_utils::save_file("users.json", &user_list) {
        eprintln!("Error saving user list: {}", err);
    }
    println!("User has been registered successfully!!\n");
}

pub fn make_business_account(phone_number: u64) {
    let mut user_list = match read_all_users() {
        Ok(list) => list,
        Err(_) => Vec::new()
    };

    if let Some(user) = user_list.iter_mut().find(|u| u.phone_number == phone_number) {
        user.role = UserRole::Seller;
    }
    else {
        println!("User not found");
    }
    
    if let Err(err) = file_utils::save_file("users.json", &user_list) {
        eprintln!("Error saving user list: {}", err);
    }
    println!("Thank you for subscribing to bussiness account\n!");
}

pub fn read_all_users() -> io::Result<Vec<User>> {
    let contents = file_utils::read_file("users.json");
    let user_list: Vec<User> = serde_json::from_str(&contents).expect("Error deserializing file data");
    Ok(user_list)
}

