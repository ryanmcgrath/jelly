//  src/users/middleware.rs
//
//  Middleware that handles loading current User for a given
//  request, along with any Session data they may have. This
//  specifically enables Anonymous users with Session data, ala
//  Django's approach.
//
//  @author Ryan McGrath <ryan@rymc.io>
//  @created 06/18/2018

use std::io::{Error as IoError, ErrorKind};
use futures::Future;
use futures::future;
use actix_web::{HttpRequest, Error};
use actix_web::middleware::session::RequestSession;

use State;
use users::models::{UserLookup, User};

pub type UserAuthenticationResult = Box<Future<Item = User, Error = Error>>;

/// UserAuthentication is kind of a request guard - it returns a Future which will resolve
/// with either the current authenticated user, or "error" out if the user has no session data
/// that'd tie them to a user profile, or if the session cache can't be read, or if the database
/// has issues, or... pick your poison I guess.
///
/// It enables your views to be as simple as:
///
/// # Example
///
/// ```
/// use users::middleware::UserAuthentication;
/// use utils::responses::{AsyncHttpResponse, async_redirect};
///
/// fn view(request: HttpRequest) -> AsyncHttpResponse {
///     request.check_authentication().then(move |a| match a {
///         Ok(user) => {
///             async_redirect("http://www.duckduckgo.com/")
///         },
///
///         Err(_) => {
///             async_redirect("http://www.google.com/")
///         }
///     })
/// }
/// ```
pub trait UserAuthentication {
    fn is_authenticated(&self) -> bool;
    fn user(&self) -> UserAuthenticationResult;
}

impl UserAuthentication for HttpRequest<State> {
    #[inline(always)]
    fn is_authenticated(&self) -> bool {
        match self.session().get::<i32>("uid") {
            Ok(session) => {
                match session {
                    Some(_session_id) => true,
                    None => false
                }
            },

            Err(e) => {
                error!("Error'd when attempting to fetch session data: {:?}", e);
                false
            }
        }
    }

    fn user(&self) -> UserAuthenticationResult {
        match self.session().get::<i32>("uid") {
            Ok(session) => { match session {
                Some(session_id) => {
                    Box::new(self.state().db.send(UserLookup {
                        id: session_id
                    }).from_err().and_then(|res| match res {
                        Ok(user) => Ok(user),
                        Err(err) => {
                            // Temporary because screw matching all these error types 
                            let e = IoError::new(ErrorKind::NotFound, format!("{}", err));
                            Err(e.into())
                        }
                    })) 
                },

                None => {
                    let e = IoError::new(ErrorKind::NotFound, "User has no session data.");
                    Box::new(future::err(e.into()))
                }
            }},

            Err(e) => {
                error!("Error'd when attempting to fetch session data: {:?}", e);
                Box::new(future::err(e.into()))
            }
        }
    }
}
