use crate::common::result::ResultApp;
use crate::domain::entity::user::User;
use crate::repositories::user::user_repository::UserRepository;
use std::sync::Arc;

#[async_trait::async_trait]
pub trait SigningUseCase: Send + Sync {
    async fn signin(&self, user: &User) -> ResultApp<User>;
}

pub struct SigningUseCaseImpl {
    user_repository: Arc<dyn UserRepository>,
}

impl SigningUseCaseImpl {
    pub fn new(user_repository: Arc<dyn UserRepository>) -> Self {
        Self { user_repository }
    }
}

#[async_trait::async_trait]
impl SigningUseCase for SigningUseCaseImpl {
    async fn signin(&self, user: &User) -> ResultApp<User> {
        self.user_repository.save(user).await
    }
}
