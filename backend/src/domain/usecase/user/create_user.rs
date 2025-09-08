use crate::common::result::ResultApp;
use crate::domain::entity::user::User;
use crate::repositories::user::user_repository::UserRepository;
use std::sync::Arc;

#[async_trait::async_trait]
pub trait CreateUserUseCase: Send + Sync {
    async fn create_user(&self, user: &User) -> ResultApp<User>;
}

pub struct CreateUserUseCaseImpl {
    user_repository: Arc<dyn UserRepository>,
}

impl CreateUserUseCaseImpl {
    pub fn new(user_repository: Arc<dyn UserRepository>) -> Self {
        Self { user_repository }
    }
}

#[async_trait::async_trait]
impl CreateUserUseCase for CreateUserUseCaseImpl {
    async fn create_user(&self, user: &User) -> ResultApp<User> {
        self.user_repository.save(user).await
    }
}
