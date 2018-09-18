#![allow(non_snake_case)]

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;
extern crate bincode;
extern crate toml;
extern crate uuid;
extern crate serde_urlencoded;
extern crate handlebars;

mod mauveweasel;
use mauveweasel::server::DynamicContentServer;

fn main() {
    println!( "MauveWeasel Dynamic Content Engine" );
    let server: DynamicContentServer = DynamicContentServer::new();
    server.run();
}
