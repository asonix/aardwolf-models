use base_actor::BaseActor;
use schema::groups;

#[derive(Debug, Queryable)]
pub struct Group {
    id: i32,
    base_actor_id: i32,
}

impl Group {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn base_actor_id(&self) -> i32 {
        self.base_actor_id
    }
}

#[derive(Debug, Insertable)]
#[table_name = "groups"]
pub struct NewGroup {
    base_actor_id: i32,
}

impl NewGroup {
    pub fn new(actor: &BaseActor) -> Self {
        NewGroup {
            base_actor_id: actor.id(),
        }
    }
}
