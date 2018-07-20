# Jelly
This is a sample repository showcasing a rather straightforward way to handle user sessions, signup, and authentication in an [actix-web](https://actix.rs) project. I extracted it from something I'm working on as I realized that it can be otherwise tricky to figure out at a glance how all of this fits together (actix-web is still fairly fast moving, and the docs can be... rough).

You might be interested in this project if:

- You want a sample Rust/actix-web project to kick off that has (mostly) sane defaults, and built-in user accounts.
- You're unsure about how to structure an actix-web project, and want an opinionated (not even necessarily correct) starter.
- You're not interested in putting a puzzle together for something as basic as user authentication, and just want it to work.

You might also not be interested in this, and that's cool too. It's licensed as a "do whatever you want" type deal, so... clone away and have fun. Some extra notes are below.

## Setup
- Clone the repo
- `mv example.env .env`, and fill in the values in there
- `diesel migration run` to create the user database table
- `cargo run` to... well, run it. Depending on whether you have `diesel_cli` installed you might need that too.

## Notes
This is probably still a bit rough around the edges, since I ripped it out of an existing project of mine, but the key things I wanted to solve were:

- User signup/login, with mostly secure cookie defaults
- An easy way to check the current active user/session on each request
- Figuring out how the hell to shove Redis in here - sessions are stored in there instead of the built-in `CookieSessionBackend` you'll find that ships with actix-web.

There's some "middleware" here (`src/users/middleware.rs`) that makes it easy to check the authentication status for the request, and load the associated `User` record. The first one, `request.is_authentication()`, simply checks the session to see if we have anything indicating a `User` is set. The second one, `request.user()`, returns a future that'll provide the actual `User` object.

`FutureResponse` and `future_redirect` are some wrappers around `actix-web` response formats to make the ergonomics of all of this more readable. You can take 'em or leave 'em.

``` rust
use users::middleware::UserAuthentication;
use utils::responses::{FutureResponse, future_redirect};

fn view(request: HttpRequest) -> FutureResponse {
    // Check the session is valid, without a database hit to load the user
    if let Err(e) = request.is_authenticated() {
        return future_redirect("http://www.mozilla.com/");
    }

    // Call over to Postgres and get that there user
    request.user().then(|a| match a {
        Ok(user) => {
            future_redirect("http://www.duckduckgo.com/")
        },

        Err(_) => {
            future_redirect("http://www.google.com/")
        }
    }).responder()
}
```

If I was the kind to use Rust nightly in a project, I'd be interested in a derive-esque macro to check auth, ala Django's `@login_required` decorator.

Also, as you read the code, you may notice a lot of this is influenced by Django. I think they got the user model stuff right at some point over the years. Thanks to the `djangohashers` package, this even matches the password hashing Django does.

Oh, and randomly, this includes a simple library for sending emails via [Postmark](https://postmarkapp.com/), since I enjoy their service.

## Questions, Comments, Etc?
- Email: [ryan@rymc.io](mailto:ryan@rymc.io)
- Twitter: [@ryanmcgrath](https://twitter.com/ryanmcgrath/)
- Web: [rymc.io](https://rymc.io/)

## License
Do what you want. Read the license, it'll say the same.
