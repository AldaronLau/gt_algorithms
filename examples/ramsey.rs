use gt_algorithms::*;

const R: usize = 3;
const S: usize = 3;

fn main() {
    // Print whether or not we have SIMD
    print_enabled();
    //
    println!("R({},{})={}", R, S, ramsey(R, S));
}
