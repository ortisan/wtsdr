use crate::presentation::user::user_handler::{
    create_user, delete_user_by_id, get_user_by_email, get_user_by_id, patch_user_by_id,
};
use actix_web::web;

pub fn routes(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("")
            .service(create_user)
            .service(get_user_by_id)
            .service(get_user_by_email)
            .service(patch_user_by_id)
            .service(delete_user_by_id),
    );
}
