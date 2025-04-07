use std::process::id;


use axum::{
    extract::{Path,State}, routing::{delete, get, post}, Extension, Router
};

use crate::{handlers::{create_user, delete_user,get_single_user_by_id,get_users, update_user, root},types::User};



pub async fn create_routes(db: Vec<User>) -> Result<Router, Box<dyn std::error::Error>> {
    let router = Router::new()
        .route("/", get(root))
        .route("/user",get(create_user))
        .route("/user:id",delete(delete_user))
        .route("/users/:id",post(update_user))
        .route("/users",get(get_users))
        .route("/user/:id",get(get_single_user_by_id))
        .layer(Extension(db));

    Ok(router)
}


