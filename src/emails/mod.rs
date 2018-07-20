//  emails/mod.rs
//
//  A module for dealing with emails - e.g, templates, etc.
//
//  @author Ryan McGrath <ryan@rymc.io>
//  @created 06/18/2018

pub mod postmark;
pub use self::postmark::{Postmark, Email};
