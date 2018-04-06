use base_actor::BaseActor;
use schema::followers;

#[derive(Queryable)]
pub struct Follower {
    id: i32,
    follower: i32, // foreign key to BaseActor
    follows: i32,  // foreign key to BaseActor
}

impl Follower {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn follower(&self) -> i32 {
        self.follower
    }

    pub fn follows(&self) -> i32 {
        self.follows
    }
}

#[derive(Insertable)]
#[table_name = "followers"]
pub struct NewFollower {
    follower: i32,
    follows: i32,
}

impl NewFollower {
    pub fn new(follower: &BaseActor, follows: &BaseActor) -> Self {
        NewFollower {
            follower: follower.id(),
            follows: follows.id(),
        }
    }
}
