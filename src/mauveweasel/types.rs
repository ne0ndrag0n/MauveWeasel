use tiny_http::{Response};
use std::io::Cursor;

pub type ServerResponse = Response< Cursor < Vec < u8 > > >;
