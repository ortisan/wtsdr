use crate::common::error::{AppError, ErrorData};
use crate::common::result::ResultApp;
use crate::domain::entity::user::User;
use crate::domain::vo::id::Id;
use crate::repositories::user::user_repository::UserRepository;
use std::error::Error;
use std::sync::Arc;

#[async_trait::async_trait]
pub trait DeleteUserUseCase: Send + Sync {
    async fn delete_user(&self, user_id: &Id) -> ResultApp<User>;
}

pub struct DeleteUserUseCaseImpl {
    user_repository: Arc<dyn UserRepository>,
}

impl DeleteUserUseCaseImpl {
    pub fn new(user_repository: Arc<dyn UserRepository>) -> Self {
        Self { user_repository }
    }
}

#[async_trait::async_trait]
impl DeleteUserUseCase for DeleteUserUseCaseImpl {
    async fn delete_user(&self, user_id: &Id) -> ResultApp<User> {
        match self.user_repository.delete(user_id).await {
            Ok(user) => {
                match user {
                    Some(user) => Ok(user),
                    None => {
                        // Create a simple error as the cause
                        let custom_err =
                            std::io::Error::new(std::io::ErrorKind::NotFound, "User not found");
                        let err_arc: Arc<dyn Error> = Arc::new(custom_err);
                        Err(Arc::new(AppError::NotFound(
                            ErrorData::new("user-not-found", "user not found")
                                .with_cause(Some(err_arc)),
                        )))
                    }
                }
            }
            Err(error) => Err(error),
        }
    }
}
