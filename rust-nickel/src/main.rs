#[macro_use] extern crate nickel;
extern crate rustc_serialize;
extern crate redis;

use std::sync::{Arc, Mutex};
use nickel::{Nickel,HttpRouter,QueryString};
use nickel::status::StatusCode;
use redis::Commands;

fn main() {
    let mut server = Nickel::new();
    let client = redis::Client::open("redis://127.0.0.1/").expect("invalid url");
    let shared_conn = Arc::new(Mutex::new(client.get_connection().expect("could not open connection")));

    let conn_lock = shared_conn.clone();
    server.get("/get", middleware! { |req, mut resp|
        let conn = conn_lock.lock().unwrap();
        match req.query().get("key") {
            Some(k) => match conn.get::<&str, Option<String>>(k) {
                Ok(Some(s)) => s,
                Ok(None) => {
                    resp.set(StatusCode::NotFound);
                    String::from("not found")
                }
                Err(_) => {
                    resp.set(StatusCode::InternalServerError);
                    String::from("operation error")
                },
            },
            None => {
                resp.set(StatusCode::BadRequest);
                String::from("missing param 'key'")
            }
        }
    });

    server.listen("127.0.0.1:8000");
}
