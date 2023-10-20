#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate rust_i18n;

pub mod clients;
pub mod config;
pub mod consts;
pub mod mvc;
pub mod response;
pub mod tables;
pub mod types;
pub mod utils;
pub mod request;
pub mod i18n;

rust_i18n::i18n!("locales");

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
