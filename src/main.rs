//use clokwerk::{Scheduler, TimeUnits};
//use clokwerk::Interval::*;
//use std::thread;
//use std::time::Duration;
//
//fn main() {
//    let mut scheduler = Scheduler::new();
//    scheduler.every(2.seconds()).run(|| println!("Periodic task"));
//    for _ in 1..10000 {
//        scheduler.run_pending();
//        thread::sleep(Duration::from_millis(10));
//    }
//}

#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket as rocket_lib;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate diesel;
extern crate serde;
extern crate dotenv;
extern crate reqwest;

mod api;
mod frontend;
use api::sqlite_db::SQLiteDbCtx;
use rocket_contrib::templates::Template;
use rocket_contrib::serve::StaticFiles;

fn main() {
    let rocket = rocket_lib::ignite();
    let rocket = api::routes::mount_routes(rocket);
    let rocket = frontend::routes::mount_routes(rocket);
    rocket.mount("/static", StaticFiles::from("static"))
        .attach(SQLiteDbCtx::fairing())
        .attach(Template::fairing()).launch();
}
