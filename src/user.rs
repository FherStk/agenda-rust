pub struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

impl User {
    // Immutable access.
    pub fn get_active(&self) -> &bool {
        &self.active
    }

    pub fn get_username(&self) -> &String {
        &self.username
    }

    pub fn get_email(&self) -> &String {
        &self.email
    }

    pub fn get_sign_in_count(&self) -> &u64 {
        &self.sign_in_count
    }

    // Mutable access.
    // pub fn set_email(&mut self) -> &mut String {
    //     &mut self.email
    // }

    pub fn build_user(email: String, username: String) -> User {
        User {
            active: true,
            username,
            email,
            sign_in_count: 1,
        }
    }

    pub fn to_string(&self) -> String {
        let username = &self.username;
        let email = &self.email;
        let signin = &self.sign_in_count;
        let active = &self.active;
        
        format!("Username: {username}\nEmail: {email}\nSign in count: {signin}\nActive: {active}\n")
    }

    pub fn set_username(&mut self, username: String){
        self.username = username;
    }
}