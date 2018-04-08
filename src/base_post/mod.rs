use diesel;
use diesel::pg::PgConnection;
use serde_json::Value;

pub mod post;
pub mod reaction;

use base_actor::BaseActor;
use file::File;
use file::image::Image;
use schema::base_posts;
use self::post::{NewPost, Post};
use self::post::media_post::{MediaPost, NewMediaPost};
use self::post::comment::{Comment, NewComment};
use sql_types::{Mime, PostVisibility};

#[derive(Debug, Queryable)]
pub struct BasePost {
    id: i32,
    name: Option<String>,   // max_length: 140
    media_type: Mime,       // max_length: 80
    posted_by: Option<i32>, // foreign key to BaseActor
    icon: Option<i32>,      // foreign key to Image
    visibility: PostVisibility,
    original_json: Value, // original json
}

impl BasePost {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_ref().map(|s| s.as_ref())
    }

    pub fn media_type(&self) -> &Mime {
        &self.media_type
    }

    pub fn posted_by(&self) -> Option<i32> {
        self.posted_by
    }

    pub fn icon(&self) -> Option<i32> {
        self.icon
    }

    pub fn visibility(&self) -> PostVisibility {
        self.visibility
    }

    pub fn original_json(&self) -> &Value {
        &self.original_json
    }
}

pub fn new_media_post(
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

pub fn new_comment(
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

pub fn new_post(
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
    use schema::posts;
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

#[derive(Insertable)]
#[table_name = "base_posts"]
pub struct NewBasePost {
    name: Option<String>,
    media_type: Mime,
    posted_by: Option<i32>,
    icon: Option<i32>,
    visibility: PostVisibility,
    original_json: Value,
}

impl NewBasePost {
    pub fn new(
        name: Option<String>,
        media_type: Mime,
        posted_by: Option<&BaseActor>,
        icon: Option<&Image>,
        visibility: PostVisibility,
        original_json: Value,
    ) -> Self {
        NewBasePost {
            name,
            media_type,
            posted_by: posted_by.map(|pb| pb.id()),
            icon: icon.map(|i| i.id()),
            visibility,
            original_json,
        }
    }
}
