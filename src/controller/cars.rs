use actix_web::{get, Responder, web::{self}, HttpResponse};

use serde_json::json;

use crate::AppState;

#[get("/cars")]
pub async fn getcar(state:web::Data<AppState>) -> impl Responder{
    let _db=&state.db;
    HttpResponse::Ok().json(
        json!({
            "asd":"qwe"
        }
    ))
}