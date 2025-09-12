use crate::domain::entity::user::User;
use crate::domain::vo::auth_token::AuthToken;
use crate::domain::vo::email::Email;
use crate::domain::vo::id::Id;
use crate::domain::vo::name::Name;
use crate::domain::vo::password::Password;
use crate::domain::vo::temporal::DateTime;
use chrono::{DateTime as ChronoDateTime, Utc};
use diesel::{Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::repositories::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserModel {
    pub id: String,
    pub name: String,
    pub email: String,
    pub password: Option<String>,
    pub auth_token: Option<String>,
    pub deleted: bool,
    pub created_at: ChronoDateTime<Utc>,
    pub updated_at: ChronoDateTime<Utc>,
    pub deleted_at: Option<ChronoDateTime<Utc>>,
}

impl From<UserModel> for User {
    fn from(user_model: UserModel) -> Self {
        let password = match user_model.password {
            Some(p) => Some(Password::new(p).unwrap()),
            None => None,
        };

        let token = match user_model.auth_token {
            Some(p) => Some(AuthToken::new(p).unwrap()),
            None => None,
        };

        Self {
            id: Id::new_from_string(user_model.id).unwrap(),
            name: Name::new(user_model.name).unwrap(),
            email: Email::new(user_model.email).unwrap(),
            password,
            auth_token: token,
            deleted: user_model.deleted,
            created_at: DateTime::new_from_date_time(user_model.created_at),
            updated_at: DateTime::new_from_date_time(user_model.updated_at),
            deleted_at: user_model.deleted_at.map(DateTime::new_from_date_time),
        }
    }
}

impl From<User> for UserModel {
    fn from(user: User) -> Self {
        Self {
            id: user.id.value(),
            name: user.name.value().to_string(),
            email: user.email.value().clone(),
            password: user.password.map(|p| p.value()),
            auth_token: user.auth_token.map(|at| at.token),
            deleted: user.deleted,
            created_at: user.created_at.to_chono_date_time(),
            updated_at: user.updated_at.to_chono_date_time(),
            deleted_at: user.deleted_at.map(|dt| dt.to_chono_date_time()),
        }
    }
}
