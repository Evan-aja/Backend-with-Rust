use actix_web::{get, Responder, web, HttpResponse, delete, post, patch};

use sea_orm::{EntityTrait, QueryFilter, ColumnTrait, ModelTrait, ActiveModelTrait, IntoActiveModel, Set};
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

#[patch("/cars")]
pub async fn patchcar(state:web::Data<AppState>,form:web::Json<cars::Model>) -> impl Responder{
    let db=&state.db;
    let queer=Cars::find_by_id(form.clone().id).one(db).await.unwrap();
    let mut patch:cars::ActiveModel = queer.unwrap().into();
    if form.clone().name.to_owned()!="".to_string(){
        patch.name=Set(form.clone().name.to_owned());
    }
    if form.clone().brand!="".to_string(){
        patch.brand=Set(form.clone().brand);   
    }
    
    let res=patch.update(db).await;

    if let Err(e)=res{
        return HttpResponse::InternalServerError().json(json!({
            "message":"Server has envountered an error",
            "error":e.to_string(),
            "success": false,
        }))
    }
    HttpResponse::Ok().json(json!({
        "message":"data updated successfully",
        "error":null,
        "success":true,
        "data":res.unwrap()
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