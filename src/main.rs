#![allow(non_snake_case)]
#![allow(dead_code)]

#[macro_use] extern crate serde_derive;
extern crate lru_cache;
extern crate comrak;
extern crate chrono;
extern crate bincode;
extern crate toml;
extern crate uuid;
extern crate serde;
extern crate serde_urlencoded;
extern crate handlebars;

mod mauveweasel;
use mauveweasel::server::DynamicContentServer;

fn main() {
    println!( "MauveWeasel Dynamic Content Engine" );
    let server: DynamicContentServer = DynamicContentServer::new();
    server.run();
}
