#[macro_use]
extern crate log;
extern crate tarantool;
use tarantool::utils::read_length;

fn main() {
    debug!("length: {}",
             read_length(&mut &[0xCE, 0x0, 0x0, 0x0, 0x4B][..]));
}
