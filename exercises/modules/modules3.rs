// modules3.rs
//
// You can use the 'use' keyword to bring module paths from modules from
// anywhere and especially from the Rust standard library into your scope. Bring
// SystemTime and UNIX_EPOCH from the std::time module. Bonus style points if
// you can do it with one line!
//
// Execute `rustlings hint modules3` or use the `hint` watch subcommand for a
// hint.


// TODO: Complete this use statement
// //self below is not needed just added to test
use std::time::{self,SystemTime,UNIX_EPOCH};

fn main() {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(res) => println!("1970-01-01 00:00:00 UTC was {} seconds ago!", res.as_secs()),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}
