use rocket::serde::json::{Json, Value, json};
use rocket::State;

use crate::model::Kennzeichen;
use crate::repo::KennzdRepo;

#[get("/kennzeichen/<kuerzel>")]
pub fn kennz_by_kuerzel(kuerzel: &str, repo: &State<KennzdRepo>) -> Option<Json<Kennzeichen>> {
    match repo.find(kuerzel) {
        Some(k) => Some(Json(k.clone())),
        None => None
    }
}

#[get("/kennzeichen")]
pub fn kennz_all(repo: &State<KennzdRepo>) -> Json<Vec<Kennzeichen>> {
    Json(repo.find_all())
}

#[catch(404)]
pub fn not_found() -> Value {
    json!({
        "status": "error",
        "reason": "Resource not found."
    })
}
