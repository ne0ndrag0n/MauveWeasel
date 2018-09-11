use tiny_http::{Request, Response};
use mauveweasel::types::ServerResponse;

pub trait Handler {
    fn handle( self, request: &mut Request ) -> ServerResponse;
}
