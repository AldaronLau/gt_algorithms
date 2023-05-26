//! Calculates fast for
//! * all R(1, _)
//! * no larger than R(2, 6)
//! * no larger R(3, 3)

mod and_eq;
mod color;
mod graph;
mod ramsey;

// use rayon::prelude::*;

pub use self::{
    and_eq::{i256, print_enabled, simd_and_eq, simd_and_eq_zero},
    color::{FALSE, TRUE},
    graph::{add, next, Graph, MAX_GRAPH_VERTICES},
    ramsey::ramsey,
};
