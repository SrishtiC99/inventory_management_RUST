mod users;
mod file_utils;

use std::io::stdin;

use crate::users::UserRole;

fn register_user() {
    let mut input = String::new();
    
    println!("Please enter your Phone Number: ");
    stdin().read_line(&mut input).expect("Did not receive anything, pleae try again!\n\n");
    let phone_number: u64 = input.trim_end().parse().expect("Could not parse phone number");
    input.clear();

    println!("Please enter your Name: ");
    stdin().read_line(&mut input).expect("Did not receive anything, pleae try again!\n\n");
    let name: String = input.trim_end().to_string();
    input.clear();

    println!("Please enter your Address: ");
    stdin().read_line(&mut input).expect("Did not receive anything, pleae try again!\n\n");
    let address: String = input.trim_end().to_string();
    input.clear();

    users::register_user(phone_number, &name, &address);
}

fn make_business_account() {
    println!("Please enter your Phone Number: ");

    let mut input = String::new();
    stdin().read_line(&mut input).expect("Did not receive anything, pleae try again!\n\n");

    let phone_number: u64 = input.trim_end().parse().expect("Missing phone number");

    users::make_business_account(phone_number);
}

fn show_all_users() {
    let user_list = match users::read_all_users() {
        Ok(list) => list,
        Err(_) => Vec::new()
    };

    for user in &user_list {
        println!("Name: {}, Phone Number: {}, Address: {}, \nBussiness Account: {}\n------------------------\n\n", 
            user.name, user.phone_number, user.address,  
            if user.role == UserRole::Seller 
            {
                true
            }
            else 
            {
                false
            }
        );
    }
}

fn main() {
    println!("Welcome to our inventory System..\n");

    loop {
        println!("Please choose your action...\n");
        println!("Enter 1: for registration");
        println!("Enter 2: for business account");
        println!("Enter 3: for all users data");
        println!("Enter 4: for exit\n");

        let mut input = String::new();
        stdin().read_line(&mut input).expect("Did not receive anything, pleae try again!\n\n");
        match input.trim_end() {
            "1" => register_user(),
            "2" => make_business_account(),
            "3" => show_all_users(),
            "4" => break,
            _ => println!("Invalid input. Please try again!\n\n")
        }
    }
}
