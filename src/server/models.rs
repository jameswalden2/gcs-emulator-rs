use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub struct Bucket {
    pub kind: String,
    pub name: String,
}

#[derive(Serialize, Debug)]
pub struct BucketList {
    pub items: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct CreateBucketRequest {
    pub bucket_name: String,
}
