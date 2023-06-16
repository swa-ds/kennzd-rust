#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod model;
mod repo;
mod ui;
mod api;

use rocket_dyn_templates::Template;

use repo::KennzdRepo;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(KennzdRepo::create())
        .attach(Template::fairing())
        .mount("/", routes![ui::index, ui::ui])
        .mount("/api", routes![api::kennz_by_kuerzel, api::kennz_all])
        .register("/api", catchers![api::not_found])
}
