//  src/users/views/reset_password.rs
//
//  Views for user registration.
//  
//  @author Ryan McGrath <ryan@rymc.io>
//  @created 06/18/2018

use askama::Template;
use futures::future::Future;
use djangohashers::{make_password};
use actix_web::{Form, AsyncResponder, HttpRequest, HttpResponse};

use State;
use users::models::{NewUser};
use util::responses::{FutureResponse, render, redirect};

#[derive(Template)]
#[template(path = "users/reset_password.html")]
pub struct ResetPassword<'a> {
    pub error: Option<&'a str>
}

impl<'a> ResetPassword<'a> {
    pub fn get(_req: HttpRequest<State>) -> HttpResponse {
        render(&ResetPassword {
            error: None
        })
    }

    pub fn post((req, item): (HttpRequest<State>, Form<NewUser>)) -> FutureResponse {
        let mut item = item.into_inner();
        item.password = make_password(&item.password);
        
        req.state().db.send(item).from_err().and_then(|res| match res {
            Ok(_) => Ok(redirect("/")),
            Err(e) => {
                warn!("Error creating new user: {:?}", e);
                Ok(render(&ResetPassword {
                    error: Some("An error occurred!")
                }))
            }
        }).responder()
    }
}
