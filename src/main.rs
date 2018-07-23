extern crate actix;
extern crate actix_web;
extern crate env_logger;
extern crate regex;
#[macro_use]
extern crate serde_json;

use actix_web::{middleware, server, App, HttpRequest, error, HttpResponse, http};
use std::process::{Command};
use regex::Regex;


fn pin_hash(req: &HttpRequest) -> HttpResponse {
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

            let stdout = std::str::from_utf8(&out.stdout)
                .expect("Invalid utf8");

            HttpResponse::Ok()
                .json(json!({"res": stdout}))
        },
        false => {
            HttpResponse::BadRequest()
                .json(json!({"error": "Invalid hash"} ))
        }
    }
}


fn main() {
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let sys = actix::System::new("ipfs-pin");

    server::new(|| {
        App::new()
            .middleware(middleware::Logger::default())
            .resource("/pin/{hash}", |r| r.method(http::Method::POST).f(pin_hash))
    }).bind("0.0.0.0:9999")
        .expect("port or host bind failure")
        .start();

    println!("Started http server: 0.0.0.0:9999");
    let _ = sys.run();
}
