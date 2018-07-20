//  database.rs
//
//  Handles setting up database routines, state, and such
//  to work within actix-web.
//
//  @author Ryan McGrath <ryan@rymc.io>
//  @created 06/16/2018

use std::env;
use actix::prelude::*;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

pub fn pool() -> Pool<ConnectionManager<PgConnection>> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set!");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::new(manager).expect("Error creating Postgres connection pool!")
}

pub struct Database(pub Pool<ConnectionManager<PgConnection>>);

unsafe impl Send for Database {}

impl Actor for Database {
    type Context = SyncContext<Self>;
}
