use chrono::DateTime;
use chrono::offset::Utc;

mod password;

use self::password::{CreationError, Password};
pub use self::password::VerificationError;
use schema::local_auth;
use user::{AuthenticatedUser, UnAuthenticatedUser};

#[derive(Queryable)]
pub struct LocalAuth {
    id: i32,
    password: Password,
    created_at: DateTime<Utc>,
    user_id: i32, // foreign key to User
}

impl LocalAuth {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn user_id(&self) -> i32 {
        self.user_id
    }

    pub(crate) fn log_in(
        self,
        user: UnAuthenticatedUser,
        password: String,
    ) -> Result<AuthenticatedUser, VerificationError> {
        use self::password::Verify;

        if self.user_id != user.id {
            return Err(VerificationError::Process);
        }

        self.password.verify(&password).map(|_| AuthenticatedUser {
            id: user.id,
            primary_email: user.primary_email,
            created_at: user.created_at,
        })
    }
}

#[derive(Insertable)]
#[table_name = "local_auth"]
pub struct NewLocalAuth {
    password: Password,
    created_at: DateTime<Utc>,
    user_id: i32,
}

impl NewLocalAuth {
    pub fn new(user: &UnAuthenticatedUser, password: String) -> Result<Self, CreationError> {
        use self::password::Create;
        let password = Password::create(&password)?;

        Ok(NewLocalAuth {
            password: password,
            created_at: Utc::now(),
            user_id: user.id,
        })
    }
}
