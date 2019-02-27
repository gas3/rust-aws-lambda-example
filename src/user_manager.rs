extern crate config;
extern crate rusoto_core;
extern crate rusoto_s3;
extern crate futures;
extern crate serde_derive;

use self::rusoto_core::Region;
use self::rusoto_s3::{S3, S3Client, GetObjectRequest};
use self::futures::{Future, Stream};
use self::serde_derive::{Deserialize, Serialize};

use std::str;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub name: String,
}

pub fn get_users() -> HashMap<String, User> {
    let mut conf = config::Config::default();
    // merge `./config.toml`
    conf.merge(config::File::with_name("config")).unwrap();

    let client = S3Client::new(Region::EuWest1);

    let request = GetObjectRequest {
        bucket: conf.get_str("s3-bucket").unwrap(),
        key: conf.get_str("s3-key").unwrap(),
        ..Default::default()
    };

    let result = client.get_object(request).sync()
        .expect("get_object failed");

    let stream = result.body.unwrap();
    let bytes = stream
        .concat2()
        .wait().unwrap();
    let body = str::from_utf8(&bytes).unwrap();

    let users: Vec<User> = serde_json::from_str(body)
        .expect("body deserialization failed");

    let user_map: HashMap<_, _> = users.into_iter()
        .map(|u| (u.username.clone(), u))
        .collect();

    user_map
}
