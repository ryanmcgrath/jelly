//  src/users/views/login.rs
//
//  Views for user authentication.
//  
//  @author Ryan McGrath <ryan@rymc.io>
//  @created 06/18/2018

use askama::Template;
use validator::Validate;
use futures::future::Future;
use djangohashers::{check_password};
use actix_web::{Form, AsyncResponder, HttpRequest, HttpResponse};
use actix_web::middleware::session::RequestSession;

use State;
use users::models::{UserLogin};
use users::middleware::UserAuthentication;
use util::forms::collect_validation_errors;
use util::responses::{FutureResponse, future_render, future_redirect, redirect, render};

#[derive(Template)]
#[template(path = "users/login.html")]
pub struct Login {
    pub errors: Option<Vec<String>>
}

impl Login {
    pub fn get(request: HttpRequest<State>) -> HttpResponse {
        if request.is_authenticated() {
            let url = request.url_for("homepage", &[""; 0]).unwrap();
            return redirect(url.as_str());
        }

        render(&Self {
            errors: None
        })
    }

    pub fn post((request, login): (HttpRequest<State>, Form<UserLogin>)) -> FutureResponse {
        if request.is_authenticated() {
            let url = request.url_for("homepage", &[""; 0]).unwrap();
            return future_redirect(url.as_str());
        }

        // No sense in wasting a database call if they didn't pass in an actual email address.
        // *shrug*
        let login = login.into_inner();
        if let Err(e) = login.validate() {
            return future_render(&Self {
                errors: Some(collect_validation_errors(e))
            });
        }

        let password = login.password.clone();
        request.state().db.send(login).from_err().and_then(move |res| match res {
            Ok(user) => {
                // Yo, so I know you're coming here and looking at this .unwrap(), and probably
                // going "man what is this guy doing?". This should only ever give a Err() result
                // if we're trying to use an unknown algorithm... which isn't the case here.
                if check_password(&password, &user.password).unwrap() {
                    if let Err(e) = request.session().set("uid", user.id) {
                        error!("Could not set UID for user session! {:?}", e);
                        return Ok(render(&Self {
                            errors: Some(vec!["An internal error occurred while attempting to sign you in. Please try again in a bit.".into()])
                        }));
                    }
                
                    let url = request.url_for("homepage", &[""; 0]).unwrap();
                    Ok(redirect(url.as_str()))
                } else {
                    Ok(render(&Self {
                        errors: Some(vec!["Email or password is incorrect.".into()])
                    }))
                }
            },
            
            Err(e) => {
                warn!("Error locating user: {:?}", e);
                Ok(render(&Self {
                    errors: Some(vec!["Email or password is incorrect.".into()])
                }))
            }
        }).responder()
    }
}
