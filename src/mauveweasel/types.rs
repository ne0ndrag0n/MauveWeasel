use tiny_http::{Request, Response,Method};
use std::io::Cursor;

pub type ServerResponse = Response< Cursor < Vec < u8 > > >;
