//  build.rs
//
//  A build script. What, you wanna fight about it? Here's your paycheck.
//
//  @author Ryan McGrath <ryan@rymc.io>
//  @created 06/18/2018

extern crate askama;

fn main() {
    askama::rerun_if_templates_changed();
}
