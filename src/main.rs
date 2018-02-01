extern crate iron;

#[macro_use] extern crate askama;

use iron::middleware::Chain;
use iron::Iron;
use iron::status;
use iron::response::Response;
use iron::IronResult;
use iron::request::Request;
use askama::Template;


fn app(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Hello world")))
}

fn main() {
    let chain = Chain::new(app);
    Iron::new(chain).http("0.0.0.0:3000").unwrap();
}
