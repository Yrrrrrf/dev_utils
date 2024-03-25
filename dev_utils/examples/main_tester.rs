#![allow(unused)]


// mod a;
// use a::some_fn;

extern crate dev_utils;

use dev_utils::{
    print_app_data,
    log::rlog::RLog,
};

fn main() {
    print_app_data(file!());
    RLog::init_logger(log::LevelFilter::Trace);

    log::info!("Some data!");

}
