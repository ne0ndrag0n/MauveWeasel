use tiny_http::{Request, Response};
use std::io::Cursor;

pub trait Handler {
    fn handle( self, request: &mut Request ) -> Response< Cursor < Vec < u8 > > >;
}
