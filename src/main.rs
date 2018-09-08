#![allow(non_snake_case)]

#[macro_use] extern crate serde_derive;
extern crate toml;

mod mauveweasel;

use mauveweasel::server::MauveWeaselServer;

fn main() {
    println!( "MauveWeasel Dynamic Content Engine" );
    let server: MauveWeaselServer = MauveWeaselServer::new();
    server.run();
}
