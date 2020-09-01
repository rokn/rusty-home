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

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate diesel;
extern crate serde;
extern crate dotenv;

mod api;
use api::routes::mount_routes;
use api::sqlite_db::SQLiteDb;

fn main() {
    let my_rocket = rocket::ignite();
    mount_routes(my_rocket)
        .attach(SQLiteDb::fairing())
        .launch();
}
