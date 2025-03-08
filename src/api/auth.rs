use actix_web::{web, HttpResponse, Responder};
use paperclip::actix::*;
use serde::Deserialize;
use serde_json::json;
use crate::db::{DbPool, models::Player};


#[derive(Deserialize, Apiv2Schema)]
pub struct RegisterInput {
    pub name: String,
    pub email: String,
    pub password: String,
    pub skill_level: Option<String>,
}

#[api_v2_operation]
pub async fn register(
    item: web::Json<RegisterInput>,
    pool: web::Data<DbPool>
) -> impl Responder {
    let conn = &mut pool.get().expect("Failed to get DB connection");

    use crate::db::schema::players::dsl::*;
    use diesel::prelude::*;
    use bcrypt::*;
    use chrono::Local;

    let hashed_password = hash(&item.password, DEFAULT_COST)
        .expect("Failed to hash password");

    match diesel::insert_into(players)
        .values((
            name.eq(&item.name),
            email.eq(&item.email), 
            password.eq(&hashed_password),
            skill_level.eq(&item.skill_level),
            created_at.eq(Local::now().naive_local())
        ))
        .execute(conn)
    {
        Ok(_) => HttpResponse::Created().json("User registered successfully"),
        Err(error) => {
            println!("Failed to register user");
            println!("Error: {:?}", error);
            HttpResponse::InternalServerError().json("Failed to register user")
        }
    }
}

#[derive(Deserialize, Apiv2Schema)]
pub struct LoginInput {
    pub email: String,
    pub password: String,
}

#[api_v2_operation]
pub async fn login(
    item: web::Json<LoginInput>,
    pool: web::Data<DbPool>
) -> impl Responder {
    let conn = &mut pool.get().expect("Failed to get DB connection");

    use crate::db::schema::players::dsl::*;
    use diesel::prelude::*;
    use bcrypt::*;

    // Find user by email
    let user_result = players
        .filter(email.eq(&item.email))
        .first::<Player>(conn);

    match user_result {
        Ok(user) => {
            // Verify password
            match verify(&item.password, &user.password) {
                Ok(valid) => {
                    if valid {
                        let response = json!({
                            "message": "Login successful",
                            "user_id": user.name,
                        });
                        HttpResponse::Ok().json(response)
                    } else {
                        HttpResponse::Unauthorized().json("Invalid credentials")
                    }
                },
                Err(_) => HttpResponse::InternalServerError().json("Error verifying password")
            }
        },
        Err(_) => HttpResponse::Unauthorized().json("Invalid credentials")
    }
}
