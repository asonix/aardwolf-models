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

use aardwolf_models::user::email::{Email, EmailVerificationToken, NewEmail, UnverifiedEmail};
use aardwolf_models::user::{NewUser, UnauthenticatedUser};
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

#[derive(Deserialize)]
struct VerificationPayload {
    email: String,
    token: EmailVerificationToken,
}

fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("TEST_DATABASE_URL").unwrap();

    PgConnection::establish(&database_url).unwrap()
}

fn insert_user(new_user: NewUser, connection: &PgConnection) -> UnauthenticatedUser {
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

fn insert_email(new_email: NewEmail, connection: &PgConnection) -> UnverifiedEmail {
    use aardwolf_models::schema::emails;

    diesel::insert_into(emails::table)
        .values(&new_email)
        .get_result(connection)
        .unwrap()
}

fn lookup_user_by_email(
    email: String,
    connection: &PgConnection,
) -> (UnauthenticatedUser, Email, LocalAuth) {
    use aardwolf_models::schema::{emails, local_auth, users};

    users::dsl::users
        .inner_join(emails::dsl::emails.on(emails::dsl::user_id.eq(users::dsl::id)))
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
        // Create a user. Users are initially unverified
        let token = {
            let json = json!({
                "email": "test@example.com",
                "password": "testpass",
                "confirm_password": "testpass",
            });

            let payload: Payload = serde_json::from_value(json).unwrap();

            let user = match insert_user(NewUser::new(), &connection)
                .to_verified(&connection)
                .unwrap()
            {
                Ok(_) => panic!("Unexpected verified user"),
                Err(user) => user,
            };

            insert_auth(
                NewLocalAuth::new_from_two(&user, payload.password, payload.confirm_password)
                    .unwrap(),
                &connection,
            );

            let (new_email, token) = NewEmail::new(payload.email, &user).unwrap();

            insert_email(new_email, &connection);

            println!("Created user, local_auth, and email!");
            token
        };

        // Log in the unverified user
        {
            let json = json!({
                "email": "test@example.com",
                "password": "testpass",
            });

            let payload: AuthPayload = serde_json::from_value(json).unwrap();

            let (unauthenticated_user, _email, local_auth) =
                lookup_user_by_email(payload.email, &connection);

            let user = unauthenticated_user
                .log_in_local(local_auth, payload.password)
                .unwrap();

            assert!(
                !user.is_verified(&connection).unwrap(),
                "User shouldn't be verified at this point"
            );

            println!("Logged in unverified User!!!");
        }

        // Verify the user
        {
            let json = json!({
                "email":"test@example.com",
                "token":format!("{}", token)
            });

            let payload: VerificationPayload = serde_json::from_value(json).unwrap();

            let (unauthenticated_user, email, _local_auth) =
                lookup_user_by_email(payload.email, &connection);

            let unverified_user = unauthenticated_user
                .to_verified(&connection)
                .unwrap()
                .unwrap_err();

            let unverified_email = match email.to_verified() {
                Ok(_) => panic!("Unexpected verified email"),
                Err(unverified_email) => unverified_email,
            };

            let (_authenticated_user, _verified_email) = unverified_user
                .verify(unverified_email, payload.token)
                .unwrap()
                .store_verify(&connection)
                .unwrap();

            println!("Verified user!");
        }

        // log in the verified user
        {
            let json = json!({
                "email": "test@example.com",
                "password": "testpass",
            });

            let payload: AuthPayload = serde_json::from_value(json).unwrap();

            let (unauthenticated_user, _email, local_auth) =
                lookup_user_by_email(payload.email, &connection);

            let user = unauthenticated_user
                .log_in_local(local_auth, payload.password)
                .unwrap();

            assert!(
                user.is_verified(&connection).unwrap(),
                "User should be verified at this point"
            );

            println!("Logged in verified User!!!");
        }

        Ok(())
    });

    println!("Hewwo, Mr Obama???");
}
