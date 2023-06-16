
use rocket_dyn_templates::{Template, context};
use rocket::State;

use crate::model::Kennzeichen;
use crate::repo::KennzdRepo;

#[get("/")]
pub fn index() -> Template {
    Template::render("index", context! {})
}

#[get("/ui?<kuerzel>")]
pub fn ui(kuerzel: Option<String>, repo: &State<KennzdRepo>) -> Template {
    match kuerzel {
        None => render(&Kennzeichen::empty()),
        Some(k) => {
            match repo.find(&k.to_uppercase()) {
                Some(kennz) => render(kennz),
                None => render(&Kennzeichen::n_a(k))
            }
        }
    }
}

fn render(kennz: &Kennzeichen) -> Template {
    Template::render("ui", context! {
        kennzeichen: kennz
    })
}
