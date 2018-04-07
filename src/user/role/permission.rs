use chrono::DateTime;
use chrono::offset::Utc;

use schema::permissions;

#[derive(Debug, Identifiable, Queryable)]
#[table_name = "permissions"]
pub struct Permission {
    id: i32,
    name: String,
    created_at: DateTime<Utc>,
}

impl Permission {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
}
