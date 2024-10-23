#[derive(Debug)]
pub struct InvalidFENStringError {
    pub message: String,
}

impl InvalidFENStringError {
    pub fn new(message: &str) -> Self {
        let mut full_message = String::from("Invalid FEN string. ");
        full_message.push_str(message);

        InvalidFENStringError {
            message: full_message,
        }
    }
}
