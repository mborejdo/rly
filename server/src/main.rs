#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
#[macro_use] extern crate serde_derive;

use clap::Clap;

#[derive(Clap)]
#[clap(version = "1.0", author = "Michael Borejdo <mib@electronic-minds.de>")]
struct Opts {
    #[clap(short, long)]
    database: String
}

use rocket::{
    response::Redirect,
    request::Form
};
use rocket_contrib::serve::StaticFiles;

mod db;
mod schema;
mod rlyurl;
use rlyurl::RlyUrl;

#[derive(FromForm, Debug)]
struct Url {
    url: String,
}

#[get("/help")]
fn help() -> &'static str {
    "
      POST /
          Ex: curl --data \"url=https://user.lan\" https://rly.eminds.de/
          Antwort: https://rly.eminds.de/id/gY

      GET /id/<id>
          Redirect zu rly-short-link. 
          Ex: curl -i https://rly.eminds.de/id/gY
    "
}

#[get("/id/<id>")]
fn lookup(id: String, connection: db::Connection) -> Result<Redirect, &'static str> {
    let rlyurl = RlyUrl::find(id.to_string(), &connection);
    match rlyurl {
        Ok(u) => Ok(Redirect::permanent(format!("{}", u.url))),
        _ => Err("ID nicht gefunden"),
    }
}

#[get("/warmup")]
fn warmup( connection: db::Connection) -> Result<String, &'static str> {
    let insert = RlyUrl { id: None, key: "0".to_string(), url: "http://localhost".to_string()};
    RlyUrl::create(insert, &connection);

    Ok("OK".to_string())
}

#[post("/", data = "<url_form>")]
fn shorten(url_form: Form<Url>, connection: db::Connection) -> Result<String, String> {
    let ref url = format!("{}", url_form.into_inner().url);
    if !url.starts_with("https") && !url.starts_with("http") {
        return Err(format!("Keine valide URL {:?}", url));
    }

    let id = RlyUrl::generate_id(&connection);
    let insert = RlyUrl { id: None, key: id.to_string(), url: url.to_string()};
    RlyUrl::create(insert, &connection);

    Ok(format!("https://rly.eminds.de/id/{}", id))
}

fn main() {
    let opts: Opts = Opts::parse();

    rocket::ignite()
        .manage(db::connect(opts.database.clone()))
        .mount("/", routes![lookup, help, warmup, shorten])
        .mount("/", StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../gui/static")).rank(0))
        .launch();
}