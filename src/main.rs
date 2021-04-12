// mod scalars;
mod complexes;
mod functions;
mod prints;
// use crate::scalars::bools;
// use crate::scalars::chars;
// use crate::scalars::strs;
// use crate::scalars::scalar;

// use crate::complexes::tuple;
use crate::complexes::array;
use crate::functions::params;
use crate::prints::display;

fn main() {
    // chars::run();
    // bools::run();
    // tuple::run();
    array::run();
    params::run(12);
    display::run();
}
