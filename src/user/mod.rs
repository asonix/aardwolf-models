use chrono::DateTime;
use chrono::offset::Utc;

pub mod local_auth;

use schema::users;
use self::local_auth::LocalAuth;
pub use self::local_auth::{PlaintextPassword, VerificationError};
use email::Email;

pub trait UserLike {
    fn id(&self) -> i32;
    fn primary_email(&self) -> i32;
    fn created_at(&self) -> DateTime<Utc>;
}

pub struct AuthenticatedUser {
    id: i32,
    primary_email: i32,
    created_at: DateTime<Utc>,
}

impl UserLike for AuthenticatedUser {
    fn id(&self) -> i32 {
        self.id
    }

    fn primary_email(&self) -> i32 {
        self.primary_email
    }

    fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
}

#[derive(Queryable)]
pub struct QueriedUser {
    id: i32,
    primary_email: i32,
    created_at: DateTime<Utc>,
}

impl UserLike for QueriedUser {
    fn id(&self) -> i32 {
        self.id
    }

    fn primary_email(&self) -> i32 {
        self.primary_email
    }

    fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
}

#[derive(Queryable)]
pub struct UnAuthenticatedUser {
    id: i32,
    primary_email: i32, // foreign key to Email
    created_at: DateTime<Utc>,
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

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    primary_email: i32,
    created_at: DateTime<Utc>,
}

impl NewUser {
    pub fn new(email: &Email) -> Self {
        NewUser {
            primary_email: email.id(),
            created_at: Utc::now(),
        }
    }
}
