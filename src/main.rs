#[macro_use] extern crate nickel;

use nickel::Nickel;



fn main() {
    let mut server = Nickel::new();

    server.utilize(router! {
        get "**" => |_req, _res| {
            "Welcome to Rust Rock!  This is a template generator written in Rust."
        }
    });

    server.listen("127.0.0.1:6767");
}