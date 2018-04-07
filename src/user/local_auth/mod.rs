use chrono::DateTime;
use chrono::offset::Utc;

mod password;

use self::password::{CreationError, Password};
pub use self::password::{PlaintextPassword, ValidationError, VerificationError};
use schema::local_auth;
use user::{AuthenticatedUser, UnAuthenticatedUser};

/// `LocalAuth` can be queried from the database, but is only really usable as a tool to "log in" a
/// user.
#[derive(Debug, Identifiable, Queryable)]
#[table_name = "local_auth"]
pub struct LocalAuth {
    id: i32,
    password: Password,
    user_id: i32, // foreign key to User
    created_at: DateTime<Utc>,
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

    /// Log In a user, given an `UnAuthenticatedUser` and a `PlaintextPassword`.
    ///
    /// This method ensures first that the `UnAuthenticatedUser` is the same user that this
    /// `LocalAuth` is associated with, and then continues to verify the `PlaintextPassword`
    /// against this type's `Password`. Upon succesful password verification, an
    /// `AuthenticatedUser` is created.
    pub(crate) fn log_in(
        self,
        user: UnAuthenticatedUser,
        password: PlaintextPassword,
    ) -> Result<AuthenticatedUser, VerificationError> {
        use self::password::Verify;

        if self.user_id != user.id {
            return Err(VerificationError::Process);
        }

        self.password.verify(password).map(|_| AuthenticatedUser {
            id: user.id,
            primary_email: user.primary_email,
            created_at: user.created_at,
        })
    }
}

/// This type exists to create new `LocalAuth` record in the database.
#[derive(Insertable)]
#[table_name = "local_auth"]
pub struct NewLocalAuth {
    password: Password,
    created_at: DateTime<Utc>,
    user_id: i32,
}

impl NewLocalAuth {
    /// Create a `NewLocalAuth`
    pub fn new(
        user: &UnAuthenticatedUser,
        password: PlaintextPassword,
    ) -> Result<Self, CreationError> {
        use self::password::Validate;

        let password = password.validate()?;

        NewLocalAuth::create(user, password)
    }

    /// Create a `NewLocalAuth` with a redundant password to check for consistency.
    pub fn new_from_two(
        user: &UnAuthenticatedUser,
        password: PlaintextPassword,
        password2: PlaintextPassword,
    ) -> Result<Self, CreationError> {
        use self::password::Validate;

        let password = password
            .validate()
            .and_then(|password| password.compare(password2))?;

        NewLocalAuth::create(user, password)
    }

    fn create(
        user: &UnAuthenticatedUser,
        password: PlaintextPassword,
    ) -> Result<Self, CreationError> {
        use self::password::Create;
        let password = Password::create(password)?;

        Ok(NewLocalAuth {
            password: password,
            created_at: Utc::now(),
            user_id: user.id,
        })
    }
}
