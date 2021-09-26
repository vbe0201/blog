#![feature(decl_macro)]

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_contrib;

mod post;

use std::path::{Path, PathBuf};

use rocket::response::NamedFile;
use rocket_contrib::templates::Template;

#[get("/")]
fn index() -> Template {
    Template::render("index", post::retrieve_posts().unwrap())
}

#[get("/<file..>")]
fn css(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("css/").join(file)).ok()
}

fn main() {
    rocket::ignite()
        .attach(Template::fairing())
        .mount("/", routes![index])
        .mount("/css", routes![css])
        .launch();
}
