// mod scalars;
mod complexes;
mod functions;
// use crate::scalars::bools;
// use crate::scalars::chars;
// use crate::scalars::strs;
// use crate::scalars::scalar;

// use crate::complexes::tuple;
use crate::complexes::array;
use crate::functions::params;
fn main() {
    // chars::run();
    // bools::run();
    // tuple::run();
    array::run();
    params::run(12);
}
