use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize, Debug)]
pub struct Pager<T: Serialize> {
    pub total: i64,
    pub rows: Vec<T>,
}