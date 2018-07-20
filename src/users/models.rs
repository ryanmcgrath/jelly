//  src/users/models.rs
//
//  Implements a basic User model, with support for creating/updating/deleting
//  users, along with welcome email and verification.
//
//  @author Ryan McGrath <ryan@rymc.io>
//  @created 06/18/2018

use chrono;
use diesel;
use diesel::prelude::*;
use actix::prelude::*;
use validator::Validate;

use schema::users;
use util::database::Database;

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i32,
    pub name: Option<String>,
    pub email: String,
    pub password: String,
    pub avatar: Option<String>,
    pub is_verified: bool,
    pub has_verified_email: bool,
    pub created: chrono::NaiveDateTime,
    pub updated: chrono::NaiveDateTime
}

#[derive(Insertable, Validate, Deserialize, Serialize, Debug)]
#[table_name="users"]
pub struct NewUser {
    #[validate(email(message="Hmmm, invalid email provided."))]
    pub email: String,

    pub password: String
}

impl Message for NewUser {
    type Result = Result<User, diesel::result::Error>;
}

impl Handler<NewUser> for Database {
    type Result = Result<User, diesel::result::Error>;

    fn handle(&mut self, msg: NewUser, _: &mut Self::Context) -> Self::Result {
        use schema::users::dsl::*;
        let conn = self.0.get().unwrap();
        diesel::insert_into(users).values(&msg).get_result::<User>(&conn)
    }
}

#[derive(Deserialize, Debug)]
pub struct UserLookup {
    pub id: i32
}

impl Message for UserLookup {
    type Result = Result<User, diesel::result::Error>;
}

impl Handler<UserLookup> for Database {
    type Result = Result<User, diesel::result::Error>;

    fn handle(&mut self, msg: UserLookup, _: &mut Self::Context) -> Self::Result {
        use schema::users::dsl::*;
        let conn = self.0.get().unwrap();
        users.filter(id.eq(msg.id)).get_result::<User>(&conn)
    }
}

#[derive(Deserialize, Validate, Serialize, Debug)]
pub struct UserLogin {
    #[validate(email(message="Hmmm, invalid email provided."))]
    pub email: String,
    pub password: String
}

impl Message for UserLogin {
    type Result = Result<User, diesel::result::Error>;
}

impl Handler<UserLogin> for Database {
    type Result = Result<User, diesel::result::Error>;

    fn handle(&mut self, msg: UserLogin, _: &mut Self::Context) -> Self::Result {
        use schema::users::dsl::*;
        let conn = self.0.get().unwrap();
        users.filter(email.eq(msg.email)).get_result::<User>(&conn)
    }
}

