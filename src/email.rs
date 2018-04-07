use schema::emails;
use user::UserLike;

#[derive(Debug, Identifiable, Queryable)]
#[table_name = "emails"]
pub struct Email {
    id: i32,
    email: String,
    user_id: i32, // foreign key to User
}

impl Email {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn email(&self) -> &str {
        &self.email
    }

    pub fn user_id(&self) -> i32 {
        self.user_id
    }
}

#[derive(Insertable)]
#[table_name = "emails"]
pub struct NewEmail {
    email: String,
    user_id: i32,
}

impl NewEmail {
    pub fn new<U: UserLike>(email: String, user: &U) -> Self {
        NewEmail {
            email,
            user_id: user.id(),
        }
    }
}
