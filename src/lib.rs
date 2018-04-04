#[macro_use]
extern crate diesel;
extern crate serde_json;
extern crate url;

use std::path::PathBuf;

use serde_json::Value;
use url::Url;

mod schema;

#[derive(Queryable)]
struct File {
    id: i32,
    path: PathBuf,
}

#[derive(Queryable)]
struct Image {
    id: i32,
    width: u32,
    height: u32,
    file_id: i32, // foreign key to File
}

#[derive(Queryable)]
struct User {
    id: i32,
}

#[derive(Queryable)]
struct BaseActor {
    id: i32,
    display_name: String,    // max_length: 80
    profile_url: Url,        // max_length: 2048
    inbox_url: Url,          // max_length: 2048
    outbox_url: Url,         // max_length: 2048
    local_user: Option<i32>, // foreign key to User
    original_json: Value,    // original json
}

#[derive(Queryable)]
struct BasePost {
    id: i32,
    name: Option<String>,       // max_length: 140
    media_type: Option<String>, // max_length: 80
    posted_by: Option<i32>,     // foreign key to BaseActor
    icon: Option<i32>,          // foreign key to Image
    original_json: Value,       // original json
}

#[derive(Queryable)]
struct Followers {
    id: i32,
    follower: i32, // foreign key to BaseActor
    follows: i32,  // foreign key to BaseActor
}

enum Visibility {
    Pub,
    Fl,
    Mut,
    List,
}

#[derive(Queryable)]
struct Persona {
    id: i32,
    default_visibility: Visibility,
    is_searchable: bool,
    avatar: Option<i32>, // foreign key to Image
    shortname: String,   // wtf is a SlugField
    base_actor: i32,     // foreign key to BaseActor
}

enum Lang {
    En,
}

#[derive(Queryable)]
struct Link {
    id: i32,
    href: Url, // max_length: 2048
    href_lang: Lang,
    height: u32,
    width: u32,
    preview: String,
    base_post: i32, // foreign key to BasePost
}

#[derive(Queryable)]
struct Post {
    id: i32,
    content: String,
    source: Option<String>,
    base_post: i32, // foreign key to BasePost
}

#[derive(Queryable)]
struct MediaPost {
    id: i32,
    file_id: i32, // foreign key to File
    post_id: i32, // foreign key to Post
}

#[derive(Queryable)]
struct Comment {
    id: i32,
    conversation: i32, // foreign key to topic Post
    parent: i32,       // foreign key to replied Post
    post: i32,         // foreign key to Post
}

enum ReactionType {
    Like,
    Dislike,
    Seen,
}

#[derive(Queryable)]
struct Reaction {
    id: i32,
    reaction_type: ReactionType,
    comment: i32, // foreign key to Comment
}
