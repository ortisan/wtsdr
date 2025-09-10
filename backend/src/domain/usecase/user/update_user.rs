use crate::common::error::{AppError, ErrorData};
use crate::common::result::ResultApp;
use crate::domain::entity::user::{User, UserPartial};
use crate::repositories::user::user_repository::UserRepository;
use std::sync::Arc;

#[async_trait::async_trait]
pub trait UpdateUserUseCase: Send + Sync {
    async fn update_user(&self, user_partial: &UserPartial) -> ResultApp<User>;
}

pub struct UpdateUserUseCaseImpl {
    user_repository: Arc<dyn UserRepository>,
}

impl UpdateUserUseCaseImpl {
    pub fn new(user_repository: Arc<dyn UserRepository>) -> Self {
        Self { user_repository }
    }
}

#[async_trait::async_trait]
impl UpdateUserUseCase for UpdateUserUseCaseImpl {
    async fn update_user(&self, user_partial: &UserPartial) -> ResultApp<User> {
        let persisted_user = match self
            .user_repository
            .find_by_id(&user_partial.id.unwrap())
            .await
        {
            Ok(user) => match user {
                Some(user) => user,
                None => {
                    return Err(Arc::new(AppError::NotFound(ErrorData::new(
                        "user-not-found",
                        "user not found",
                    ))));
                }
            },
            Err(error) => return Err(error),
        };

        let user = User::new(
            user_partial.id.unwrap(),
            user_partial
                .name
                .as_ref()
                .unwrap_or(&persisted_user.name)
                .clone(),
            user_partial
                .email
                .as_ref()
                .unwrap_or(&persisted_user.email)
                .clone(),
            user_partial
                .password
                .as_ref()
                .unwrap_or(&persisted_user.password)
                .clone(),

            user_partial
                .token  
                .as_ref()
                .unwrap_or(&persisted_user.token)
                .clone(),

            user_partial.deleted.unwrap_or(persisted_user.deleted),
            persisted_user.created_at,
            persisted_user.updated_at,
            persisted_user.deleted_at,
        );

        // TODO Match with the session

        match self.user_repository.update(&user).await {
            Ok(user) => match user {
                Some(user) => Ok(user),
                None => Err(Arc::new(AppError::NotFound(ErrorData::new(
                    "user-not-found",
                    "user not found",
                )))),
            },
            Err(error) => Err(error),
        }
    }
}
