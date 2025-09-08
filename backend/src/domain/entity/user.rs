use crate::domain::vo::email::Email;
use crate::domain::vo::id::Id;
use crate::domain::vo::name::Name;
use crate::domain::vo::password::Password;
use crate::domain::vo::temporal::DateTime;

#[derive(Debug, Clone)]
pub struct User {
    pub id: Id,
    pub name: Name,
    pub email: Email,
    pub password: Password,
    pub deleted: bool,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub deleted_at: Option<DateTime>,
}

impl User {
    pub fn new(
        id: Id,
        name: Name,
        email: Email,
        password: Password,
        deleted: bool,
        created_at: DateTime,
        updated_at: DateTime,
        deleted_at: Option<DateTime>,
    ) -> Self {
        Self {
            id,
            name,
            email,
            password,
            deleted,
            created_at,
            updated_at,
            deleted_at,
        }
    }
}

#[derive(Debug, Clone)]
pub struct UserPartial {
    pub id: Option<Id>,
    pub name: Option<Name>,
    pub email: Option<Email>,
    pub password: Option<Password>,
    pub deleted: Option<bool>,
}

impl UserPartial {
    pub fn new(
        id: Option<Id>,
        name: Option<Name>,
        email: Option<Email>,
        password: Option<Password>,
        deleted: Option<bool>,
    ) -> Self {
        Self {
            id,
            name,
            email,
            password,
            deleted,
        }
    }

    pub fn set_id(&mut self, id: Id) {
        self.id = Some(id);
    }
}
