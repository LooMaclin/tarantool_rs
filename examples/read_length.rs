extern crate tarantool;
use tarantool::utils::read_length;

fn main() {
    println!("length: {}",
             read_length(&mut &[0xCE, 0x0, 0x0, 0x0, 0x4B][..]));
}
