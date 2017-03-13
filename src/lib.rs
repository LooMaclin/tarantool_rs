#![feature(custom_derive)]
extern crate rmpv;
extern crate rmp_serde;
extern crate serde;

#[macro_use]
extern crate rmp;

extern crate base64;
extern crate sha1;
extern crate rmp_serialize;
extern crate rustc_serialize;
extern crate byteorder;
extern crate hex_slice;

#[macro_use]
extern crate log;

pub mod tarantool;
pub mod code;
pub mod greeting_packet;
pub mod greeting_packet_parameters;
pub mod upsert_operation;
pub mod integer_operation;
pub mod string_operation;
pub mod common_operation;
pub mod request_type_key;
pub mod iterator_type;
pub mod header;
pub mod response;
pub mod select;
pub mod insert;
pub mod upsert;
pub mod update_integer;
pub mod update_common;
pub mod update_string;
pub mod eval;
pub mod call;
pub mod delete;
pub mod action;
pub mod replace;

pub use rmpv::Value;
pub use tarantool::Tarantool;
pub use select::{Select};
pub use insert::{Insert};
pub use update_common::UpdateCommon;
pub use update_string::UpdateString;
pub use update_integer::UpdateInteger;
pub use replace::Replace;
pub use eval::Eval;
pub use delete::Delete;
pub use call::Call;
pub use iterator_type::IteratorType;
pub use common_operation::CommonOperation;
pub use integer_operation::IntegerOperation;


pub const FIX_STR_PREFIX: u8 = 0xA1;
