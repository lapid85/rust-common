#[macro_use]
extern crate lazy_static;

pub mod clients;
pub mod config;
pub mod consts;
pub mod mvc;
pub mod response;
pub mod tables;
pub mod types;
pub mod utils;
pub mod request;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
