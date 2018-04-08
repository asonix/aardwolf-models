use diesel;
use diesel::connection::Connection;
use diesel::pg::PgConnection;
use chrono::DateTime;
use chrono::offset::Utc;

pub mod email;
pub mod local_auth;
pub mod role;

use schema::users;
use self::email::{EmailVerificationToken, UnverifiedEmail, VerifiedEmail, VerifyEmail};
use self::local_auth::LocalAuth;
pub use self::local_auth::{PlaintextPassword, VerificationError};

pub trait UserLike {
    fn id(&self) -> i32;
    fn primary_email(&self) -> Option<i32>;
    fn created_at(&self) -> DateTime<Utc>;

    fn can_post(&self, conn: &PgConnection) -> Result<bool, diesel::result::Error> {
        self.has_permission("make-post", conn)
    }

    fn can_follow(&self, conn: &PgConnection) -> Result<bool, diesel::result::Error> {
        self.has_permission("follow-user", conn)
    }

    fn is_verified(&self, conn: &PgConnection) -> Result<bool, diesel::result::Error> {
        self.has_role("verified", conn)
    }

    fn is_moderator(&self, conn: &PgConnection) -> Result<bool, diesel::result::Error> {
        self.has_role("moderator", conn)
    }

    fn is_admin(&self, conn: &PgConnection) -> Result<bool, diesel::result::Error> {
        self.has_role("admin", conn)
    }

    fn has_role(&self, name: &str, conn: &PgConnection) -> Result<bool, diesel::result::Error> {
        use schema::{roles, user_roles};
        use diesel::prelude::*;

        roles::dsl::roles
            .inner_join(user_roles::dsl::user_roles)
            .filter(user_roles::dsl::user_id.eq(self.id()))
            .filter(roles::dsl::name.eq(name))
            .count()
            .get_result(conn)
            .map(|count: i64| count > 0)
    }

    fn has_permission(
        &self,
        name: &str,
        conn: &PgConnection,
    ) -> Result<bool, diesel::result::Error> {
        use schema::{permissions, role_permissions, roles, user_roles};
        use diesel::prelude::*;

        roles::dsl::roles
            .inner_join(user_roles::dsl::user_roles)
            .inner_join(role_permissions::dsl::role_permissions)
            .inner_join(
                permissions::dsl::permissions
                    .on(role_permissions::dsl::permission_id.eq(permissions::dsl::id)),
            )
            .filter(user_roles::dsl::user_id.eq(self.id()))
            .filter(permissions::dsl::name.eq(name))
            .count()
            .get_result(conn)
            .map(|count: i64| count > 0)
    }
}

fn grant_role<U: UserLike>(
    user: &U,
    role: &str,
    conn: &PgConnection,
) -> Result<(), diesel::result::Error> {
    use schema::{roles, user_roles};
    use diesel::prelude::*;

    if user.has_role(role, conn)? {
        return Ok(());
    }

    roles::table
        .filter(roles::dsl::name.eq(role))
        .select(roles::dsl::id)
        .get_result(conn)
        .and_then(|role_id: i32| {
            diesel::insert_into(user_roles::table)
                .values((
                    user_roles::dsl::user_id.eq(user.id()),
                    user_roles::dsl::role_id.eq(role_id),
                    user_roles::dsl::created_at.eq(Utc::now()),
                ))
                .execute(conn)
                .map(|_| ())
        })
}

pub struct AdminUser {
    id: i32,
    primary_email: Option<i32>,
    created_at: DateTime<Utc>,
}

impl AdminUser {
    pub fn grant_role<U: UserLike>(
        &self,
        user: &U,
        role: &str,
        conn: &PgConnection,
    ) -> Result<(), diesel::result::Error> {
        grant_role(user, role, conn)
    }
}

impl UserLike for AdminUser {
    fn id(&self) -> i32 {
        self.id
    }

    fn primary_email(&self) -> Option<i32> {
        self.primary_email
    }

    fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
}

#[derive(Debug, Fail)]
pub enum UserVerifyError {
    #[fail(display = "Error in diesel: {}", _0)]
    Diesel(#[cause] diesel::result::Error),
    #[fail(display = "Cannot verify user with other user's ID")]
    IdMismatch,
}

impl From<diesel::result::Error> for UserVerifyError {
    fn from(e: diesel::result::Error) -> Self {
        UserVerifyError::Diesel(e)
    }
}

#[derive(Identifiable)]
#[table_name = "users"]
pub struct AuthenticatedUser {
    id: i32,
    primary_email: Option<i32>,
    created_at: DateTime<Utc>,
}

impl UserLike for AuthenticatedUser {
    fn id(&self) -> i32 {
        self.id
    }

    fn primary_email(&self) -> Option<i32> {
        self.primary_email
    }

    fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
}

impl AuthenticatedUser {
    pub fn set_default_email(
        &mut self,
        email: &VerifiedEmail,
        conn: &PgConnection,
    ) -> Result<(), diesel::result::Error> {
        use diesel::prelude::*;

        diesel::update(&*self)
            .set(users::primary_email.eq(Some(email.id())))
            .execute(conn)
            .map(|_| {
                self.primary_email = Some(email.id());
                ()
            })
    }

    pub fn upgrade_if_admin(
        self,
        conn: &PgConnection,
    ) -> Result<Result<AdminUser, AuthenticatedUser>, diesel::result::Error> {
        self.is_admin(conn).map(|is_admin| {
            if is_admin {
                Ok(AdminUser {
                    id: self.id,
                    primary_email: self.primary_email,
                    created_at: self.created_at,
                })
            } else {
                Err(self)
            }
        })
    }

    fn verify(&self, email: &VerifiedEmail, conn: &PgConnection) -> Result<(), UserVerifyError> {
        if self.id != email.user_id() {
            return Err(UserVerifyError::IdMismatch);
        }

        grant_role(self, "verified", conn).map_err(From::from)
    }
}

pub struct MemVerified {
    email: VerifyEmail,
    user: AuthenticatedUser,
}

impl MemVerified {
    pub fn store_verify(
        self,
        conn: &PgConnection,
    ) -> Result<(AuthenticatedUser, VerifiedEmail), UserVerifyError> {
        conn.transaction(|| {
            let MemVerified { email, mut user } = self;

            email
                .store_verify(conn)
                .map_err(From::from)
                .and_then(|verified_email| {
                    user.verify(&verified_email, conn).and_then(|_| {
                        user.set_default_email(&verified_email, conn)
                            .map(|_| (user, verified_email))
                            .map_err(From::from)
                    })
                })
        })
    }
}

pub struct UnverifiedUser {
    id: i32,
    created_at: DateTime<Utc>,
}

impl UserLike for UnverifiedUser {
    fn id(&self) -> i32 {
        self.id
    }

    fn primary_email(&self) -> Option<i32> {
        None
    }

    fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
}

impl UnverifiedUser {
    pub fn verify(
        self,
        email: UnverifiedEmail,
        token: EmailVerificationToken,
    ) -> Result<MemVerified, email::VerificationError> {
        email
            .verify_and_log_in(self, token)
            .map(|(user, email)| MemVerified { email, user })
    }
}

#[derive(Debug, Queryable)]
pub struct QueriedUser {
    id: i32,
    created_at: DateTime<Utc>,
    primary_email: Option<i32>,
}

impl UserLike for QueriedUser {
    fn id(&self) -> i32 {
        self.id
    }

    fn primary_email(&self) -> Option<i32> {
        self.primary_email
    }

    fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
}

#[derive(Debug, Queryable)]
pub struct UnauthenticatedUser {
    id: i32,
    created_at: DateTime<Utc>,
    primary_email: Option<i32>,
}

impl UnauthenticatedUser {
    pub fn log_in_local(
        self,
        local_auth: LocalAuth,
        password: PlaintextPassword,
    ) -> Result<AuthenticatedUser, VerificationError> {
        local_auth.log_in(self, password)
    }

    pub fn to_verified(
        self,
        conn: &PgConnection,
    ) -> Result<Result<UnauthenticatedUser, UnverifiedUser>, diesel::result::Error> {
        self.has_role("verified", conn).map(|has_role| {
            if has_role {
                Ok(self)
            } else {
                Err(UnverifiedUser {
                    id: self.id,
                    created_at: self.created_at,
                })
            }
        })
    }
}

impl UserLike for UnauthenticatedUser {
    fn id(&self) -> i32 {
        self.id
    }

    fn primary_email(&self) -> Option<i32> {
        self.primary_email
    }

    fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    created_at: DateTime<Utc>,
    primary_email: Option<i32>,
}

impl NewUser {
    pub fn new() -> Self {
        NewUser {
            created_at: Utc::now(),
            primary_email: None,
        }
    }
}
