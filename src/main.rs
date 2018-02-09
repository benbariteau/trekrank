extern crate serde_json;
extern crate params;

#[macro_use] extern crate askama;
#[macro_use] extern crate iron;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate error_chain;

use askama::Template;
use iron::Iron;
use iron::IronResult;
use iron::headers::ContentType;
use iron::middleware::Chain;
use iron::request::Request;
use iron::response::Response;
use iron::status;
use std::fs::File;
use params::Params;
use params::Value;
use iron::Plugin;

#[derive(Serialize, Deserialize)]
struct Episode {
    season: i8,
    title: String,
    link: String,
    episode_num: String,
    description: String,
    series: String,
}

struct RankedEpisode {
    rank: u16,
    episode: Episode,
}

#[derive(Template)]
#[template(path="app.tmpl.html")]
struct App {
    episodes: Vec<RankedEpisode>,
}

mod error {
    error_chain!{}
}


fn app(req: &mut Request) -> IronResult<Response> {
    let params = req.get::<Params>().unwrap();

    let show_description = itry!(params.find(&["description"]).map_or_else(
        || Ok(false),
        |ref value| -> Result<bool, error::Error> {
            match value {
                &&Value::String(ref string) => {
                    if string == "show" {
                        Ok(true)
                    } else {
                        Err("invalid value for description".into())
                    }
                }
                _ => Err("invalid type for description".into())
            }
        },
    ));

    let file = itry!(File::open("star_trek_rank.json"));
    let episodes: Vec<Episode> = itry!(serde_json::from_reader(file));
    let ranked_episodes: Vec<RankedEpisode> = episodes.into_iter().enumerate().map(|(rank, episode)| RankedEpisode{rank: rank as u16, episode: episode}).collect();

    let mut response = Response::with((
        status::Ok,
        itry!(App{episodes: ranked_episodes}.render())
    ));
    response.headers.set(ContentType::html());
    Ok(response)
}

fn main() {
    let chain = Chain::new(app);
    Iron::new(chain).http("0.0.0.0:3000").unwrap();
}
