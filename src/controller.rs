use crate::user::User;
use std::collections::HashMap;

pub struct Controller {    
    users: HashMap<String, User>
}

impl Controller {
    pub fn build_controller() -> Controller {
        Controller {
            users: HashMap::new()
        }
    }

    pub fn add_or_update_user(&mut self, user: User){
        //Why a clone() is needed -> https://stackoverflow.com/a/72778152
        self.users.insert(user.get_email().clone(), user);
    }

    pub fn remove_user(&mut self, email: &String){
        self.users.remove(email);
    }

    pub fn get_user(&mut self, email: &String) -> Option<&mut User> {
        self.users.get_mut(email)
    }    

}




