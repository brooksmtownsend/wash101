extern crate wapc_guest as guest;
use wasmcloud_actor_core as actor;
use wasmcloud_actor_http_server as http;

use guest::prelude::*;

#[actor::init]
fn init() {
    // Register your message handlers here
    http::Handlers::register_handle_request(handle_ping);
}

fn handle_ping(req: http::Request) -> HandlerResult<http::Response> {
    let msg: String = String::from_utf8(req.body)?;
    if msg == "ping" {
        Ok(http::Response::json("pong", 200, "OK"))
    } else {
        Ok(http::Response::bad_request())
    }
}
