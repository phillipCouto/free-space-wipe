extern crate clap;
extern crate rand;

use clap::{Arg, App};

mod cseitsg06;

fn main() {
    let matches = App::new("Free Space Wipe")
        .version("1.0")
        .author("Eckler Ltd. <fssuport@eckler.ca>")
        .about("Erases free space by writing a file until the disk is full.")
        .arg(Arg::with_name("procedure")
            .short("p")
            .long("procedure")
            .help("method to erase free space with. Supported: CSE-ITSG-06")
            .takes_value(true))
        .get_matches();

    match matches.value_of("procedure") {
        Some("CSE-ITSG-O6") => cseitsg06::execute(),
        Some(&_) => cseitsg06::execute(),
        None => cseitsg06::execute(),
    }
}
