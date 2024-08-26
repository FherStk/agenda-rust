pub mod user;
pub mod controller;

use crate::controller::Controller;
use crate::user::User;

fn main() {
    let mut controller = Controller::build_controller();

    let user = User::build_user(String::from("user1@example.com"), String::from("User 1"));
    controller.add_or_update_user(user);
    
    let user = User::build_user(String::from("user2@example.com"), String::from("User 2"));
    controller.add_or_update_user(user);

    let user = User::build_user(String::from("user3@example.com"), String::from("User 3"));
    controller.add_or_update_user(user);
    
    println!();

    print(&mut controller, String::from("User 1"));
    print(&mut controller, String::from("user1@example.com"));
    print(&mut controller, String::from("user2@example.com"));
    print(&mut controller, String::from("user3@example.com"));

    let email = String::from("user1@example.com");
    let user = controller.get_user(&email).unwrap(); //assumes exists    
    //user.username = String::from("Username 1 [modified]"); //Not allowed as intended!
    user.set_username(String::from("Username 1 [modified]"));    
    print(&mut controller, email);

    let email = String::from("user2@example.com");
    controller.remove_user(&email);
    print(&mut controller, email);
}

fn print(controller: &mut Controller, email: String){
    let user = controller.get_user(&email);

    match user {
        Some(_) =>{
            let data = user.unwrap().to_string();
            println!("{data}");
        },
        None => println!("No user found with email '{email}'\n")
    }
}
