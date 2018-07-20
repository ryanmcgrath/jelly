//  src/users/views/mod.rs
//
//  View hoisting. *shrug*
//
//  @author Ryan McGrath <ryan@rymc.io>
//  @created 06/18/2018

pub mod signup;
pub use self::signup::Signup;

pub mod login;
pub use self::login::Login;

pub mod logout;
pub use self::logout::logout;

pub mod reset_password;
pub use self::reset_password::ResetPassword;
