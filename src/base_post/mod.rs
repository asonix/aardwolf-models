use serde_json::Value;

pub mod post;
pub mod reaction;

use base_actor::BaseActor;
use file::image::Image;
use schema::base_posts;
use sql_types::PostVisibility;

#[derive(Debug, Identifiable, Queryable)]
#[table_name = "base_posts"]
pub struct BasePost {
    id: i32,
    name: Option<String>,       // max_length: 140
    media_type: Option<String>, // max_length: 80
    posted_by: Option<i32>,     // foreign key to BaseActor
    icon: Option<i32>,          // foreign key to Image
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

    pub fn media_type(&self) -> Option<&str> {
        self.media_type.as_ref().map(|s| s.as_ref())
    }

    pub fn posted_by(&self) -> Option<i32> {
        self.posted_by
    }

    pub fn icon(&self) -> Option<i32> {
        self.icon
    }

    pub fn original_json(&self) -> &Value {
        &self.original_json
    }
}

#[derive(Insertable)]
#[table_name = "base_posts"]
pub struct NewBasePost {
    name: Option<String>,
    media_type: Option<String>,
    posted_by: Option<i32>,
    icon: Option<i32>,
    visibility: PostVisibility,
    original_json: Value,
}

impl NewBasePost {
    pub fn new(
        name: Option<String>,
        media_type: Option<String>,
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
