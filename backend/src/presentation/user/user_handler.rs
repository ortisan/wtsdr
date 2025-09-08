use crate::common::error::AppError;
use crate::domain::entity::user::{User, UserPartial};
use crate::domain::usecase::user::create_user::CreateUserUseCase;
use crate::domain::usecase::user::delete_user::DeleteUserUseCase;
use crate::domain::usecase::user::update_user::UpdateUserUseCase;
use crate::domain::vo::email::Email;
use crate::domain::vo::id::Id;
use crate::presentation::user::dto::{
    QueryFilter, UserDataDto, UserDataResponseDto, UserPartialDataDto,
};
use crate::repositories::user::user_repository::UserRepository;
use actix_web::{HttpResponse, delete, get, patch, post, web};
use serde_json::json;
use std::sync::Arc;
use validator::Validate;

#[post("/users{tail:/*}")]
pub async fn create_user(
    create_user_use_case: web::Data<Arc<dyn CreateUserUseCase>>,
    user_data: web::Json<UserDataDto>,
) -> HttpResponse {
    if let Err(error) = user_data.validate() {
        return HttpResponse::from(AppError::from(error));
    }

    let user = match User::try_from(user_data.into_inner()) {
        Ok(u) => u,
        Err(error) => return HttpResponse::from(AppError::from(error.clone())),
    };

    let persisted_user_result = create_user_use_case.create_user(&user).await;

    match persisted_user_result {
        Ok(user) => {
            let user_response = UserDataResponseDto::from(&user);
            HttpResponse::Created().json(user_response)
        }
        Err(error) => HttpResponse::from(AppError::from(error.clone())),
    }
}

#[get("/users/{id}")]
pub async fn get_user_by_id(
    user_repository: web::Data<Arc<dyn UserRepository>>,
    id_path: web::Path<String>,
) -> HttpResponse {
    let user_id = Id::new_from_string(id_path.into_inner());
    if user_id.is_err() {
        return HttpResponse::from(AppError::from(user_id.err().unwrap()));
    }
    let user_result = user_repository.find_by_id(&user_id.unwrap()).await;
    match user_result {
        Ok(user) => match user {
            Some(u) => {
                let user_response = UserDataResponseDto::from(&u);
                HttpResponse::Ok().json(user_response)
            }
            None => HttpResponse::NotFound().json(json!({})),
        },
        Err(error) => HttpResponse::from(AppError::from(error.clone())),
    }
}

#[get("/users{tail:/*}")]
pub async fn get_user_by_email(
    user_repository: web::Data<Arc<dyn UserRepository>>,
    query_params_data: web::Query<QueryFilter>,
) -> HttpResponse {
    if let Err(error) = query_params_data.validate() {
        return HttpResponse::from(AppError::from(error));
    }

    let user_email = Email::new(query_params_data.into_inner().email);
    if user_email.is_err() {
        return HttpResponse::from(AppError::from(user_email.err().unwrap()));
    }
    let user_result = user_repository.find_by_email(&user_email.unwrap()).await;
    match user_result {
        Ok(user) => match user {
            Some(user) => {
                let user_response = UserDataResponseDto::from(&user);
                HttpResponse::Ok().json(vec![user_response])
            }
            None => HttpResponse::Ok().json(Vec::<UserDataResponseDto>::new()),
        },
        Err(error) => HttpResponse::from(AppError::from(error.clone())),
    }
}

#[patch("/users/{id}")]
pub async fn patch_user_by_id(
    update_use_case: web::Data<Arc<dyn UpdateUserUseCase>>,
    id_path: web::Path<String>,
    user_partial_data: web::Json<UserPartialDataDto>,
) -> HttpResponse {
    if let Err(error) = user_partial_data.validate() {
        return HttpResponse::from(AppError::from(error));
    }

    let mut user_partial: UserPartial = match UserPartial::try_from(user_partial_data.into_inner())
    {
        Ok(u) => u,
        Err(error) => return HttpResponse::from(AppError::from(error.clone())),
    };
    user_partial.set_id(Id::new_from_string(id_path.into_inner()).unwrap());

    let update_user_result = update_use_case.update_user(&user_partial).await;
    match update_user_result {
        Ok(user) => {
            let user_response = UserDataResponseDto::from(&user);
            HttpResponse::Ok().json(user_response)
        }
        Err(error) => HttpResponse::from(AppError::from(error.clone())),
    }
}

#[delete("/users/{id}")]
pub async fn delete_user_by_id(
    delete_use_case: web::Data<Arc<dyn DeleteUserUseCase>>,
    id_path: web::Path<String>,
) -> HttpResponse {
    let user_id = Id::new_from_string(id_path.into_inner());
    let delete_user_result = delete_use_case.delete_user(&user_id.unwrap()).await;
    match delete_user_result {
        Ok(user) => {
            let user_response = UserDataResponseDto::from(&user);
            HttpResponse::Ok().json(user_response)
        }
        Err(error) => HttpResponse::from(AppError::from(error.clone())),
    }
}
