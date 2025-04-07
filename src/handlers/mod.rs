use std::{any::type_name, fs::remove_dir};

use axum::{extract::Path, response::IntoResponse, Json};
use serde_json::json;


use crate::types::User;

static mut USERS:Vec<User>=Vec::new();

pub async fn root() -> &'static str {
    "Intro to user project"
}

// this creates user profiles and stores them in a vector
pub async fn create_user() -> Json<Vec<User>>{
  
  let user=User{
        name : "Ebuka".to_string(),
        id : 40441};
    let user1=User{
        name:"Frank".to_string(),
        id:49042,};
    let user2=User{
        name: "Fred".to_string(),
        id: 40896
    };
    let user3=User{
        name:"Akunna".to_string(),
        id:40875};
      
    unsafe{
        if USERS.is_empty(){
        USERS.push(user);
        USERS.push(user1);
        USERS.push(user2);
        USERS.push(user3);
        }
    }
        Json(unsafe{USERS.clone()})
}

        
//this is to delete a user using the user's id
pub async fn delete_user(Path(id):Path<u32>)-> impl IntoResponse{
   unsafe{ if let Some(pos)=USERS.iter().position(|user|user.id==id){
        USERS.remove(pos);
        return "User has been deleted".to_string().into_response();
    }
    "User not found".to_string().into_response()
}
}
            


pub async fn update_user(Path(id):Path<u32>, Json(updated_user):Json<User>)-> impl IntoResponse{
    {
      unsafe { if let Some(user)=
            USERS.iter_mut().find(|user|user.id==id){
            user.name=updated_user.name;
            user.id = updated_user.id;
            return "User has been updated".to_string().into_response();}
            "user not found".to_string().into_response()}}}
          
    


pub async fn get_users() -> Json<Vec<User>>{
    Json(unsafe{USERS.clone()})
}


pub async fn get_single_user_by_id(Path(id):Path<u32>) -> impl IntoResponse {
    unsafe{if let Some(user)=USERS.iter().find(|user|user.id==id){
    return Json(user.clone()).into_response();
}
    "User not found".to_string().into_response()   
}}
