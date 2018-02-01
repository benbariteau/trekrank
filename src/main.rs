extern crate serde_json;

#[macro_use] extern crate iron;
#[macro_use] extern crate askama;
#[macro_use] extern crate serde_derive;

use std::fs::File;
use iron::middleware::Chain;
use iron::Iron;
use iron::status;
use iron::response::Response;
use iron::IronResult;
use iron::request::Request;
use askama::Template;

#[derive(Serialize, Deserialize)]
struct Episode {
    Season: i8,
    Title: String,
    Link: String,
    EpisodeNum: String,
    Description: String,
    Series: String,
}


fn app(_: &mut Request) -> IronResult<Response> {
    let file = itry!(File::open("sorted_eps.json"));
    let episodes: Vec<Episode> = itry!(serde_json::from_reader(file));
    let episodes_json = itry!(serde_json::to_string_pretty(&episodes));
    Ok(Response::with((status::Ok, episodes_json)))
}

fn main() {
    let chain = Chain::new(app);
    Iron::new(chain).http("0.0.0.0:3000").unwrap();
}
