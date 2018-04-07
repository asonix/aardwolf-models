use chrono::DateTime;
use chrono::offset::Utc;

pub mod email;
pub mod local_auth;

use schema::users;
use self::local_auth::LocalAuth;
pub use self::local_auth::{PlaintextPassword, VerificationError};

pub trait UserLike {
    fn id(&self) -> i32;
    fn primary_email(&self) -> &str;
    fn created_at(&self) -> DateTime<Utc>;
}

pub struct AuthenticatedUser {
    id: i32,
    primary_email: String,
    created_at: DateTime<Utc>,
}

impl UserLike for AuthenticatedUser {
    fn id(&self) -> i32 {
        self.id
    }

    fn primary_email(&self) -> &str {
        &self.primary_email
    }

    fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
}

#[derive(Debug, Identifiable, Queryable)]
#[table_name = "users"]
pub struct QueriedUser {
    id: i32,
    created_at: DateTime<Utc>,
    primary_email: String,
}

impl UserLike for QueriedUser {
    fn id(&self) -> i32 {
        self.id
    }

    fn primary_email(&self) -> &str {
        &self.primary_email
    }

    fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
}

#[derive(Debug, Identifiable, Queryable)]
#[table_name = "users"]
pub struct UnAuthenticatedUser {
    id: i32,
    created_at: DateTime<Utc>,
    primary_email: String,
}

impl UnAuthenticatedUser {
    pub fn log_in_local(
        self,
        local_auth: LocalAuth,
        password: PlaintextPassword,
    ) -> Result<AuthenticatedUser, VerificationError> {
        local_auth.log_in(self, password)
    }
}

impl UserLike for UnAuthenticatedUser {
    fn id(&self) -> i32 {
        self.id
    }

    fn primary_email(&self) -> &str {
        &self.primary_email
    }

    fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    primary_email: String,
    created_at: DateTime<Utc>,
}

impl NewUser {
    pub fn new(email: String) -> Self {
        NewUser {
            primary_email: email,
            created_at: Utc::now(),
        }
    }
}
