use crate::domain::vo::name::Name;
use crate::domain::vo::tax_id::TaxId;
use crate::domain::vo::temporal::Date;

#[derive(Debug, Clone)]
pub struct Person {
    name: String,
    birth_date: Date,
    social_name: Name,
    tax_id: TaxId,
}
