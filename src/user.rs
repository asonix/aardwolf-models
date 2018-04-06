use password::{CreationError, Password, VerificationError};
use schema::users;
use email::Email;

pub trait UserLike {
    fn id(&self) -> i32;
    fn primary_email(&self) -> i32;
}

pub struct Authenticateduser {
    id: i32,
    primary_email: i32,
}

impl UserLike for Authenticateduser {
    fn id(&self) -> i32 {
        self.id
    }

    fn primary_email(&self) -> i32 {
        self.primary_email
    }
}

#[derive(Queryable)]
pub struct QueriedUser {
    id: i32,
    primary_email: i32,
}

impl UserLike for QueriedUser {
    fn id(&self) -> i32 {
        self.id
    }

    fn primary_email(&self) -> i32 {
        self.primary_email
    }
}

#[derive(Queryable)]
pub struct User {
    id: i32,
    password: Password,
    primary_email: i32, // foreign key to Email
}

impl User {
    pub fn log_in(self, password: String) -> Result<Authenticateduser, VerificationError> {
        use password::Verify;

        self.password.verify(&password).map(|_| Authenticateduser {
            id: self.id,
            primary_email: self.primary_email,
        })
    }
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    password: Password,
    primary_email: i32,
}

impl NewUser {
    pub fn new(email: &Email, password: String) -> Result<Self, CreationError> {
        use password::Create;
        let password = Password::create(&password)?;

        Ok(NewUser {
            password: password,
            primary_email: email.id(),
        })
    }
}
