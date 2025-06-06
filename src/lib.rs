//! A minimal (i.e. very incomplete) implementation of a Redis server and
//! client.
//! 
//! Made by following the tutorial at https://tokio.rs/tokio/tutorial, with
//! some modifications to fill in the details excluded from the tutorial.
//! (like this file!)

pub mod connection;
pub use connection::Connection;
