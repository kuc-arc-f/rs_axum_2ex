use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use anyhow::{ensure, Context, Result};
use argon2::{Argon2, PasswordHasher, PasswordVerifier};
use argon2::password_hash::{SaltString, PasswordHash, rand_core::OsRng};
use sqlx::PgPool;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::error::Error;
use uuid::Uuid;

use crate::models::{
    CreateTodoRequest, Todo ,
    UserRes, UserCreateReq , DbUser
};

pub async fn user_login(
    State(pool): State<PgPool>,
    Json(payload): Json<UserCreateReq>,
) -> Result<StatusCode, String> {
    let mut user = UserRes {
        email: "".to_string(),
        password: "".to_string(),
    };

    println!("{:?}", payload);
    let ret: Vec<Todo> = Vec::new();
    let email = payload.email.trim();
    if email.is_empty() {
        return Ok(StatusCode::BAD_REQUEST);
    }
    let password = payload.password.trim();
    if password.is_empty() {
        return Ok(StatusCode::BAD_REQUEST);
    }
    println!("email={}", email);
    println!("password={}", password);

    let users: Vec<DbUser> = sqlx::query_as::<_, DbUser>(
        r#"
        SELECT email, password_hash
        FROM users
        WHERE email = $1
        "#
    )
    .bind(email)
    .fetch_all(&pool)
    .await.unwrap();

    let mut user_count = 0;
    let mut db_pass = "".to_string();
    for user in users {
        db_pass = user.password_hash.clone();
        println!("email: {}", user.email);
        println!("password_hash: {}", db_pass);
        user_count += 1;
    }    
    println!("user_count={}", user_count);
    if user_count == 0 {
        println!("error , user_count == 0");
        return Ok(StatusCode::BAD_REQUEST);
    }
    let pw = payload.password.clone();
    let phc = db_pass.clone();    
    let ok = super::mod_auth::verify_password(&pw, &db_pass).expect("error , veryfy password");
    println!("password ok? {}", ok);    
    if ok == false {
        return Ok(StatusCode::BAD_REQUEST);
    }    
    Ok(StatusCode::CREATED)
}

pub async fn user_create(
    State(pool): State<PgPool>,
    Json(payload): Json<UserCreateReq>,
) -> Result<StatusCode, String> {
    let mut user = UserRes {
        email: "".to_string(),
        password: "".to_string(),
    };

    println!("{:?}", payload);
    let ret: Vec<Todo> = Vec::new();
    let email = payload.email.trim();
    if email.is_empty() {
        return Ok(StatusCode::BAD_REQUEST);
    }
    let password = payload.password.trim();
    if password.is_empty() {
        return Ok(StatusCode::BAD_REQUEST);
    }
    println!("email={}", email);
    println!("password={}", password);

    let users: Vec<DbUser> = sqlx::query_as::<_, DbUser>(
        r#"
        SELECT email, password_hash
        FROM users
        WHERE email = $1
        "#
    )
    .bind(email)
    .fetch_all(&pool)
    .await.unwrap();

    let mut user_count = 0;
    for user in users {
        println!("email: {}", user.email);
        user_count += 1;
    }    
    println!("user_count={}", user_count);
    if user_count > 0 {
        println!("error , user_count > 0");
        return Ok(StatusCode::BAD_REQUEST);
    }
    let pw = payload.password.clone();
    let phc = super::mod_auth::hash_password(&pw).expect("error , hash_password");
    println!("phc: {}", phc);
    
    let result = sqlx::query(
        r#"
        INSERT INTO users (email, password_hash)
        VALUES ($1, $2)
        RETURNING id, email
        "#,
    )
    .bind(email)
    .bind(phc)
    .execute(&pool)
    .await.unwrap();    
    Ok(StatusCode::CREATED)
}
