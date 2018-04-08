use chrono::DateTime;
use chrono::offset::Utc;
use diesel;
use diesel::connection::Connection;
use diesel::pg::PgConnection;

pub mod email;
pub mod local_auth;
mod permissions;
pub mod role;

use base_actor::BaseActor;
use schema::users;
use self::email::{EmailVerificationToken, UnverifiedEmail, VerifiedEmail, VerifyEmail};
use self::local_auth::LocalAuth;
pub use self::local_auth::{PlaintextPassword, VerificationError};

pub trait UserLike {
    fn id(&self) -> i32;
    fn primary_email(&self) -> Option<i32>;
    fn created_at(&self) -> DateTime<Utc>;

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
}

#[derive(Debug, Fail)]
pub enum PermissionError {
    #[fail(display = "Failed to check user's permission")]
    Diesel(diesel::result::Error),
    #[fail(display = "User doesn't have this permission")]
    Permission,
}

impl From<diesel::result::Error> for PermissionError {
    fn from(e: diesel::result::Error) -> Self {
        PermissionError::Diesel(e)
    }
}

pub type PermissionResult<T> = Result<T, PermissionError>;

/// Define things a logged-in user is allowed to do.
///
/// The end-goal for this trait is to produce types like `PostCreator`, `UserFollower`, and
/// `InstanceConfigurator`. These types would *only* be producable through this trait, and would be
/// the only ways to perform the actions associated with the permission they came from.
///
/// This way, permission checking would be enforced by the compiler, since "making a post" or
/// "configuring the instance" would not be possible without calling these methods.
pub trait AuthenticatedUserLike: UserLike {
    fn can_post<'a>(
        &self,
        base_actor: &'a BaseActor,
        conn: &PgConnection,
    ) -> PermissionResult<permissions::PostMaker<'a>> {
        if self.is_actor(base_actor) {
            self.has_permission("make-post", conn)
                .map(|_| permissions::PostMaker::new(base_actor))
        } else {
            Err(PermissionError::Permission)
        }
    }

    fn can_post_media<'a>(
        &self,
        base_actor: &'a BaseActor,
        conn: &PgConnection,
    ) -> PermissionResult<permissions::MediaPostMaker<'a>> {
        if self.is_actor(base_actor) {
            self.has_permission("make-media-post", conn)
                .map(|_| permissions::MediaPostMaker::new(base_actor))
        } else {
            Err(PermissionError::Permission)
        }
    }

    /// TODO: Maybe do more verification here. Is this actor allowed to comment on this post?
    ///
    /// check the target post's visibility,
    /// check whether user follows target post's author,
    /// check whether parent post is in the same thread as conversation post
    fn can_post_comment<'a>(
        &self,
        base_actor: &'a BaseActor,
        conn: &PgConnection,
    ) -> PermissionResult<permissions::CommentMaker<'a>> {
        if self.is_actor(base_actor) {
            self.has_permission("make-comment", conn)
                .map(|_| permissions::CommentMaker::new(base_actor))
        } else {
            Err(PermissionError::Permission)
        }
    }

    fn can_follow<'a>(
        &self,
        base_actor: &'a BaseActor,
        conn: &PgConnection,
    ) -> PermissionResult<permissions::ActorFollower<'a>> {
        if self.is_actor(base_actor) {
            self.has_permission("follow-user", conn)
                .map(|_| permissions::ActorFollower::new(base_actor))
        } else {
            Err(PermissionError::Permission)
        }
    }

    fn can_make_persona(&self, conn: &PgConnection) -> PermissionResult<()> {
        self.has_permission("make-persona", conn)
    }

    fn can_manage_follow_requests<'a>(
        &self,
        base_actor: &'a BaseActor,
        conn: &PgConnection,
    ) -> PermissionResult<permissions::FollowRequestManager<'a>> {
        if self.is_actor(base_actor) {
            self.has_permission("manage-follow-requests", conn)
                .map(|_| permissions::FollowRequestManager::new(base_actor))
        } else {
            Err(PermissionError::Permission)
        }
    }

    fn can_configure_instance(&self, conn: &PgConnection) -> PermissionResult<()> {
        self.has_permission("configure-instance", conn)
    }

    fn can_ban_user(&self, conn: &PgConnection) -> PermissionResult<()> {
        self.has_permission("ban-user", conn)
    }

    fn can_block_instance(&self, conn: &PgConnection) -> PermissionResult<()> {
        self.has_permission("block-instance", conn)
    }

    fn can_grant_role(&self, conn: &PgConnection) -> PermissionResult<permissions::RoleGranter> {
        self.has_permission("grant-role", conn)
            .map(|_| permissions::RoleGranter::new())
    }

    fn can_revoke_role(&self, conn: &PgConnection) -> PermissionResult<permissions::RoleRevoker> {
        self.has_permission("revoke-role", conn)
            .map(|_| permissions::RoleRevoker::new())
    }

    fn is_actor(&self, base_actor: &BaseActor) -> bool {
        base_actor
            .local_user()
            .map(|id| id == self.id())
            .unwrap_or(false)
    }

    fn has_permission(&self, name: &str, conn: &PgConnection) -> PermissionResult<()> {
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
            .map_err(From::from)
            .and_then(|count: i64| {
                if count > 0 {
                    Ok(())
                } else {
                    Err(PermissionError::Permission)
                }
            })
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

impl From<UpdateFieldError> for UserVerifyError {
    fn from(e: UpdateFieldError) -> Self {
        match e {
            UpdateFieldError::Diesel(d) => UserVerifyError::Diesel(d),
            UpdateFieldError::Relation => UserVerifyError::IdMismatch,
        }
    }
}

#[derive(Debug, Fail)]
pub enum UpdateFieldError {
    #[fail(display = "Error updating record: {}", _0)]
    Diesel(#[cause] diesel::result::Error),
    #[fail(display = "Provided records are not related")]
    Relation,
}

impl From<diesel::result::Error> for UpdateFieldError {
    fn from(e: diesel::result::Error) -> Self {
        UpdateFieldError::Diesel(e)
    }
}

#[derive(Identifiable)]
#[table_name = "users"]
pub struct AuthenticatedUser {
    id: i32,
    primary_email: Option<i32>,
    created_at: DateTime<Utc>,
}

impl AuthenticatedUser {
    pub fn set_default_email(
        &mut self,
        email: &VerifiedEmail,
        conn: &PgConnection,
    ) -> Result<(), UpdateFieldError> {
        if email.user_id() != self.id {
            return Err(UpdateFieldError::Relation);
        }

        use diesel::prelude::*;

        diesel::update(&*self)
            .set(users::primary_email.eq(Some(email.id())))
            .execute(conn)
            .map_err(From::from)
            .map(|_| {
                self.primary_email = Some(email.id());
                ()
            })
    }

    fn verify(&self, email: &VerifiedEmail, conn: &PgConnection) -> Result<(), UserVerifyError> {
        if self.id != email.user_id() {
            return Err(UserVerifyError::IdMismatch);
        }

        permissions::RoleGranter::new()
            .grant_role(self, "verified", conn)
            .map_err(From::from)
    }
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

impl AuthenticatedUserLike for AuthenticatedUser {}

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
        self.is_verified(conn).map(|has_role| {
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
