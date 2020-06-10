use detail_error::DetailError;

#[derive(DetailError)]
pub enum BusinessError {
    InvalidEmail,
    #[detail(code=400, message="this is an invalid password")]
    InvalidPassword
}

impl BusinessError {

    pub fn get_message(&self) -> String {
        match self {
            BusinessError::InvalidEmail => String::from("Invalid email"),
            BusinessError::InvalidPassword => String::from("Invalid password"),
        }
    }
}

fn main() {
    let error = BusinessError::InvalidPassword;
    println!("error {} {} {}",error.get_http_code(), error.get_code(),error.get_message());
}
