extern crate lambda_runtime;
extern crate serde_derive;

use std::error::Error;
use serde_derive::{Serialize, Deserialize};
use lambda_runtime::{lambda, Context, error::HandlerError};

use std::collections::HashMap;

mod user_manager;

use user_manager::{User, get_users};

fn main() -> Result<(), Box<dyn Error>> {
    lambda!(user_exists_handler);
    Ok(())
}

fn user_exists_handler(event: UserExistsEvent, _ctx: Context) -> Result<UserExistsResult, HandlerError> {
    let user_map = get_users();
    Ok(user_exists(event.username.as_str(), user_map))
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

fn user_exists(username: &str, user_map: HashMap<String, User>) -> UserExistsResult {
    let mut result = UserExistsResult { exists: true, errors: Vec::new() };

    if !validate_username_length(username) {
        result.exists = false;
        result.errors.push(UserExistsError::InvalidFormat.value());
    }

    if !validate_username_alphanumeric(username) {
        result.exists = false;
        result.errors.push(UserExistsError::InvalidFormat.value());
    }

    if !username_exists(username, user_map) {
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

fn username_exists(username: &str, user_map: HashMap<String, User>) -> bool {
    user_map.contains_key(username)
}
