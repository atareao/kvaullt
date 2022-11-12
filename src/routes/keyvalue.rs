use actix_web::{post, get, put, delete, web, HttpResponse, HttpRequest,
                http::header::ContentType, Responder};
use serde_json::json;
use sqlx::SqlitePool;
use crate::models::{user::User, keyvalue::{KeyValue, NewKeyValue}};

#[post("/v1/kv")]
pub async fn create(req: HttpRequest, pool: web::Data<SqlitePool>, new: web::Json<NewKeyValue>) -> impl Responder{
    match User::from_request(&req, &pool).await {
        Some(user) => match KeyValue::create(&pool, user.id, &new.into_inner()).await {
                    Ok(kv) => HttpResponse::Created()
                        .content_type(ContentType::json())
                        .body(serde_json::to_string(&kv).unwrap()),
                    Err(_) => HttpResponse::UnprocessableEntity().finish(),
        },
        None => HttpResponse::Forbidden().finish(),
    }
}

#[get("/v1/kv")]
pub async fn read(req: HttpRequest, pool: web::Data<SqlitePool>, key: String) -> impl Responder{
    match User::from_request(&req, &pool).await {
        Some(user) => match KeyValue::read(&pool, user.id, &key).await {
            Ok(kv) => HttpResponse::Ok()
                .content_type(ContentType::json())
                .body(json!({"value": kv.value}).to_string()),
            Err(_) => HttpResponse::NotFound().finish(),
        },
        None => HttpResponse::Forbidden().finish()
    }
}

#[put("/v1/kv")]
pub async fn update(req: HttpRequest, pool: web::Data<SqlitePool>, new: web::Json<NewKeyValue>) -> impl Responder{
    match User::from_request(&req, &pool).await {
        Some(user) => match KeyValue::update(&pool, user.id, &new.into_inner()).await {
                    Ok(kv) => HttpResponse::Ok()
                        .content_type(ContentType::json())
                        .body(serde_json::to_string(&kv).unwrap()),
                    Err(_) => HttpResponse::UnprocessableEntity().finish(),
        },
        None => HttpResponse::Forbidden().finish(),
    }
}

#[delete("/v1/kv")]
pub async fn delete(req: HttpRequest, pool: web::Data<SqlitePool>, key: String) -> impl Responder{
    match User::from_request(&req, &pool).await {
        Some(user) => match KeyValue::delete(&pool, user.id, &key).await {
                    Ok(kv) => HttpResponse::Ok()
                        .content_type(ContentType::json())
                        .body(serde_json::to_string(&kv).unwrap()),
                    Err(_) => HttpResponse::UnprocessableEntity().finish(),
        },
        None => HttpResponse::Forbidden().finish(),
    }
}

