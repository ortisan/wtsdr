use crate::domain::vo::customer_service_category::CustomerServiceCategory;
use crate::domain::vo::description::Description;
use crate::domain::vo::geopoint::GeoPoint;
use crate::domain::vo::id::Id;
use crate::domain::vo::name::Name;
use crate::domain::vo::phone::Phone;
use crate::domain::vo::photo::Photo;
use crate::domain::vo::tags::Tags;
use crate::domain::vo::temporal::DateTime;
use crate::domain::vo::url::Url;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct CustomerService {
    pub id: Id,
    pub name: Name,
    pub description: Description,
    pub location: GeoPoint,
    pub phone: Phone,
    pub website: Option<Url>,
    pub photos: Vec<Photo>,
    pub tags: Tags,
    pub categories: HashSet<CustomerServiceCategory>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}
