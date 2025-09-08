use crate::common::error::{AppError, ErrorData};
use crate::common::result::ResultApp;
use crate::domain::entity::user::User;
use crate::domain::vo::email::Email;
use crate::domain::vo::id::Id;
use crate::infrastructure::postgres::PostgresBaseRepository;
use crate::repositories::schema::users;
use crate::repositories::schema::users::dsl::users as users_dsl;
use crate::repositories::schema::users::{
    deleted, deleted_at, email, id, name, password, updated_at,
};
use crate::repositories::user::model::UserModel;
use async_trait::async_trait;
use diesel::insert_into;
use diesel::prelude::*;
use diesel::update;
use std::sync::Arc;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn save(&self, user: &User) -> ResultApp<User>;
    async fn find_by_id(&self, id: &Id) -> ResultApp<Option<User>>;
    async fn find_by_email(&self, email: &Email) -> ResultApp<Option<User>>;
    async fn delete(&self, id: &Id) -> ResultApp<Option<User>>;
    async fn update(&self, user: &User) -> ResultApp<Option<User>>;
}

#[derive(Debug, Clone)]
pub struct UserRepositoryPostgres {
    pub base_repository: PostgresBaseRepository,
}

impl UserRepositoryPostgres {
    pub fn new(base_repository: PostgresBaseRepository) -> Self {
        UserRepositoryPostgres { base_repository }
    }
}

#[async_trait]
impl UserRepository for UserRepositoryPostgres {
    async fn save(&self, user: &User) -> ResultApp<User> {
        let payment_model = UserModel::from(user.clone());

        let connection_result = self.base_repository.pool.get();
        if let Err(err) = connection_result {
            let app_error = AppError::Database(
                ErrorData::new("internal", "database error").with_cause(Some(Arc::new(err))),
            );
            return Err(Arc::new(app_error));
        }

        let mut connection = connection_result.unwrap();

        let insert_result = insert_into(users::table)
            .values(&payment_model)
            .get_result::<UserModel>(&mut connection);

        if insert_result.is_err() {
            let app_error = AppError::Database(
                ErrorData::new("internal", "database error")
                    .with_cause(Some(Arc::new(insert_result.err().unwrap()))),
            );
            return Err(Arc::new(app_error));
        }

        Ok(user.clone())
    }

    async fn find_by_id(&self, user_id: &Id) -> ResultApp<Option<User>> {
        let connection_result = self.base_repository.pool.get();
        if let Err(err) = connection_result {
            let app_error = AppError::Database(
                ErrorData::new("internal", "database error").with_cause(Some(Arc::new(err))),
            );
            return Err(Arc::new(app_error));
        }

        let user_response = users::table
            .filter(id.eq(user_id.value()))
            .select(UserModel::as_select())
            .first(&mut connection_result.unwrap())
            .optional();

        match user_response {
            Ok(Some(user)) => Ok(Some(User::from(user))),
            Ok(None) => Ok(None),
            Err(err) => {
                let app_error = AppError::Database(
                    ErrorData::new("internal", "database error").with_cause(Some(Arc::new(err))),
                );
                Err(Arc::new(app_error))
            }
        }
    }

    async fn find_by_email(&self, user_email: &Email) -> ResultApp<Option<User>> {
        let connection_result = self.base_repository.pool.get();
        if let Err(err) = connection_result {
            let app_error = AppError::Database(
                ErrorData::new("internal", "database error").with_cause(Some(Arc::new(err))),
            );
            return Err(Arc::new(app_error));
        }

        let user_response = users::table
            .filter(email.eq(user_email.value()))
            .select(UserModel::as_select())
            .first(&mut connection_result.unwrap())
            .optional();

        match user_response {
            Ok(Some(user)) => Ok(Some(User::from(user))),
            Ok(None) => Ok(None),
            Err(err) => {
                let app_error = AppError::Database(
                    ErrorData::new("internal", "database error").with_cause(Some(Arc::new(err))),
                );
                Err(Arc::new(app_error))
            }
        }
    }

    async fn delete(&self, user_id: &Id) -> ResultApp<Option<User>> {
        let connection_result = self.base_repository.pool.get();
        if let Err(err) = connection_result {
            let app_error = AppError::Database(
                ErrorData::new("internal", "database error").with_cause(Some(Arc::new(err))),
            );
            return Err(Arc::new(app_error));
        }

        let current_time = chrono::Utc::now().naive_utc();
        let updated_result = update(users_dsl.find(user_id.value()))
            .set((
                updated_at.eq(current_time),
                deleted.eq(true),
                deleted_at.eq(current_time),
            ))
            .returning(UserModel::as_returning())
            .get_result(&mut connection_result.unwrap());

        match updated_result {
            Ok(user) => Ok(Some(User::from(user))),
            Err(err) => {
                let app_error = AppError::Database(
                    ErrorData::new("internal", "database error").with_cause(Some(Arc::new(err))),
                );
                Err(Arc::new(app_error))
            }
        }
    }

    async fn update(&self, user: &User) -> ResultApp<Option<User>> {
        let connection_result = self.base_repository.pool.get();
        if let Err(err) = connection_result {
            let app_error = AppError::Database(
                ErrorData::new("internal", "database error").with_cause(Some(Arc::new(err))),
            );
            return Err(Arc::new(app_error));
        }

        let current_time = chrono::Utc::now().naive_utc();
        let updated_result = update(users_dsl.find(user.id.value()))
            .set((
                name.eq(user.name.value()),
                email.eq(user.email.value()),
                password.eq(user.password.value()),
                updated_at.eq(current_time),
            ))
            .returning(UserModel::as_returning())
            .get_result(&mut connection_result.unwrap());

        match updated_result {
            Ok(user) => Ok(Some(User::from(user))),
            Err(err) => {
                let app_error = AppError::Database(
                    ErrorData::new("internal", "database error").with_cause(Some(Arc::new(err))),
                );
                Err(Arc::new(app_error))
            }
        }
    }
}
