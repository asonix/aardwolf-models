use diesel;
use diesel::pg::PgConnection;
use serde_json::Value;

pub mod direct_post;
pub mod post;
pub mod reaction;

use base_actor::BaseActor;
use file::image::Image;
use schema::base_posts;
use self::direct_post::DirectPost;
use sql_types::{Mime, PostVisibility};

#[derive(Debug, Queryable)]
pub struct BasePost {
    id: i32,
    name: Option<String>, // max_length: 140
    media_type: Mime,     // max_length: 80
    posted_by: i32,       // foreign key to BaseActor
    icon: Option<i32>,    // foreign key to Image
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

    pub fn posted_by(&self) -> i32 {
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

    pub fn is_viewable_by(
        &self,
        base_actor: &BaseActor,
        conn: &PgConnection,
    ) -> Result<bool, diesel::result::Error> {
        use schema::direct_posts;
        use diesel::prelude::*;

        direct_posts::table
            .filter(direct_posts::dsl::base_post_id.eq(self.id))
            .filter(direct_posts::dsl::base_actor_id.eq(base_actor.id()))
            .get_result(conn)
            .map(|_: DirectPost| true)
            .or_else(|e| match e {
                diesel::result::Error::NotFound => Ok(false),
                e => Err(e),
            })
    }
}

#[derive(Insertable)]
#[table_name = "base_posts"]
pub struct NewBasePost {
    name: Option<String>,
    media_type: Mime,
    posted_by: i32,
    icon: Option<i32>,
    visibility: PostVisibility,
    original_json: Value,
}

impl NewBasePost {
    pub fn new(
        name: Option<String>,
        media_type: Mime,
        posted_by: &BaseActor,
        icon: Option<&Image>,
        visibility: PostVisibility,
        original_json: Value,
    ) -> Self {
        NewBasePost {
            name,
            media_type,
            posted_by: posted_by.id(),
            icon: icon.map(|i| i.id()),
            visibility,
            original_json,
        }
    }
}
