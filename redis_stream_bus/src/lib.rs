pub mod bus;
pub mod client;
pub mod config;
pub mod error;

#[cfg(test)]
mod client_test;

#[cfg(test)]
mod server;



pub trait StreamParsable : serde::Serialize {
    fn key(&self)-> String;
}