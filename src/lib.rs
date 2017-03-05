#![feature(custom_derive)]
extern crate rmpv;
extern crate rmp_serde;
extern crate serde;

#[macro_use]
extern crate derive_builder;

#[macro_use]
extern crate rmp;

extern crate base64;
extern crate sha1;
extern crate rmp_serialize;
extern crate rustc_serialize;
extern crate byteorder;
extern crate hex_slice;

extern crate futures;
extern crate tokio_core;
extern crate tokio_proto;
extern crate tokio_service;

#[macro_use]
extern crate log;

pub mod async_client;
pub mod sync_client;
pub mod client;
pub mod tarantool;
pub mod code;
pub mod greeting_packet;
pub mod greeting_packet_parameters;
pub mod operation;
pub mod request_type_key;
pub mod iterator_type;
pub mod header;
pub mod response;
mod select;

pub use rmpv::Value;
