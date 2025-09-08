use crate::domain::entity::user::{User, UserPartial};
use crate::domain::vo::password::Password;
use crate::domain::vo::{email::Email, id::Id, name::Name, temporal::DateTime};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::sync::Arc;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UserDataDto {
    #[validate(length(min = 1, max = 50))]
    name: String,
    #[validate(email)]
    email: String,
    #[validate(length(min = 1, max = 50))]
    password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UserPartialDataDto {
    #[validate(length(min = 1, max = 50))]
    name: Option<String>,
    #[validate(email)]
    email: Option<String>,
    #[validate(length(min = 1, max = 50))]
    password: Option<String>,
    deleted: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct QueryFilter {
    #[validate(email)]
    pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserDataResponseDto {
    id: String,
    name: String,
    email: String,
}

impl TryFrom<UserDataDto> for User {
    type Error = Arc<dyn Error>;

    fn try_from(value: UserDataDto) -> Result<Self, Self::Error> {
        let id = Id::new()?;
        let name = Name::new(value.name)?;
        let email = Email::new(value.email)?;
        let password = Password::new(value.password)?;

        Ok(User::new(
            id,
            name,
            email,
            password,
            false,
            DateTime::new(),
            DateTime::new(),
            None,
        ))
    }
}

impl TryFrom<UserPartialDataDto> for UserPartial {
    type Error = Arc<dyn Error>;

    fn try_from(value: UserPartialDataDto) -> Result<Self, Self::Error> {
        let id: Option<Id>;
        let id_result = Id::new();
        match id_result {
            Ok(i) => {
                id = Some(i);
            }
            Err(e) => return Err(Arc::new(e)),
        }

        let name: Option<Name>;
        let name_result = value.name.map(Name::new);
        match name_result {
            Some(result) => match result {
                Ok(n) => {
                    name = Some(n);
                }
                Err(e) => return Err(e),
            },
            None => {
                name = None;
            }
        }
        let email: Option<Email>;
        let email_result = value.email.map(Email::new);
        match email_result {
            Some(result) => match result {
                Ok(e) => {
                    email = Some(e);
                }
                Err(e) => return Err(e),
            },
            None => {
                email = None;
            }
        }
        let password: Option<Password>;
        let password_result = value.password.map(Password::new);
        match password_result {
            Some(result) => match result {
                Ok(p) => {
                    password = Some(p);
                }
                Err(e) => return Err(e),
            },
            None => {
                password = None;
            }
        }
        let deleted = value.deleted;

        Ok(UserPartial::new(id, name, email, password, deleted))
    }
}

impl From<&User> for UserDataResponseDto {
    fn from(value: &User) -> Self {
        Self {
            id: value.id.value(),
            name: value.name.value(),
            email: value.email.value(),
        }
    }
}
