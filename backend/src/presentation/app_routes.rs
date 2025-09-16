use actix_web::web;
use crate::presentation::auth::auth_handler::signup;
use crate::presentation::user::user_handler::{create_user, delete_user_by_id, get_user_by_email, get_user_by_id, patch_user_by_id};

pub fn routes(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("")

            .service(signup)

            .service(create_user)
            .service(get_user_by_id)
            .service(get_user_by_email)
            .service(patch_user_by_id)
            .service(delete_user_by_id),
    );
}