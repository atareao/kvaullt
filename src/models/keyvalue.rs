
use actix_web::web;
use sqlx::{sqlite::{SqlitePool, SqliteRow}, Error, query, Row};
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KeyValue {
    pub id: i64,
    pub key: String,
    pub value: String,
    pub user_id: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewKeyValue {
    pub key: String,
    pub value: String,
}

impl KeyValue{
    fn from_row(row: SqliteRow) -> KeyValue{
        KeyValue {
            id: row.get("id"),
            key: row.get("key"),
            value: row.get("value"),
            user_id: row.get("user_id"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }
    }

    pub async fn create(pool: &web::Data<SqlitePool>, user_id: i64, new: &NewKeyValue) -> Result<KeyValue, Error>{
        let created_at = Utc::now();
        let updated_at = Utc::now();
        let sql = "INSERT INTO keyvalues (key, value, user_id, created_at,
                   updated_at) VALUES($1, $2, $3, $4, $5) 
                   RETURNING id, key, value, user_id, created_at, updated_at;";
        query(sql)
            .bind(&new.key)
            .bind(&new.value)
            .bind(user_id)
            .bind(&created_at)
            .bind(&updated_at)
            .map(Self::from_row)
            .fetch_one(pool.get_ref())
            .await
    }

    pub async fn read(pool: &web::Data<SqlitePool>, user_id: i64, key: &str) -> Result<KeyValue, Error>{
        let sql = "SELECT * FROM keyvalues WHERE user_id = $1 AND key = $2";
        query(sql)
            .bind(user_id)
            .bind(key)
            .map(Self::from_row)
            .fetch_one(pool.get_ref())
            .await
    }

    pub async fn update(pool: &web::Data<SqlitePool>, user_id: i64, new: &NewKeyValue) -> Result<KeyValue, Error>{
        let updated_at = Utc::now();
        let sql = "UPDATE keyvalues SET value = $1, updated_at = $2 WHERE
                   key = $3 AND user_id = $4 RETURNING id, key, value, user_id,
                   created_at, updated_at;";
        query(sql)
            .bind(&new.value)
            .bind(&updated_at)
            .bind(&new.key)
            .bind(user_id)
            .map(Self::from_row)
            .fetch_one(pool.get_ref())
            .await
    }

    pub async fn delete(pool: &web::Data<SqlitePool>, user_id: i64, key: &str) -> Result<KeyValue, Error>{
        let sql = "DELETE FROM keyvalues WHERE key = $1 AND user_id = $2
                   RETURNING id, key, value, user_id, token, created_at,
                   updated_at;";
        query(sql)
            .bind(key)
            .bind(user_id)
            .map(Self::from_row)
            .fetch_one(pool.get_ref())
            .await
    }
}
