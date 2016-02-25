extern crate iron;
extern crate router;
extern crate rustc_serialize;
extern crate redis;
extern crate persistent;

use iron::prelude::*;
use iron::status;
use iron::typemap::Key;
use router::Router;
use persistent::Write;
use redis::{Connection,Commands};

pub struct Foo;
impl Key for Foo { type Value = Connection; }

fn get(req: &mut Request) -> IronResult<Response> {
    let mutex = req.get::<Write<Foo>>().unwrap();
    let conn = mutex.lock().unwrap();
    match query(req, "key") {
        Some(k) => match conn.get::<&str, Option<String>>(k.as_ref()) {
            Ok(Some(s)) => (Ok(Response::with((status::Ok, s)))),
            Ok(None) => Ok(Response::with((status::NotFound, "not found"))),
            Err(_) => Ok(Response::with((status::InternalServerError, "operation error"))),
        },
        None => Ok(Response::with((status::BadRequest, "missing param 'key'"))),
    }
}

fn main() {
    let client = redis::Client::open("redis://127.0.0.1/").expect("invalid url");
    let conn = Write::<Foo>::one(client.get_connection().expect("could not open connection"));
    let mut router = Router::new();
    router.get("/get", get);

    let mut chain = Chain::new(router);
    chain.link_before(conn);

    Iron::new(chain).http("localhost:8000").unwrap();
}

fn query(request: &Request, key: &str) -> Option<String> {
    match request.url.clone().into_generic_url().query_pairs() {
        Some(pairs) => {
            for (k, v) in pairs {
                if key == k { return Some(v) }
            }
            return None
        }
        None => None
    }
}
