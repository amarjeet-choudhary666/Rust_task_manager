use actix_web::{web, HttpResponse, Responder};
use mongodb::{bson::{doc, Bson}, Client};
use bcrypt::{DEFAULT_COST, hash, verify};
use mongodb::bson::oid::ObjectId;
use futures::stream::TryStreamExt;
use crate::models::user_model::{LoginUser, NewUser, User, UserResponse};
use crate::utils::jwt::{create_access_token, create_refresh_token};

pub async fn create_user(user: web::Json<NewUser>,
    client: web::Data<Client>
) -> impl Responder {
    if user.name.trim().is_empty() || user.email.trim().is_empty() || user.password.trim().is_empty() {
        return HttpResponse::BadRequest().body("Name, email, and password are required");
    }

    let database = client.database("rust_backend");
    let collection = database.collection::<User>("users");

    if let Ok(Some(_)) = collection.find_one(doc! { "email": &user.email }).await {
        return HttpResponse::BadRequest().body("Email already exists");
    }


    let hashed_password = match hash(&user.password, DEFAULT_COST) {
        Ok(h) => h,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to hash password"),
    };

    let new_user = User {
        id: None,
        name: user.name.clone(),
        email: user.email.clone(),
        password: hashed_password,
        refresh_token: None,
    };


    match collection.insert_one(new_user).await {
        Ok(insert_result) => {
            if let Some(oid) = insert_result.inserted_id.as_object_id() {
                let response = UserResponse {
                    id: Some(oid.to_hex()),
                    name: user.name.clone(),
                    email: user.email.clone(),
                };
                HttpResponse::Ok().json(response)
            } else {
                HttpResponse::InternalServerError().body("Failed to get inserted ID")
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(format!("Error inserting user: {}", err)),
    }
}


pub async fn login_user(
    user: web::Json<LoginUser>,
    client: web::Data<Client>,
) -> impl Responder {
    if user.email.trim().is_empty() || user.password.trim().is_empty() {
        return HttpResponse::BadRequest().body("Email and password are required");
    }

    let database = client.database("rust_backend");
    let collection = database.collection::<User>("users");

    match collection.find_one(doc! { "email": &user.email } ).await {
        Ok(Some(user_doc)) => {
            match verify(&user.password, &user_doc.password) {
                Ok(true) => {
                    let user_id = match user_doc.id.as_ref() {
                        Some(id) => id,
                        None => return HttpResponse::InternalServerError().body("User ID not found"),
                    };
                    let access_token = match create_access_token(user_id) {
                        Ok(token) => token,
                        Err(_) => return HttpResponse::InternalServerError().body("Failed to create access token"),
                    };
                    let refresh_token = match create_refresh_token(user_id) {
                        Ok(token) => token,
                        Err(_) => return HttpResponse::InternalServerError().body("Failed to create refresh token"),
                    };

                    let oid = match ObjectId::parse_str(user_id) {
                        Ok(o) => o,
                        Err(_) => return HttpResponse::InternalServerError().body("Invalid user ID format"),
                    };
                    let update_result = collection
                        .update_one(
                            doc! {"_id": Bson::ObjectId(oid)},
                            doc! {"$set": {"refresh_token": &refresh_token}},
                        )
                        .await;

                    match update_result {
                        Ok(_) => {
                            let response = UserResponse {
                                id: user_doc.id,
                                name: user_doc.name,
                                email: user_doc.email,
                            };

                            HttpResponse::Ok().json(serde_json::json!({
                                "user": response,
                                "access_token": access_token,
                                "refresh_token": refresh_token
                            }))
                        }
                        Err(err) => HttpResponse::InternalServerError()
                            .body(format!("Failed to save refresh token: {}", err)),
                    }
                }
                Ok(false) => HttpResponse::Unauthorized().body("Invalid credentials"),
                Err(_) => HttpResponse::InternalServerError().body("Error verifying password"),
            }
        }
        Ok(None) => HttpResponse::Unauthorized().body("Invalid credentials"),
        Err(err) => HttpResponse::InternalServerError().body(format!("Database error: {}", err)),
    }
}


pub async fn get_user(
    client: web::Data<Client>,
) -> impl Responder {
    let database = client.database("rust_backend");
    let collection = database.collection::<User>("users");

    let cursor = match collection.find(doc!{}).await {
        Ok(c) => c,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Database error: {}", e)),
    };

    let users: Vec<UserResponse> = match cursor.try_collect::<Vec<_>>().await {
        Ok(docs) => docs.into_iter().map(|user| UserResponse {
            id: user.id,
            name: user.name,
            email: user.email,
        }).collect(),
        Err(e) => return HttpResponse::InternalServerError().body(format!("Error reading users: {}", e)),
    };

    HttpResponse::Ok().json(users)
}