#[macro_use] extern crate rocket;

use std::net::SocketAddr;

use chrono::Local;


#[get("/")]
fn index(remote_addr: SocketAddr) -> String {
    let now = Local::now();
    format!("{remote_addr}\n{}", now.format("[%Y-%m-%d][%H:%M:%S]"))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}

