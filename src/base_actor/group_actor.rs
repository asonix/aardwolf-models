use base_actor::BaseActor;
use base_actor::group::Group;
use schema::group_actors;

#[derive(Debug, Queryable)]
pub struct GroupActor {
    id: i32,
    group_id: i32,      // foreign key to Group
    base_actor_id: i32, // foriegn key to BaseActor
}

impl GroupActor {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn group_id(&self) -> i32 {
        self.group_id
    }

    pub fn base_actor_id(&self) -> i32 {
        self.base_actor_id
    }
}

#[derive(Debug, Insertable)]
#[table_name = "group_actors"]
pub struct NewGroupActor {
    group_id: i32,
    base_actor_id: i32,
}

impl NewGroupActor {
    pub fn new(group: &Group, base_actor: &BaseActor) -> Self {
        NewGroupActor {
            group_id: group.id(),
            base_actor_id: base_actor.id(),
        }
    }
}
