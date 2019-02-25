extern crate config;
extern crate rusoto_core;
extern crate rusoto_sts;
extern crate rusoto_s3;

// use std::collections::HashMap;

use self::rusoto_core::{Region, HttpClient};
use self::rusoto_sts::{StsClient, StsAssumeRoleSessionCredentialsProvider};
use self::rusoto_s3::{ListBucketsOutput, S3, S3Client};

pub fn init() -> S3Client {
    let mut conf = config::Config::default();
    // Add in `./config.toml`
    conf.merge(config::File::with_name("config")).unwrap();

    // Print out our settings (as a HashMap)
    // println!("{:?}", conf.clone().try_into::<HashMap<String, String>>().unwrap());

    let sts = StsClient::new(Region::EuWest1);

    let provider = StsAssumeRoleSessionCredentialsProvider::new(
        sts,
        conf.get_str("role-arn").unwrap().clone(),
        conf.get_str("session-name").unwrap().clone(),
        None, None, None, None
    );

    let client = S3Client::new_with(HttpClient::new().unwrap(), provider, Region::EuWest1);
    return client;
}

pub fn list_bucket_sync(s3: &S3) {
    match s3.list_buckets().sync() {
        Ok(out) => show_buckets(out),
        Err(e) => eprintln!("{:?}", e),
    }
}

fn show_buckets(out: ListBucketsOutput) {
    if let Some(buckets) = out.buckets {
        for b in buckets.iter() {
            println!("{}", b.name.as_ref().unwrap());
        }
    }
}