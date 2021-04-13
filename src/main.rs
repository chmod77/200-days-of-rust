// mod scalars;
// mod complexes;
// mod functions;
// mod prints;
// mod scalars;
// mod names;

// use crate::scalars::bools;
// use crate::scalars::chars;
// use crate::scalars::strs;
// use crate::names::display_names;
// use crate::scalars::scalar;

// use crate::complexes::tuple;
// use crate::complexes::array;
// use crate::functions::params;
// use crate::prints::display;

// fn main() {
//     // chars::run();
//     // bools::run();
//     // tuple::run();
//     strs::run("Shalom".to_string());
//     display_names::run("Dwayne".to_string());
//     // array::run();
//     // params::run(12);
//     display::run();
// }

use std::io;
mod randomize;

use crate::randomize::random;

fn main() {
    println!("Welcome to the guessing game");

    let secret_number = random::make_random(0..101);
    println!("{:?}", secret_number);
    println!("Enter the number here.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Error happened here");

    
    println!("You guessed {}", guess);
}
