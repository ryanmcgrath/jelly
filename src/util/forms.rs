//  src/util/forms.rs
//
//  Helper methods for dealing with certain form-related things, like... validation.
//
//  @author Ryan McGrath <ryan@rymc.io>
//  @created 06/18/2018

use validator::ValidationErrors;

/// Handles collecting validation errors from Form-ish structs.
/// This has to be done this way to work with Askama templates, which... well, I'm
/// not sold on, but it gets the job done for now.
///
/// @TODO: See about String -> &str?
#[inline(always)]
pub fn collect_validation_errors(e: ValidationErrors) -> Vec<String> {
    e.inner().into_iter().map(|(_k, v)| {
        v.into_iter().map(|a| {
            a.message.unwrap().to_string()
        }).collect()
    }).collect()
}
