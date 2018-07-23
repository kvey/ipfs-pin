extern crate actix;
extern crate actix_web;
extern crate env_logger;
extern crate regex;
#[macro_use]
extern crate serde_json;

use actix_web::{middleware, server, App, HttpRequest, error, HttpResponse, http};
use std::process::{Command};
use regex::Regex;


fn hash(req: &HttpRequest) -> HttpResponse {
    let target_hash = req.match_info().get("hash").unwrap();

    let re = Regex::new(r"^[a-zA-Z0-9]+$").unwrap();

    match re.is_match(target_hash) {
        true => {
            let out = Command::new("ipfs")
                .arg("pin")
                .arg("add")
                .arg(target_hash)
                .output()
                .expect("failed to execute process");

            println!("ipfs pin add {}", target_hash);

            HttpResponse::Ok()
                .json(json!( {"res": std::str::from_utf8(&out.stdout).unwrap()}))
        },
        false => {
            HttpResponse::BadRequest()
                .json(json!( {"error": "Invalid hash"} ))
        }
    }
}



fn main() {
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let sys = actix::System::new("hello-world");

    server::new(|| {
        App::new()
            .middleware(middleware::Logger::default())
            .resource("/hash/{hash}", |r| r.method(http::Method::POST).f(hash))
    }).bind("0.0.0.0:9999")
        .unwrap()
        .start();

    println!("Started http server: 0.0.0.0:9999");
    let _ = sys.run();
}
