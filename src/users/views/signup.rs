//  src/users/views/signup.rs
//
//  Views for user registration.
//  
//  @author Ryan McGrath <ryan@rymc.io>
//  @created 06/18/2018

use askama::Template;
use validator::Validate;
use futures::future::Future;
use djangohashers::{make_password};
use actix_web::{Form, AsyncResponder, HttpRequest, HttpResponse};
use actix_web::middleware::session::RequestSession;

use State;
use users::middleware::UserAuthentication;
use users::models::{NewUser};
use util::forms::collect_validation_errors;
use util::responses::{FutureResponse, render, future_render, redirect, future_redirect};

#[derive(Template)]
#[template(path = "users/signup.html")]
pub struct Signup {
    pub errors: Option<Vec<String>>
}

impl Signup {
    pub fn get(request: HttpRequest<State>) -> HttpResponse {
        if request.is_authenticated() {
            let url = request.url_for("homepage", &[""; 0]).unwrap();
            return redirect(url.as_str());
        }

        render(&Self {
            errors: None
        })
    }

    pub fn post((request, user): (HttpRequest<State>, Form<NewUser>)) -> FutureResponse {
        if request.is_authenticated() {
            let url = request.url_for("homepage", &[""; 0]).unwrap();
            return future_redirect(url.as_str());
        }

        let mut user = user.into_inner();
        if let Err(e) = user.validate() {
            return future_render(&Self {
                errors: Some(collect_validation_errors(e))
            });
        }

        user.password = make_password(&user.password); 
        request.state().db.send(user).from_err().and_then(move |res| match res {
            Ok(user) => {
                if let Err(e) = request.session().set("uid", user.id) {
                    error!("Could not set UID for user session! {:?}", e);
                    return Ok(render(&Self {
                        errors: Some(vec![
                            "Your account was created, but an internal error happened while \
                            attempting to sign you in. Try again in a bit!".into()
                        ])
                    }))
                }
                
                let url = request.url_for("homepage", &[""; 0]).unwrap();
                Ok(redirect(url.as_str()))
            },
            Err(e) => {
                error!("Error creating new user: {:?}", e);
                Ok(render(&Self {
                    errors: Some(vec![
                        "An error occurred while trying to create your account. We've \
                        notified the engineering team and are looking into it - feel \
                        free to contact us for more information, or if you continue to \
                        see the issue after a short period.".into()
                    ])
                }))
            }
        }).responder()
    }
}
