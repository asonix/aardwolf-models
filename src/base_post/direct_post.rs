use base_post::BasePost;
use base_actor::BaseActor;
use schema::direct_posts;

#[derive(Debug, Queryable)]
pub struct DirectPost {
    id: i32,
    base_post_id: i32,  // foreign key to BasePost
    base_actor_id: i32, // foreign key to BaseActor
}

impl DirectPost {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn base_post_id(&self) -> i32 {
        self.base_post_id
    }

    pub fn base_actor_id(&self) -> i32 {
        self.base_actor_id
    }
}

#[derive(Debug, Insertable)]
#[table_name = "direct_posts"]
pub struct NewDirectPost {
    base_post_id: i32,
    base_actor_id: i32,
}

impl NewDirectPost {
    pub fn new(post: &BasePost, actor: &BaseActor) -> Self {
        NewDirectPost {
            base_post_id: post.id(),
            base_actor_id: actor.id(),
        }
    }
}
