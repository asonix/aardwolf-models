use chrono::offset::Utc;
use diesel;
use diesel::pg::PgConnection;
use serde_json::Value;

use file::File;
use file::image::Image;
use base_actor::BaseActor;
use base_post::{BasePost, NewBasePost};
use base_post::post::{NewPost, Post};
use base_post::post::media_post::{MediaPost, NewMediaPost};
use base_post::post::comment::{Comment, NewComment};
use sql_types::{Mime, PostVisibility};
use super::UserLike;

pub struct RoleGranter(());

impl RoleGranter {
    pub(crate) fn new() -> RoleGranter {
        RoleGranter(())
    }

    pub fn grant_role<U: super::UserLike>(
        &self,
        user: &U,
        role: &str,
        conn: &super::PgConnection,
    ) -> Result<(), super::diesel::result::Error> {
        grant_role(user, role, conn)
    }
}

pub struct RoleRevoker(());

impl RoleRevoker {
    pub(crate) fn new() -> RoleRevoker {
        RoleRevoker(())
    }

    pub fn revoke_role<U: super::UserLike>(
        &self,
        user: &U,
        role: &str,
        conn: &super::PgConnection,
    ) -> Result<(), super::diesel::result::Error> {
        revoke_role(user, role, conn)
    }
}

pub struct PostMaker<'a>(&'a BaseActor);

impl<'a> PostMaker<'a> {
    pub(crate) fn new(base_actor: &BaseActor) -> PostMaker {
        PostMaker(base_actor)
    }

    pub fn make_post(
        &self,
        name: Option<String>,
        media_type: Mime,
        icon: Option<&Image>,
        visibility: PostVisibility,
        original_json: Value,
        content: String,
        source: String,
        conn: &PgConnection,
    ) -> Result<(BasePost, Post), diesel::result::Error> {
        new_post(
            name,
            media_type,
            Some(self.0),
            icon,
            visibility,
            original_json,
            content,
            Some(source),
            conn,
        )
    }
}

pub struct MediaPostMaker<'a>(&'a BaseActor);

impl<'a> MediaPostMaker<'a> {
    pub(crate) fn new(base_actor: &BaseActor) -> MediaPostMaker {
        MediaPostMaker(base_actor)
    }

    pub fn make_media_post(
        &self,
        name: Option<String>,
        media_type: Mime,
        icon: Option<&Image>,
        visibility: PostVisibility,
        original_json: Value,
        content: String,
        source: String,
        media: &File,
        conn: &PgConnection,
    ) -> Result<(BasePost, Post, MediaPost), diesel::result::Error> {
        new_media_post(
            name,
            media_type,
            Some(self.0),
            icon,
            visibility,
            original_json,
            content,
            Some(source),
            media,
            conn,
        )
    }
}

pub struct CommentMaker<'a>(&'a BaseActor);

impl<'a> CommentMaker<'a> {
    pub(crate) fn new(base_actor: &BaseActor) -> CommentMaker {
        CommentMaker(base_actor)
    }

    pub fn make_comment(
        &self,
        name: Option<String>,
        media_type: Mime,
        icon: Option<&Image>,
        visibility: PostVisibility,
        original_json: Value,
        content: String,
        source: String,
        conversation: &Post,
        parent: &Post,
        conn: &PgConnection,
    ) -> Result<(BasePost, Post, Comment), diesel::result::Error> {
        new_comment(
            name,
            media_type,
            Some(self.0),
            icon,
            visibility,
            original_json,
            content,
            Some(source),
            conversation,
            parent,
            conn,
        )
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

fn revoke_role<U: UserLike>(
    user: &U,
    role: &str,
    conn: &PgConnection,
) -> Result<(), diesel::result::Error> {
    use schema::{roles, user_roles};
    use diesel::prelude::*;

    if !user.has_role(role, conn)? {
        return Ok(());
    }

    roles::table
        .filter(roles::dsl::name.eq(role))
        .select(roles::dsl::id)
        .get_result(conn)
        .and_then(|role_id: i32| {
            let user_role = user_roles::table
                .filter(user_roles::dsl::user_id.eq(user.id()))
                .filter(user_roles::dsl::role_id.eq(role_id));

            diesel::delete(user_role).execute(conn)
        })
        .map(|_| ())
}

fn new_media_post(
    name: Option<String>,
    media_type: Mime,
    posted_by: Option<&BaseActor>,
    icon: Option<&Image>,
    visibility: PostVisibility,
    original_json: Value,
    content: String,
    source: Option<String>,
    media: &File,
    conn: &PgConnection,
) -> Result<(BasePost, Post, MediaPost), diesel::result::Error> {
    use schema::media_posts;
    use diesel::prelude::*;

    conn.transaction(|| {
        new_post(
            name,
            media_type,
            posted_by,
            icon,
            visibility,
            original_json,
            content,
            source,
            conn,
        ).and_then(|(base_post, post)| {
            diesel::insert_into(media_posts::table)
                .values(&NewMediaPost::new(media, &post))
                .get_result(conn)
                .map(|media_post: MediaPost| (base_post, post, media_post))
        })
    })
}

fn new_comment(
    name: Option<String>,
    media_type: Mime,
    posted_by: Option<&BaseActor>,
    icon: Option<&Image>,
    visibility: PostVisibility,
    original_json: Value,
    content: String,
    source: Option<String>,
    conversation: &Post,
    parent: &Post,
    conn: &PgConnection,
) -> Result<(BasePost, Post, Comment), diesel::result::Error> {
    use schema::comments;
    use diesel::prelude::*;

    conn.transaction(|| {
        new_post(
            name,
            media_type,
            posted_by,
            icon,
            visibility,
            original_json,
            content,
            source,
            conn,
        ).and_then(|(base_post, post)| {
            diesel::insert_into(comments::table)
                .values(NewComment::new(conversation, parent, &post))
                .get_result(conn)
                .map(|comment: Comment| (base_post, post, comment))
        })
    })
}

fn new_post(
    name: Option<String>,
    media_type: Mime,
    posted_by: Option<&BaseActor>,
    icon: Option<&Image>,
    visibility: PostVisibility,
    original_json: Value,
    content: String,
    source: Option<String>,
    conn: &PgConnection,
) -> Result<(BasePost, Post), diesel::result::Error> {
    use schema::{base_posts, posts};
    use diesel::prelude::*;

    conn.transaction(|| {
        diesel::insert_into(base_posts::table)
            .values(&NewBasePost::new(
                name,
                media_type,
                posted_by,
                icon,
                visibility,
                original_json,
            ))
            .get_result(conn)
            .and_then(|base_post: BasePost| {
                diesel::insert_into(posts::table)
                    .values(&NewPost::new(content, source, &base_post))
                    .get_result(conn)
                    .map(|post: Post| (base_post, post))
            })
    })
}
