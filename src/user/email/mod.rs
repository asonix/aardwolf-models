mod token;

use diesel;
use diesel::pg::PgConnection;

use schema::emails;
use self::token::{create_token, CreationError, HashedEmailToken, VerificationError};
pub use self::token::{EmailToken, EmailVerificationToken};
use user::{AuthenticatedUser, UnverifiedUser, UserLike};

pub struct VerifiedEmail {
    id: i32,
    email: String,
    user_id: i32,
}

impl VerifiedEmail {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn email(&self) -> &str {
        &self.email
    }

    pub fn user_id(&self) -> i32 {
        self.user_id
    }
}

#[derive(Debug, Queryable)]
pub struct Email {
    id: i32,
    email: String,
    user_id: i32,
    verified: bool,
    verification_token: Option<HashedEmailToken>,
}

impl Email {
    pub fn to_verified(self) -> Result<VerifiedEmail, UnverifiedEmail> {
        if self.verified {
            Ok(VerifiedEmail {
                id: self.id,
                email: self.email,
                user_id: self.user_id,
            })
        } else {
            Err(UnverifiedEmail {
                id: self.id,
                email: self.email,
                user_id: self.user_id,
                verified: false,
                verification_token: self.verification_token,
            })
        }
    }
}

#[derive(AsChangeset)]
#[table_name = "emails"]
pub struct VerifyEmail {
    id: i32,
    email: String,
    user_id: i32,
    verified: bool,
    verification_token: Option<HashedEmailToken>,
}

impl VerifyEmail {
    pub fn store_verify(self, conn: &PgConnection) -> Result<VerifiedEmail, diesel::result::Error> {
        use schema::emails;
        use diesel::prelude::*;

        diesel::update(emails::table)
            .set(&self)
            .execute(conn)
            .map(|_| VerifiedEmail {
                id: self.id,
                email: self.email,
                user_id: self.user_id,
            })
    }
}

#[derive(Queryable)]
pub struct UnverifiedEmail {
    id: i32,
    email: String,
    user_id: i32, // foreign key to User
    verified: bool,
    verification_token: Option<HashedEmailToken>,
}

impl UnverifiedEmail {
    pub fn verify(
        self,
        user: UnverifiedUser,
        token: EmailVerificationToken,
    ) -> Result<(AuthenticatedUser, VerifyEmail), VerificationError> {
        if self.verification_token.is_some() && !self.verified {
            token::VerifyEmail::verify_email(self.verification_token.as_ref().unwrap(), token).map(
                |_| {
                    (
                        AuthenticatedUser {
                            id: user.id,
                            primary_email: None,
                            created_at: user.created_at,
                        },
                        VerifyEmail {
                            id: self.id,
                            email: self.email,
                            user_id: self.user_id,
                            verified: true,
                            verification_token: None,
                        },
                    )
                },
            )
        } else if !self.verified {
            Err(VerificationError::Process)
        } else {
            // TODO: don't error if email is already verified
            Err(VerificationError::Process)
        }
    }
}

#[derive(Insertable)]
#[table_name = "emails"]
pub struct NewEmail {
    email: String,
    user_id: i32,
    verified: bool,
    verification_token: HashedEmailToken,
}

impl NewEmail {
    pub fn new<U: UserLike>(email: String, user: &U) -> Result<(Self, EmailToken), CreationError> {
        create_token().map(|(email_token, verification_token)| {
            (
                NewEmail {
                    email,
                    user_id: user.id(),
                    verified: false,
                    verification_token,
                },
                email_token,
            )
        })
    }
}
