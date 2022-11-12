use actix_web::web;
use sqlx::{sqlite::{SqlitePool, SqliteRow}, Error, query, Row};
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use std::env;
use md5;


#[derive(Debug, Serialize, Deserialize, Clone)]
struct User {
    pub id: i64,
    pub username: String,
    pub hashed_password: String,
    pub role: String,
    pub token: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct NewUser<'a> {
    pub username: &'a str,
    pub password: &'a str,
}

impl User{
    fn from_row(row: SqliteRow) -> User{
        User {
            id: row.get("id"),
            username: row.get("username"),
            hashed_password: row.get("hashed_password"),
            role: row.get("role"),
            token: row.get("token"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }
    }
    async fn next_id(pool: &web::Data<SqlitePool>) -> Result<i64, Error>{
        let sql = "SELECT MAX(id) max_id FROM users";
        query(sql)
            .map(|row: SqliteRow| row.get("max_id"))
            .fetch_one(pool.get_ref())
            .await
    }

    fn wrap(word: &str) -> String{
        let pepper = env::var("PEPPER").unwrap_or("pepper".to_string());
        let salt = env::var("SALT").unwrap_or("salt".to_string());
        let composition = format!("{}{}{}", pepper, word, salt);
        format!("{:x}", md5::compute(composition))
    }

    pub async fn create(pool: &web::Data<SqlitePool>, role: &str, new: &NewUser<'_>) -> Result<User, Error>{
        let hashed_password = Self::wrap(&new.password);
        let next_id = Self::next_id(&pool).await.unwrap();
        let token = Self::wrap(&next_id.to_string());
        let created_at = Utc::now();
        let updated_at = Utc::now();
        let sql = "INSERT INTO users (username, hashed_password, role, token,
            created_at, updated_at) VALUES($1, $2, $3, $4, $5, $6) 
            RETURNING id, username, hashed_password, role, token, created_at,
            updated_at;";
        query(sql)
            .bind(&new.username)
            .bind(&hashed_password)
            .bind(role)
            .bind(&token)
            .bind(&created_at)
            .bind(&updated_at)
            .map(Self::from_row)
            .fetch_one(pool.get_ref())
            .await
    }

    pub async fn read(pool: &web::Data<SqlitePool>, token: &str) -> Result<User, Error>{
        let sql = "SELECT * FROM users WHERE token = $1";
        query(sql)
            .bind(token)
            .map(Self::from_row)
            .fetch_one(pool.get_ref())
            .await
    }

    pub async fn delete(pool: &web::Data<SqlitePool>, username: &str) -> Result<User, Error>{
        let sql = "DELETE FROM users WHERE username = $1 RETURNING id,
                   username, hashed_password, role, token, created_at,
                   updated_at;";
        query(sql)
            .bind(username)
            .map(Self::from_row)
            .fetch_one(pool.get_ref())
            .await
    }
}
