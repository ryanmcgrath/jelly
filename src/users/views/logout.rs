//  src/users/views/signup.rs
//
//  Endpoint for logging a user out. Nothing particularly special,
//  just obliterates their session entry + cookie.
//  
//  @author Ryan McGrath <ryan@rymc.io>
//  @created 06/18/2018

use actix_web::{HttpRequest, HttpResponse};
use actix_web::middleware::session::RequestSession;

use State;
use util::responses::redirect;

pub fn logout(request: HttpRequest<State>) -> HttpResponse {
    request.session().clear();
    let url = request.url_for("homepage", &[""; 0]).unwrap();
    redirect(url.as_str())
}
