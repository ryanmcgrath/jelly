//  pages/mod.rs
//
//  Basic pages that don't really belong anywhere else, that
//  I want included but really don't want to deal with separate
//  repos/etc.
//
//  @author Ryan McGrath <ryan@rymc.io>
//  @created 06/18/2018

use futures::Future;
use askama::Template;
use actix_web::{http::Method, App};
use actix_web::{AsyncResponder, HttpRequest};

use State;
use users::middleware::UserAuthentication;
use users::models::User;
use util::responses::{FutureResponse, render, redirect};

#[derive(Template)]
#[template(path = "index.html")]
pub struct Homepage {}

#[derive(Template)]
#[template(path = "pages/terms.html")]
pub struct TermsOfService {}

#[derive(Template)]
#[template(path = "pages/privacy.html")]
pub struct PrivacyPolicy {}

#[derive(Template)]
#[template(path = "pages/cookies.html")]
pub struct CookiesPolicy {}

#[derive(Template)]
#[template(path = "pages/about.html")]
pub struct About {}

#[derive(Template)]
#[template(path = "pages/team.html")]
pub struct TeamBreakdown {}

#[derive(Template)]
#[template(path = "app/index.html")]
pub struct AppRoot {
    user: User
}

fn render_root(request: HttpRequest<State>) -> FutureResponse {
    // If the session is blank, or has no UID for the current user, this will
    // Err() immediately without a database hit.
    request.user().then(|res| match res {
        Ok(user) => Ok(render(&AppRoot {
            user: user
        })),

        Err(_e) => {
            Ok(render(&Homepage {}))
        }
    }).responder()
}

pub fn configure(application: App<State>) -> App<State> {
    application
        .resource("/", |r| {
            r.name("homepage");
            r.method(Method::GET).f(render_root);
            r.method(Method::POST).f(render_root)
        })
        .resource("/terms/", |r| r.method(Method::GET).f(|_req| render(&TermsOfService {})))
        .resource("/privacy/", |r| r.method(Method::GET).f(|_req| render(&PrivacyPolicy {})))
        .resource("/cookies/", |r| r.method(Method::GET).f(|_req| render(&CookiesPolicy {})))
        .resource("/about/", |r| r.method(Method::GET).f(|_req| render(&About {})))
        .resource("/team/", |r| r.method(Method::GET).f(|_req| render(&TeamBreakdown{})))
}
