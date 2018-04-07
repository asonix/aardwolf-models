extern crate aardwolf_models;
extern crate diesel;
extern crate dotenv;
extern crate env_logger;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use std::env;

use aardwolf_models::user::email::{Email, NewEmail};
use aardwolf_models::user::{NewUser, UnAuthenticatedUser};
use aardwolf_models::user::local_auth::{LocalAuth, NewLocalAuth};
use aardwolf_models::user::local_auth::PlaintextPassword;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;

#[derive(Deserialize)]
struct Payload {
    email: String,
    password: PlaintextPassword,
    confirm_password: PlaintextPassword,
}

#[derive(Deserialize)]
struct AuthPayload {
    email: String,
    password: PlaintextPassword,
}

fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("TEST_DATABASE_URL").unwrap();

    PgConnection::establish(&database_url).unwrap()
}

fn insert_user(new_user: NewUser, connection: &PgConnection) -> UnAuthenticatedUser {
    use aardwolf_models::schema::users;

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(connection)
        .unwrap()
}

fn insert_auth(new_local_auth: NewLocalAuth, connection: &PgConnection) -> LocalAuth {
    use aardwolf_models::schema::local_auth;

    diesel::insert_into(local_auth::table)
        .values(&new_local_auth)
        .get_result(connection)
        .unwrap()
}

fn insert_email(new_email: NewEmail, connection: &PgConnection) -> Email {
    use aardwolf_models::schema::emails;

    diesel::insert_into(emails::table)
        .values(&new_email)
        .get_result(connection)
        .unwrap()
}

fn lookup_user_by_email(
    email: String,
    connection: &PgConnection,
) -> (UnAuthenticatedUser, Email, LocalAuth) {
    use aardwolf_models::schema::{emails, local_auth, users};

    users::dsl::users
        .inner_join(emails::dsl::emails)
        .inner_join(local_auth::dsl::local_auth)
        .filter(emails::dsl::email.eq(email))
        .first(connection)
        .unwrap()
}

fn main() {
    env::set_var("RUST_LOG", "aardwolf_models=debug");
    env_logger::init();

    let connection = establish_connection();

    connection.test_transaction::<(), diesel::result::Error, _>(|| {
        {
            let json = json!({
                "email": "test@example.com",
                "password": "testpass",
                "confirm_password": "testpass",
            });

            let payload: Payload = serde_json::from_value(json).unwrap();

            let user = insert_user(NewUser::new(payload.email.clone()), &connection);

            insert_auth(
                NewLocalAuth::new_from_two(&user, payload.password, payload.confirm_password)
                    .unwrap(),
                &connection,
            );

            insert_email(NewEmail::new(payload.email, &user), &connection);

            println!("Created user, local_auth, and email!");
        }

        {
            let json = json!({
                "email": "test@example.com",
                "password": "testpass",
            });

            let payload: AuthPayload = serde_json::from_value(json).unwrap();

            let (unauthenticated_user, _email, local_auth) =
                lookup_user_by_email(payload.email, &connection);

            unauthenticated_user
                .log_in_local(local_auth, payload.password)
                .unwrap();

            println!("Logged in User!!!");
        }

        Ok(())
    });

    println!("Hewwo, Mr Obama???");
}
