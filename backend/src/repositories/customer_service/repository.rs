use crate::common::error::{AppError, ErrorData};
use crate::common::result::ResultApp;
use crate::domain::entity::customer_service::CustomerService;
use crate::domain::vo::id::Id;
use crate::infrastructure::postgres::PostgresBaseRepository;
use crate::repositories::customer_service::model::CustomerServiceModel;
use crate::repositories::schema::customer_services;
use crate::repositories::schema::customer_services::dsl::customer_services as services_dsl;
use crate::repositories::schema::customer_services::{
    created_at, deleted, deleted_at, description, id, latitude, longitude, name, phone, updated_at,
    website,
};
use async_trait::async_trait;
use diesel::insert_into;
use diesel::prelude::*;
use diesel::update;
use std::sync::Arc;

#[async_trait]
pub trait CustomerServiceRepository: Send + Sync {
    async fn save(&self, service: &CustomerService) -> ResultApp<CustomerService>;
    async fn find_by_id(&self, id: &Id) -> ResultApp<Option<CustomerService>>;
    async fn delete(&self, id: &Id) -> ResultApp<Option<CustomerService>>;
    async fn update(&self, service: &CustomerService) -> ResultApp<Option<CustomerService>>;
}

#[derive(Debug, Clone)]
pub struct CustomerServiceRepositoryPostgres {
    pub base_repository: PostgresBaseRepository,
}

impl CustomerServiceRepositoryPostgres {
    pub fn new(base_repository: PostgresBaseRepository) -> Self {
        CustomerServiceRepositoryPostgres { base_repository }
    }
}

#[async_trait]
impl CustomerServiceRepository for CustomerServiceRepositoryPostgres {
    async fn save(&self, service: &CustomerService) -> ResultApp<CustomerService> {
        let model = CustomerServiceModel::from(service.clone());

        let connection_result = self.base_repository.pool.get();
        if let Err(err) = connection_result {
            let app_error = AppError::Database(
                ErrorData::new("internal", "database error").with_cause(Some(Arc::new(err))),
            );
            return Err(Arc::new(app_error));
        }

        let mut connection = connection_result.unwrap();

        let insert_result = insert_into(customer_services::table)
            .values(&model)
            .get_result::<CustomerServiceModel>(&mut connection);

        if insert_result.is_err() {
            let app_error = AppError::Database(
                ErrorData::new("internal", "database error")
                    .with_cause(Some(Arc::new(insert_result.err().unwrap()))),
            );
            return Err(Arc::new(app_error));
        }

        Ok(service.clone())
    }

    async fn find_by_id(&self, service_id: &Id) -> ResultApp<Option<CustomerService>> {
        let connection_result = self.base_repository.pool.get();
        if let Err(err) = connection_result {
            let app_error = AppError::Database(
                ErrorData::new("internal", "database error").with_cause(Some(Arc::new(err))),
            );
            return Err(Arc::new(app_error));
        }

        let response = customer_services::table
            .filter(id.eq(service_id.value()))
            .select(CustomerServiceModel::as_select())
            .first(&mut connection_result.unwrap())
            .optional();

        match response {
            Ok(Some(model)) => Ok(Some(CustomerService::from(model))),
            Ok(None) => Ok(None),
            Err(err) => {
                let app_error = AppError::Database(
                    ErrorData::new("internal", "database error").with_cause(Some(Arc::new(err))),
                );
                Err(Arc::new(app_error))
            }
        }
    }

    async fn delete(&self, service_id: &Id) -> ResultApp<Option<CustomerService>> {
        let connection_result = self.base_repository.pool.get();
        if let Err(err) = connection_result {
            let app_error = AppError::Database(
                ErrorData::new("internal", "database error").with_cause(Some(Arc::new(err))),
            );
            return Err(Arc::new(app_error));
        }

        let current_time = chrono::Utc::now().naive_utc();
        let updated_result = update(services_dsl.find(service_id.value()))
            .set((
                updated_at.eq(current_time),
                deleted.eq(true),
                deleted_at.eq(current_time),
            ))
            .returning(CustomerServiceModel::as_returning())
            .get_result(&mut connection_result.unwrap());

        match updated_result {
            Ok(model) => Ok(Some(CustomerService::from(model))),
            Err(err) => {
                let app_error = AppError::Database(
                    ErrorData::new("internal", "database error").with_cause(Some(Arc::new(err))),
                );
                Err(Arc::new(app_error))
            }
        }
    }

    async fn update(&self, service: &CustomerService) -> ResultApp<Option<CustomerService>> {
        let connection_result = self.base_repository.pool.get();
        if let Err(err) = connection_result {
            let app_error = AppError::Database(
                ErrorData::new("internal", "database error").with_cause(Some(Arc::new(err))),
            );
            return Err(Arc::new(app_error));
        }

        let current_time = chrono::Utc::now().naive_utc();
        let updated_result = update(services_dsl.find(service.id.value()))
            .set((
                name.eq(service.name.value()),
                description.eq(service.description.value()),
                latitude.eq(service.location.value().0),
                longitude.eq(service.location.value().1),
                phone.eq(service.phone.value()),
                website.eq(service.website.as_ref().map(|u| u.as_str())),
                updated_at.eq(current_time),
            ))
            .returning(CustomerServiceModel::as_returning())
            .get_result(&mut connection_result.unwrap());

        match updated_result {
            Ok(model) => Ok(Some(CustomerService::from(model))),
            Err(err) => {
                let app_error = AppError::Database(
                    ErrorData::new("internal", "database error").with_cause(Some(Arc::new(err))),
                );
                Err(Arc::new(app_error))
            }
        }
    }
}
