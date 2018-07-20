//  main.rs
//
//  Main linker for all the external crates and such. Server logic
//  is handled over in app.rs for brevity's sake.
//
//  @author Ryan McGrath <ryan@rymc.io>
//  @created 06/02/2018

extern crate uuid;
extern crate futures;
extern crate dotenv;
extern crate sentry;
extern crate chrono;
extern crate actix;
extern crate num_cpus;
extern crate actix_web;
extern crate validator;
extern crate env_logger;
extern crate actix_redis;
extern crate sentry_actix;
extern crate djangohashers;
#[macro_use] extern crate log;
#[macro_use] extern crate askama;
extern crate serde_json;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate validator_derive;
#[macro_use] extern crate diesel;

use std::env;
use dotenv::dotenv;
use actix::prelude::*;
use actix_web::{server, App, middleware};
use sentry_actix::SentryMiddleware;

pub mod schema;
pub mod users;
pub mod util;
pub mod pages;
pub mod emails;

pub struct State {
    pub db: Addr<Syn, util::database::Database>
}

fn main() {
    dotenv().ok();
    env::set_var("RUST_BACKTRACE", "1");
    env::set_var("RUST_LOG", "actix_web=debug,info,warn");
    env_logger::init();

    let _sentry;
    if let Ok(dsn) = env::var("SENTRY_DSN") {
        _sentry = sentry::init(dsn);
        sentry::integrations::panic::register_panic_handler();
    }
    
    let address = env::var("BIND_TO").expect("BIND_TO not set!");
    let sys = System::new("user-auth-demo");
    /*actix::Arbiter::handle().spawn({
        let postmark = emails::Postmark::new(env::var("POSTMARK").expect("No Postmark API key set!");
        postmark.send("Testing", "123?", "ryan@rymc.io", "ryan@rymc.io").map_err(|e| {
            println!("Error? {:?}", e);
        }).and_then(|response| {
            println!("Response: {:?}", response);
            Ok(())
        })
    });*/

    let pool = util::database::pool();
    //let addr = SyncArbiter::start(num_cpus::get() * 3, move || database::Database(pool.clone()));
    let addr = SyncArbiter::start(12, move || util::database::Database(pool.clone()));

    server::new(move || {
        let mut app = App::with_state(State {
            db: addr.clone()
        });

        app = app.middleware(SentryMiddleware::new());
        app = app.middleware(middleware::Logger::default());        

        app = users::configure(app);
        app = pages::configure(app);

        app
    }).backlog(8192).workers(4).bind(&address).unwrap().start();

    let _ = sys.run();
}
