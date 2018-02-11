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
use std::env;

#[derive(Serialize, Deserialize)]
struct Episode {
    season: u8,
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

struct SeasonPresenter {
    number: String,
    display: String,
    selected: bool,
}

struct Series<'a> {
    value: &'a str,
    name: &'a str,
}

struct SeriesPresenter<'a> {
    series: Series<'a>,
    selected: bool,
}

#[derive(Template)]
#[template(path="app.tmpl.html")]
struct App<'a> {
    episodes: Vec<RankedEpisode>,
    show_description: bool,
    seasons: Vec<SeasonPresenter>,
    show_rank: bool,
    series_list: Vec<SeriesPresenter<'a>>,
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

    let season_filter: Option<u8> = params.find(&["season"]).and_then(|ref value| {
        match value {
            &&Value::String(ref string) => string.parse().ok(),
            _ => None,
        }
    });

    let series_filter: Option<String> = params.find(&["series"]).and_then(|ref value| {
        match value {
            &&Value::String(ref string) => {
                let string = string.clone();
                if vec!["TNG", "DS9", "Voyager"].contains(&string.as_str()) {
                    Some(string)
                } else {
                    None
                }
            },
            _ => None,
        }
    });

    let file = itry!(File::open("star_trek_rank.json"));
    let episodes: Vec<Episode> = itry!(serde_json::from_reader(file));
    let episodes: Vec<RankedEpisode> = episodes.into_iter().enumerate().map(
        |(rank, episode)| RankedEpisode{
            rank: (rank + 1) as u16,
            episode: episode,
        }
    ).collect();
    let episodes: Vec<RankedEpisode> = episodes.into_iter().filter(
        |episode| season_filter.map_or(
            true,
            |season| episode.episode.season == season,
        )
    ).filter(
        |episode| series_filter.clone().map_or(
            true,
            |series| episode.episode.series == series,
        ),
    ).collect();

    let series_list: Vec<SeriesPresenter> = vec![
        Series{value: "", name: "All Series"},
        Series{value: "TNG", name: "The Next Generation"},
        Series{value: "DS9", name: "Deep Space 9"},
        Series{value: "Voyager", name: "Voyager"},
    ].into_iter().map(|thing| {
        let value = thing.value.clone();
        SeriesPresenter{
            series: thing,
            selected: series_filter.clone().map_or(
                false,
                |inner_series| inner_series == value,
            ),
        }
    }).collect();

    let seasons = vec![SeasonPresenter{
        number: "".to_string(),
        display: "All Seasons".to_string(),
        selected: season_filter.is_none(),
    }].into_iter().chain(
        (1..7).map(
            |num| SeasonPresenter{
                number: num.to_string(),
                display: format!("Season {}", num),
                selected: if let Some(season) = season_filter {
                    season == num
                } else { false }
            }
        ),
    ).collect();

    let show_rank = season_filter.is_some() || series_filter.is_some();

    let mut response = Response::with((
        status::Ok,
        itry!(App{
            episodes: episodes,
            show_description: show_description,
            seasons: seasons,
            show_rank: show_rank,
            series_list: series_list,
        }.render())
    ));
    response.headers.set(ContentType::html());
    Ok(response)
}

fn main() {
    let chain = Chain::new(app);
    let port: u16 = env::var("PORT").ok().and_then(
        |port| port.parse().ok()
    ).unwrap_or(3000);
    Iron::new(chain).http(format!("0.0.0.0:{}", port)).unwrap();
}
