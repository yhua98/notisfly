use axum::{extract::State, http::{HeaderMap, StatusCode}, Json};
use serde::{Deserialize, Serialize};

use crate::{api::{auth::Claims, types::ResponsePayload}, store, AppState};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    name: String,
    email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserLogin{
    email: String,
    name: String,
    token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestUserLogin {
    email: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestUserRegister {
    name: String,
    email: String,
    password: String,
}

pub async fn login(
    State(app_state): State<AppState>,
    Json(req_user): Json<RequestUserLogin>,
) -> (StatusCode, HeaderMap, Json<ResponsePayload<UserLogin>>) {

    let mut headers = HeaderMap::new();

    let result = store::user::get_user_by_emial(req_user.email, &app_state.db).await;

    if let Err(err) = result {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            headers,
            Json(ResponsePayload {
                status: 500,
                message: err.to_string(),
                data: None,
            }),
        );
    }

    let user = result.unwrap();

    if user.password != req_user.password {
        return (
            StatusCode::BAD_REQUEST,
            headers,
            Json(ResponsePayload {
                status: 400,
                message: "Invalid password".to_string(),
                data: None,
            }),
        );
    }

    let result = crate::api::auth::get_token(crate::api::auth::Claims::new(
        "notfly@qq.com".to_string(),
        chrono::Duration::days(10).num_milliseconds() + chrono::Utc::now().timestamp(),
        user.id.to_string(),
        user.name.clone(),
        user.email.clone(),
    ))
    .await;

    if let Err(err) = result {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            headers,
            Json(ResponsePayload {
                status: 500,
                message: err.to_string(),
                data: None,
            }),
        );
    }

    let token = result.unwrap();
    headers.insert("Authorization", format!("bearer {token}").parse().unwrap());

    (
        StatusCode::OK,
        headers,
        Json(ResponsePayload {
            status: 200,
            message: "success".to_string(),
            data: Some(UserLogin {
                name: user.name,
                email: user.email,
                token,
            }),
        }),
    )
}


pub async fn register(
    State(app_state): State<AppState>,
    Json(user_create): Json<RequestUserRegister>,
) -> (StatusCode, Json<ResponsePayload<User>>) {
    // check if user already exists
    let result = store::user::get_user_by_emial(user_create.email.clone(), &app_state.db).await;
    if let Ok(user) = result  {
        return (
            StatusCode::BAD_REQUEST,
            Json(ResponsePayload {
                status: 400,
                message: "User already exists".to_string(),
                data: Some(User {
                    name: user.name,
                    email: user.email,
                }),
            }),
        );
    }

    let result = store::user::create_user(
        store::user::UserCreate {
            name: user_create.name.clone(),
            password: user_create.password.clone(),
            email: user_create.email.clone(),
        },
        &app_state.db,
    ).await;

    if let Err(err) = result {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ResponsePayload{
                status: 500,
                message: err.to_string(),
                data: None
            })
        )
    }

    let user = result.unwrap();

    let result = crate::api::auth::get_token(Claims::new(
        "notlfy@qq.com".to_string(),
        3600,
        user.id.clone().to_string(),
        user.name.clone(),
        user.email.clone())).await;

    if let Err(err) = result {
        return  (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ResponsePayload{
                status: 500,
                message: err.to_string(),
                data: None
            })
        );
    }

    (StatusCode::CREATED, Json(ResponsePayload{
        status: 200,
        message: "create user success".to_string(),
        data: Some(User{
            name: user.name.clone(),
            email: user.email.clone()
        })
    }))
}