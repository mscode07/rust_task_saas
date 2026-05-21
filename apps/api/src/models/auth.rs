use serde::{Deserialize,Serialize};

#[derive(Debug,Deserialize)]
pub struct RegisterRequest{
    pub name: String,
    pub email: String,
    pub password:String,
}

#[derive(Debug,Serialize)]
pub struct AuthResponse{
    pub message: String,
}