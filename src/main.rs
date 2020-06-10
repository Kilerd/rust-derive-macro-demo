
pub enum BusinessError {
    InvalidEmail,
    InvalidPassword
}

impl BusinessError {
    pub fn get_http_code(&self) -> u16 {
        match self {
            BusinessError::InvalidEmail => 400,
            BusinessError::InvalidPassword => 400,
        }
    }
    pub fn get_code(&self) -> String {
        match self {
            BusinessError::InvalidEmail => String::from("INVALID_EMAIL"),
            BusinessError::InvalidPassword => String::from("INVALID_PASSWORD"),
        }
    }
    pub fn get_message(&self) -> String {
        match self {
            BusinessError::InvalidEmail => String::from("Invalid email"),
            BusinessError::InvalidPassword => String::from("Invalid password"),
        }
    }
}

fn main() {
    println!("Hello, world!");
}
