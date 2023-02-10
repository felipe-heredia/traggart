extern crate rand;

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

fn main() {
    let random_string: String = thread_rng().sample_iter(Alphanumeric).take(32).collect();

    println!("{}", random_string);
}
