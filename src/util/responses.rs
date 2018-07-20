//  src/util/responses.rs
//
//  Basic response objects that are commonly thrown about in API calls.
//
//  @author Ryan McGrath <ryan@rymc.io>
//  @created 06/18/2018

use askama::Template;
use futures::future::{Future, result};
use actix_web::{HttpResponse, Error, AsyncResponder};

pub type FutureResponse = Box<Future<Item = HttpResponse, Error = Error>>;

#[derive(Deserialize, Serialize)]
pub struct OperationResponse<'a> {
    pub success: bool,
    pub message: Option<&'a str>
}

#[inline(always)]
pub fn render(template: &Template) -> HttpResponse {
    HttpResponse::Ok().content_type("text/html").body(template.render().unwrap())
}

#[inline(always)]
pub fn future_render(template: &Template) -> FutureResponse {
    result(Ok(HttpResponse::Ok().content_type("text/html").body(template.render().unwrap()))).responder()
}

#[inline(always)]
pub fn redirect(location: &str) -> HttpResponse {
    HttpResponse::TemporaryRedirect().header("Location", location).finish()
}

#[inline(always)]
pub fn future_redirect(location: &str) -> FutureResponse {
    result(Ok(HttpResponse::TemporaryRedirect().header("Location", location).finish())).responder()
}
