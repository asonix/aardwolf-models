use serde_json::Value;
use url::Url;

use schema::base_actors;
use user::UserLike;

#[derive(Debug, Identifiable, Queryable)]
#[table_name = "base_actors"]
pub struct BaseActor {
    id: i32,
    display_name: String,    // max_length: 80
    profile_url: Url,        // max_length: 2048
    inbox_url: Url,          // max_length: 2048
    outbox_url: Url,         // max_length: 2048
    local_user: Option<i32>, // foreign key to User
    original_json: Value,    // original json
}

impl BaseActor {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn display_name(&self) -> &str {
        &self.display_name
    }

    pub fn profile_url(&self) -> &Url {
        &self.profile_url
    }

    pub fn inbox_url(&self) -> &Url {
        &self.inbox_url
    }

    pub fn outbox_url(&self) -> &Url {
        &self.outbox_url
    }

    pub fn local_user(&self) -> Option<i32> {
        self.local_user
    }

    pub fn original_json(&self) -> &Value {
        &self.original_json
    }
}

#[derive(Insertable)]
#[table_name = "base_actors"]
pub struct NewBaseActor {
    display_name: String,
    profile_url: String,
    inbox_url: String,
    local_user: Option<i32>,
    original_json: Value,
}

impl NewBaseActor {
    pub fn new<U: UserLike>(
        display_name: String,
        profile_url: Url,
        inbox_url: Url,
        local_user: Option<&U>,
        original_json: Value,
    ) -> Self {
        NewBaseActor {
            display_name,
            profile_url: profile_url.into_string(),
            inbox_url: inbox_url.into_string(),
            local_user: local_user.map(|lu| lu.id()),
            original_json,
        }
    }
}
