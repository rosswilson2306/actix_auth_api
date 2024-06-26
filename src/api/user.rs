use crate::auth::{authorize, create_jwt};
use crate::db::{users::Database, users_trait::UserData};
use crate::model::auth::{LoginRequest, LoginResponse};
use crate::model::user::{AddUserRequest, GetUserRequest, Role, UpdateUserRequest, User};
use crate::{Error, Result};
use actix_web::HttpRequest;
use actix_web::{
    get, patch, post,
    web::{Data, Json, Path},
    HttpResponse, Responder,
};
use uuid::Uuid;
use validator::Validate;

#[post("/login")]
async fn login(db: Data<Database>, body: Json<LoginRequest>) -> Result<Json<LoginResponse>> {
    let user = Database::get_user_by_login(&db, body.clone()).await?;

    let token = create_jwt(&user.uuid, &Role::from_str(&user.role))?;
    Ok(Json(LoginResponse { token }))
}

#[get("/verify")]
pub async fn verify() -> impl Responder {
    HttpResponse::Ok().body("Verify token")
}

#[get("/users")]
async fn get_users(req: HttpRequest, db: Data<Database>) -> Result<Json<Vec<User>>> {
    let headers = req.headers();
    // TODO: remove from here as this request won't block access
    let authorized = authorize(Role::Admin, headers);

    match authorized {
        Ok(_) => {
            let users = Database::get_all_users(&db).await?;
            Ok(Json(users))
        }
        Err(_) => Err(Error::AccessForbidden),
    }
}

#[get("users/{uuid}")]
async fn get_user(path: Path<GetUserRequest>, db: Data<Database>) -> Result<Json<User>> {
    let uuid = path.into_inner().uuid;
    let user = Database::get_user(&db, uuid).await?;

    Ok(Json(user))
}

#[post("/add-user")]
async fn add_user(body: Json<AddUserRequest>, db: Data<Database>) -> Result<Json<User>> {
    let is_valid = body.validate();

    match is_valid {
        // TODO: investigate refactoring to avoid clones
        Ok(_) => {
            let user_from_body = User {
                uuid: Uuid::new_v4().to_string(),
                name: body.name.clone(),
                email: body.email.clone(),
                password: body.password.clone(),
                role: body.role.clone(),
            };

            let new_user = Database::add_user(&db, user_from_body).await;

            match new_user {
                Some(created_user) => Ok(Json(created_user)),
                None => Err(Error::BadUserRequest),
            }
        }
        Err(_) => Err(Error::BadUserRequest),
    }
}

#[patch("/update-user/{uuid}")]
async fn update_user(
    body: Json<UpdateUserRequest>,
    path: Path<String>,
    db: Data<Database>,
) -> Result<Json<User>> {
    let uuid = path.into_inner();
    let is_valid = body.validate();

    match is_valid {
        Ok(_) => {
            // TODO: allow optional fields in request and update only fields passed in request
            let updated_user = Database::update_user(&db, uuid, body.into_inner()).await;

            match updated_user {
                Some(user) => Ok(Json(user)),
                None => Err(Error::BadUserRequest),
            }
        }
        Err(_) => Err(Error::BadUserRequest),
    }
}
