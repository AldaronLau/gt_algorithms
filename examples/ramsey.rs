use gt_algorithms::*;

const R: usize = 2;
const S: usize = 7;

fn main() {
    // Print whether or not we have SIMD
    print_enabled();
    //
    println!("R({},{})={}", R, S, ramsey(R, S));
}
