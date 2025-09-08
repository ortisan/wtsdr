use crate::domain::entity::customer_service::CustomerService;
use crate::domain::vo::description::Description;
use crate::domain::vo::geopoint::GeoPoint;
use crate::domain::vo::id::Id;
use crate::domain::vo::name::Name;
use crate::domain::vo::phone::Phone;
use crate::domain::vo::temporal::DateTime;
use crate::domain::vo::url::Url;
use chrono::{DateTime as ChronoDateTime, Utc};
use diesel::{Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::repositories::schema::customer_services)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CustomerServiceModel {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub description: String,
    pub latitude: f64,
    pub longitude: f64,
    pub phone: String,
    pub website: Option<String>,
    pub deleted: bool,
    pub created_at: ChronoDateTime<Utc>,
    pub updated_at: ChronoDateTime<Utc>,
    pub deleted_at: Option<ChronoDateTime<Utc>>,
}

impl From<CustomerServiceModel> for CustomerService {
    fn from(m: CustomerServiceModel) -> Self {
        Self {
            id: Id::new_from_string(m.id).unwrap(),
            user_id: Id::new_from_string(m.user_id).unwrap(),           
            name: Name::new(m.name).unwrap(),
            description: Description::new(m.description).unwrap(),
            location: GeoPoint::new(m.latitude, m.longitude).unwrap(),
            phone: Phone::new(m.phone).unwrap(),
            website: m.website.map(|u| Url::new(u).unwrap()),
            photos: vec![],
            tags: Default::default(),
            categories: Default::default(),
            created_at: DateTime::new_from_date_time(m.created_at),
            updated_at: DateTime::new_from_date_time(m.updated_at),
            deleted: m.deleted,
            deleted_at: m.deleted_at.map(DateTime::new_from_date_time),
        }
    }
}

impl From<CustomerService> for CustomerServiceModel {
    fn from(customer_service: CustomerService) -> Self {
        Self {
            id: customer_service.id.value(),
            user_id: customer_service.user_id.value(),
            name: customer_service.name.value().to_string(),
            description: customer_service.description.value().to_string(),
            latitude: customer_service.location.value().0,
            longitude: customer_service.location.value().1,
            phone: customer_service.phone.value().to_string(),
            website: customer_service.website.map(|w| w.as_str().to_string()),
            deleted: customer_service.deleted,
            created_at: customer_service.created_at.to_chono_date_time(),
            updated_at: customer_service.updated_at.to_chono_date_time(),
            deleted_at: customer_service.deleted_at.map(|dt| dt.to_chono_date_time()),
        }
    }
}
