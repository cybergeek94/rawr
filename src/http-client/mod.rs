use url::Url;
use std::collections::HashMap;

#[cfg(not(teepee))]
pub use self::rust-http::Client as Client;

#[cfg(teepee)]
pub use self::teepee::Client as Client;

mod rust-http;
mod teepee;

pub trait HttpClient {
    /// Make a GET request, returning a string response
    fn get(&self, url: &Url, params: HashMap<String, String>) -> String;

    /// Make a POST request, returning a string response and storing any cookies
    fn post(&mut self, url: &Url, params: HashMap<String, String>, modhash: &str) -> String;
}
