extern crate lambda_runtime;
extern crate serde_derive;

use std::error::Error;
use serde_derive::{Serialize, Deserialize};
use lambda_runtime::{lambda, Context, error::HandlerError};

mod users;

fn main() -> Result<(), Box<dyn Error>> {
    lambda!(user_exists_handler);
    let s3_client = users::init();
    users::list_bucket_sync(&s3Client);
    Ok(())
}

fn user_exists_handler(event: UserExistsEvent, _ctx: Context) -> Result<UserExistsResult, HandlerError> {
    Ok(user_exists(event.username.as_str()))
}

enum UserExistsError {
    InvalidFormat,
    UserNotFound
}

impl UserExistsError {
    fn value(&self) -> String {
        match *self {
            UserExistsError::InvalidFormat => String::from("invalid_format"),
            UserExistsError::UserNotFound => String::from("user_not_found")
        }
    }
}

#[derive(Serialize, Deserialize)]
struct UserExistsResult {
    exists: bool,
    errors: Vec<String>
}

#[derive(Serialize, Deserialize)]
struct UserExistsEvent {
    username: String
}

fn user_exists(username: &str) -> UserExistsResult {
    let mut result = UserExistsResult { exists: true, errors: Vec::new() };

    if !validate_username_length(username) {
        result.exists = false;
        result.errors.push(UserExistsError::InvalidFormat.value());
    }

    if !validate_username_alphanumeric(username) {
        result.exists = false;
        result.errors.push(UserExistsError::InvalidFormat.value());
    }

    if !username_exists(username) {
        result.exists = false;
        result.errors.push(UserExistsError::UserNotFound.value());
    }

    return result;
}

fn validate_username_length(username: &str) -> bool {
    username.chars().count() >= 6
}

fn validate_username_alphanumeric(username: &str) -> bool {
    username.chars().all(char::is_alphanumeric) && username.chars().any(char::is_alphabetic)
}

fn username_exists(username: &str) -> bool {
    let existing_usernames = vec!["tizio90", "caio83", "sempronio85"];
    existing_usernames.contains(&username)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn user_exists_result_for_invalid_length() {
        let username = "otto";
        let user_exists_result = user_exists(username);
        assert_eq!(false, user_exists_result.exists);
        assert_eq!(true, user_exists_result.errors.contains(&String::from("invalid_format")))
    }

    #[test]
    fn user_exists_result_for_invalid_characters() {
        let username = "o.t.t.o.";
        let user_exists_result = user_exists(username);
        assert_eq!(false, user_exists_result.exists);
        assert_eq!(true, user_exists_result.errors.contains(&String::from("invalid_format")))
    }

    #[test]
    fn user_exists_result_for_user_not_found() {
        let username = "otto88";
        let user_exists_result = user_exists(username);
        assert_eq!(false, user_exists_result.exists);
        assert_eq!(true, user_exists_result.errors.contains(&String::from("user_not_found")))
    }

    #[test]
    fn user_exists_result_for_existing_user() {
        let username = "tizio90";
        let user_exists_result = user_exists(username);
        assert_eq!(true, user_exists_result.exists);
        assert_eq!(true, user_exists_result.errors.is_empty())
    }

    #[test]
    fn validates_length_of_four_characters_as_invalid() {
        let username = "four";
        let validation_result = validate_username_length(username);
        assert_eq!(false, validation_result);
    }

    #[test]
    fn validates_length_of_six_characters_as_valid() {
        let username = "sixsix";
        let validation_result = validate_username_length(username);
        assert_eq!(true, validation_result);
    }

    #[test]
    fn validates_length_of_ten_characters_as_valid() {
        let username = "testtheten";
        let validation_result = validate_username_length(username);
        assert_eq!(true, validation_result);
    }

    #[test]
    fn validates_string_with_numbers_as_invalid() {
        let username = "123456";
        let validation_result = validate_username_alphanumeric(username);
        assert_eq!(false, validation_result);
    }

    #[test]
    fn validates_string_with_alphabetic_characters_as_valid() {
        let username = "testalphabetic";
        let validation_result = validate_username_alphanumeric(username);
        assert_eq!(true, validation_result);
    }

    #[test]
    fn validates_string_with_special_characters_as_invalid() {
        let username = "testspecial!";
        let validation_result = validate_username_alphanumeric(username);
        assert_eq!(false, validation_result);
    }

    #[test]
    fn username_exists_not_existing_username_as_false() {
        let username = "otto88";
        let exists = username_exists(username);
        assert_eq!(false, exists);
    }

    #[test]
    fn username_exists_existing_username_as_true() {
        let username = "caio83";
        let exists = username_exists(username);
        assert_eq!(true, exists);
    }
}
