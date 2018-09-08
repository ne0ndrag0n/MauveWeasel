#![allow(non_snake_case)]

#[macro_use] extern crate serde_derive;
extern crate toml;

mod mauveweasel;
use mauveweasel::server;

fn main() {
    println!( "MauveWeasel Dynamic Content Engine" );
    server::run();
}
