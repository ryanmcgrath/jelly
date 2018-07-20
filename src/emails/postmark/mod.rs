//  postmark.rs
//
//  A basic API client library for utilizing Postmark (https://postmarkapp.com/),
//  a great service for transactional emails.
//
//  @author Ryan McGrath <ryan@rymc.io>
//  @created 06/18/2018

use actix_web::client;
use std::collections::HashMap;

#[derive(Debug, Default, Serialize)]
pub struct Email<'a> {
    #[serde(rename = "Subject")]
    pub subject: &'a str,

    #[serde(rename = "TextBody")]
    pub body: &'a str,

    #[serde(rename = "HtmlBody")]
    pub html_body: Option<&'a str>,
    
    #[serde(rename = "From")]
    pub from: &'a str,
    
    #[serde(rename = "To")]
    pub to: &'a str,

    #[serde(rename = "ReplyTo")]
    pub reply_to: Option<&'a str>,

    #[serde(rename = "Cc")]
    pub cc: Option<&'a str>,

    #[serde(rename = "Bcc")]
    pub bcc: Option<&'a str>,

    #[serde(rename = "Tag")]
    pub tag: Option<&'a str>,

    #[serde(rename = "Metadata")]
    pub metadata: Option<HashMap<&'a str, &'a str>>,

    #[serde(rename = "Headers")]
    pub headers: Option<Vec<HashMap<&'a str, &'a str>>>,

    #[serde(rename = "TrackOpens")]
    pub track_opens: Option<bool>,

    #[serde(rename = "TrackLinks")]
    pub track_links: Option<&'a str>
}

#[derive(Debug)]
pub struct Postmark {
    api_token: String
}

impl Postmark {
    pub fn new(api_token: &str) -> Self {
        Postmark { api_token: api_token.into() }
    }

    pub fn send(&self, subject: &str, body: &str, from: &str, to: &str) -> client::SendRequest {
        let email = Email {
            from: from,
            to: to,
            subject: subject,
            body: body,
            ..Default::default()
        };

        client::post("https://api.postmarkapp.com/email")
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("X-Postmark-Server-Token", self.api_token.as_str())
            .json(email).unwrap().send()
    }
}
