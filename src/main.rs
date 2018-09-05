#![allow(non_snake_case)]

extern crate hyper;
#[macro_use]
extern crate serde_derive;
extern crate toml;

mod mauveweasel;

use mauveweasel::server::Server;

fn main() {
    println!( "MauveWeasel Dynamic Content Engine" );

    let server: Server = Server::new();
}
