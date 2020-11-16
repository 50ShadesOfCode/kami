extern crate clap;
use clap::{crate_version, App};

fn main() {
    let _matches = App::new("kami")
        .version(crate_version!())
        .about("kami app")
        .author("50ShadesOfCode <killershadow425@gmail.com>")
        .get_matches();
    println!("Hello, world!");
}
