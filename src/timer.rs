use chrono::DateTime;
use chrono::offset::Utc;

use schema::timers;

#[derive(Queryable)]
pub struct Timer {
    id: i32,
    fire_time: DateTime<Utc>,
}

impl Timer {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn fire_time(&self) -> DateTime<Utc> {
        self.fire_time
    }
}

#[derive(Insertable)]
#[table_name = "timers"]
pub struct NewTimer {
    fire_time: DateTime<Utc>,
}

impl NewTimer {
    pub fn new(fire_time: DateTime<Utc>) -> Self {
        NewTimer { fire_time }
    }
}
