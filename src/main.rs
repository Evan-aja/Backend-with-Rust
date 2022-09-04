use actix_web::{middleware::Logger,*};
use migration::{Migrator, MigratorTrait};
use std::io::Result;

mod controller;
mod database;
use controller::*;

#[derive(Debug, Clone)]
pub struct AppState {
    db: sea_orm::DatabaseConnection,
}

#[actix_web::main]
async fn main() -> Result<()>{
    println!("Hello, world!");
    dotenv::dotenv().ok();

    let db=database::database::run().await.to_owned();
    if let Err(e)=db{
        println!("Error:{}",e);
        return Ok(());
    }
    Migrator::up(&db.clone().unwrap(), None).await.unwrap();
    
    let state=AppState{db: db.unwrap()};
    
    HttpServer::new(move ||{
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(state.clone()))
            .service(cars::getcar)
    })
    .bind(("0.0.0.0",8000))?
    .bind(("::1",8000))?
    .run()
    .await
}
