//  users/mod.rs
//
//  URL dispatcher for user account related API endpoints.
//
//  @author Ryan McGrath <ryan@rymc.io>
//  @created 06/15/2018

use std::env;
use actix_redis::{RedisSessionBackend, SameSite};
use actix_web::{http::Method, App, middleware::session::SessionStorage};

use State;
pub mod views;
pub mod models;
pub mod middleware;

pub fn configure(application: App<State>) -> App<State> {
    let key = env::var("SECRET_KEY").expect("SECRET_KEY not set!");

    application.middleware(SessionStorage::new(
        RedisSessionBackend::new(env::var("REDIS").expect("REDIS not set!"), key.as_bytes())
            .cookie_name("sessionid")
            .cookie_secure(true)
            //.cookie_domain("your domain here")
            .cookie_path("/")
            .cookie_same_site(SameSite::Lax)
    )).scope("/users", |scope| {
        scope.resource("/signup/", |r| {
            r.method(Method::GET).with(views::Signup::get);
            r.method(Method::POST).with(views::Signup::post)
        }).resource("/login/", |r| {
            r.method(Method::GET).with(views::Login::get);
            r.method(Method::POST).with(views::Login::post)
        }).resource("/logout/", |r| {
            r.method(Method::POST).with(views::logout)
        }).resource("/forgot_password/", |r| {
            r.method(Method::GET).with(views::ResetPassword::get);
            r.method(Method::POST).with(views::ResetPassword::post)
        })
    })
}
