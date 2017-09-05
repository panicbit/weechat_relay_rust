#![allow(warnings)]
#![feature(proc_macro, conservative_impl_trait, generators, try_from)]

#[macro_use]
extern crate error_chain;
extern crate futures_await as futures;
extern crate tokio_core;
extern crate tokio_io;
extern crate byteorder;
extern crate libflate;
extern crate typemap;
extern crate hexdump;

mod errors;
mod raw;
mod command;
mod message;
mod message_resolver;
mod object;
pub mod client;

pub use object::Object;

pub use client::Client;

pub use errors::*;
