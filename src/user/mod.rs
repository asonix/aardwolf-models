use diesel;
use diesel::pg::PgConnection;
use chrono::DateTime;
use chrono::offset::Utc;

pub mod email;
pub mod local_auth;
pub mod role;

use schema::users;
use self::email::VerifiedEmail;
use self::local_auth::LocalAuth;
pub use self::local_auth::{PlaintextPassword, VerificationError};

pub trait UserLike {
    fn id(&self) -> i32;
    fn primary_email(&self) -> Option<i32>;
    fn created_at(&self) -> DateTime<Utc>;
}

#[derive(Identifiable)]
#[table_name = "users"]
pub struct AdminUser {
    id: i32,
    primary_email: Option<i32>,
    created_at: DateTime<Utc>,
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

    pub fn verify(
        &self,
        email: &VerifiedEmail,
        conn: &PgConnection,
    ) -> Result<(), diesel::result::Error> {
        use schema::{roles, user_roles};
        use diesel::prelude::*;

        roles::table
            .filter(roles::dsl::name.eq("verified"))
            .select(roles::dsl::id)
            .get_result(conn)
            .and_then(|role_id: i32| {
                diesel::insert_into(user_roles::table)
                    .values((
                        user_roles::dsl::user_id.eq(email.user_id()),
                        user_roles::dsl::role_id.eq(role_id),
                        user_roles::dsl::created_at.eq(Utc::now()),
                    ))
                    .execute(conn)
                    .map(|_| ())
            })
    }

    pub fn is_admin(
        self,
        conn: &PgConnection,
    ) -> Result<Result<AdminUser, AuthenticatedUser>, diesel::result::Error> {
        use schema::{permissions, role_permissions, roles, user_roles};
        use diesel::prelude::*;

        role_permissions::dsl::role_permissions
            .inner_join(roles::dsl::roles)
            .inner_join(permissions::dsl::permissions)
            .inner_join(user_roles::dsl::user_roles.on(roles::dsl::id.eq(user_roles::dsl::role_id)))
            .filter(permissions::dsl::name.eq("admin"))
            .filter(user_roles::dsl::user_id.eq(self.id))
            .count()
            .get_result(conn)
            .map(|num_admin: i64| {
                if num_admin == 0 {
                    Err(self)
                } else {
                    Ok(AdminUser {
                        id: self.id,
                        primary_email: self.primary_email,
                        created_at: self.created_at,
                    })
                }
            })
    }

    pub fn is_verified(&self, conn: &PgConnection) -> Result<bool, diesel::result::Error> {
        use schema::{roles, user_roles};
        use diesel::prelude::*;

        roles::dsl::roles
            .inner_join(user_roles::dsl::user_roles)
            .filter(roles::dsl::name.eq("verified"))
            .filter(user_roles::dsl::user_id.eq(self.id))
            .count()
            .get_result(conn)
            .map(|num_verified: i64| num_verified > 0)
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
        use schema::{permissions, role_permissions, roles, user_roles};
        use diesel::prelude::*;

        role_permissions::dsl::role_permissions
            .inner_join(roles::dsl::roles)
            .inner_join(permissions::dsl::permissions)
            .inner_join(user_roles::dsl::user_roles.on(roles::dsl::id.eq(user_roles::dsl::role_id)))
            .filter(permissions::dsl::name.eq("verified"))
            .filter(user_roles::dsl::user_id.eq(self.id))
            .count()
            .get_result(conn)
            .map(|num_verified: i64| {
                if num_verified == 0 {
                    Err(UnverifiedUser {
                        id: self.id,
                        created_at: self.created_at,
                    })
                } else {
                    Ok(self)
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
