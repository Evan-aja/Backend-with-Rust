use actix_web::{get, Responder, web, HttpResponse, delete, post};

use sea_orm::{EntityTrait, QueryFilter, ColumnTrait, ModelTrait, ActiveModelTrait, IntoActiveModel};
use serde_json::json;
use entity::{cars, cars::Entity as Cars};

use crate::AppState;

#[get("/cars")]
pub async fn getcar(state:web::Data<AppState>) -> impl Responder{
    let db=&state.db;
    let car=Cars::find().all(db).await.unwrap();
    HttpResponse::Ok().json(
        json!({
            "car":car
        }
    ))
}

#[post("/cars")]
pub async fn postcar(state:web::Data<AppState>,form:web::Json<cars::Model>) -> impl Responder{
    let db=&state.db;
    let car=form.clone().into_active_model();
    let add=car.insert(db).await;
    if let Err(e)=add.clone(){
        return HttpResponse::InternalServerError().json(
            json!({
                "message":"Car does not exist",
                "error":e.to_string(),
                "success":false,
            })
        );
    }
    HttpResponse::Ok().json(json!({
        "message":"Car added successfully",
        "error":null,
        "success":true,
        "data":add.unwrap(),
    }))
}

#[delete("/cars")]
pub async fn delcar(state:web::Data<AppState>,form:web::Json<cars::Model>) -> impl Responder{
    let db=&state.db;
    let car=Cars::find()
        .filter(cars::Column::Id.eq(form.id))
        .one(db)
        .await;
    if let Err(e)=car.clone(){
        return HttpResponse::InternalServerError().json(
            json!({
                "message":"Car does not exist",
                "error":e.to_string(),
                "success":false,
            })
        );
    }
    if car.clone().unwrap()==None{
        return HttpResponse::BadRequest().json(
            json!({
                "message":"Car does not exist",
                "error":null,
                "success":false,
            })
        );
    }
    let del=car.clone().unwrap().unwrap().delete(db).await;
    assert_ne!(del.unwrap().rows_affected,0);

    HttpResponse::Ok().json(
        json!({
            "message":"car deleted successfully",
            "error":null,
            "success":true,
            "data":car.unwrap(),
        })
    )
}