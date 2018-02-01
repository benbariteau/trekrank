extern crate serde_json;

#[macro_use] extern crate askama;
#[macro_use] extern crate iron;
#[macro_use] extern crate serde_derive;

use askama::Template;
use iron::Iron;
use iron::IronResult;
use iron::headers::ContentType;
use iron::middleware::Chain;
use iron::request::Request;
use iron::response::Response;
use iron::status;
use std::fs::File;

#[derive(Serialize, Deserialize)]
struct Episode {
    season: i8,
    title: String,
    link: String,
    episode_num: String,
    description: String,
    series: String,
}

#[derive(Template)]
#[template(path="app.tmpl.html")]
struct App {
    episodes: Vec<Episode>,
}


fn app(_: &mut Request) -> IronResult<Response> {
    let file = itry!(File::open("star_trek_rank.json"));
    let episodes: Vec<Episode> = itry!(serde_json::from_reader(file));

    let mut response = Response::with((
        status::Ok,
        itry!(App{episodes: episodes}.render())
    ));
    response.headers.set(ContentType::html());
    Ok(response)
}

fn main() {
    let chain = Chain::new(app);
    Iron::new(chain).http("0.0.0.0:3000").unwrap();
}
