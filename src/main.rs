mod models;
mod routes;

use actix_web::{web::Data, App, HttpServer, middleware::Logger};
use dotenv::dotenv;
use std::{env, path::Path, process};
use env_logger::Env;
use sqlx::{sqlite::SqlitePoolOptions, migrate::{Migrator, MigrateDatabase}};
use crate::models::user::{User, NewUser, Role};
use log::{debug, error, info};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let port = env::var("PORT").expect("PORT not set");
    let debug_level = env::var("DEBUG_LEVEL").unwrap_or("info".to_string());
    let username = env::var("USERNAME").expect("USERNAME not set");
    let password = env::var("PASSWORD").expect("PASSWORD not set");
    env_logger::init_from_env(Env::default().default_filter_or(debug_level));

    if sqlx::Sqlite::database_exists(&db_url).await.unwrap(){
        info!("The database exists");
    }else{
        info!("The database not exists. Creating database");
        sqlx::Sqlite::create_database(&db_url).await.unwrap();
        info!("Database creted");
    }

    let migrations = if env::var("RUST_ENV") == Ok("production".to_string()){
        std::env::current_exe().unwrap().parent().unwrap().join("migrations")
    }else{
        let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        Path::new(&crate_dir).join("./migrations")
    };

    info!("{}", &migrations.display());

    let pool = SqlitePoolOptions::new()
        .max_connections(4)
        .connect(&db_url)
        .await
        .expect("Pool failed");

    info!("Doing migrations");
    Migrator::new(migrations)
        .await
        .unwrap()
        .run(&pool)
        .await
        .unwrap();
    info!("Migrations done");

    let data_pool = Data::new(pool.clone());
    if !User::exists_admin(&data_pool).await{
        let role = Role::Admin.to_string();
        let new = NewUser {username, password};
        match User::create(&data_pool, &role, &new).await{
            Ok(_) => {
                info!("Created admin user");
            },
            Err(_) => {
                error!("Can not create admin user");
                process::exit(1);
            }
        };
    }else{
        info!("The admin user exists");
    }

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(Data::new(pool.clone()))
            .service(routes::user::create)
            //.service(routes::user::read)
            .service(routes::user::read_one)
            .service(routes::user::delete)
            .service(routes::keyvalue::create)
            .service(routes::keyvalue::read)
            .service(routes::keyvalue::update)
            .service(routes::keyvalue::delete)
    })
    .workers(4)
    .bind(format!("0.0.0.0:{}", &port))
    .unwrap()
    .run()
    .await
}
