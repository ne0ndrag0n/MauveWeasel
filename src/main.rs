#![allow(non_snake_case)]

#[macro_use] extern crate serde_derive;
extern crate toml;
extern crate tiny_http;
extern crate sha3;

mod mauveweasel;
use mauveweasel::server::DynamicContentServer;

fn main() {
    println!( "MauveWeasel Dynamic Content Engine" );
    let server: DynamicContentServer = DynamicContentServer::new();
    server.run();
}
