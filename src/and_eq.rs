use std::arch::x86_64 as asm;

pub fn is_simd_enabled() -> bool {
    is_x86_feature_detected!("avx") && is_x86_feature_detected!("avx2")
        && is_x86_feature_detected!("sse4.1")
}

// A type that is four 64 bit integers packed together (256 bits).
pub use asm::__m256i as i256;

/// Print whether or not SIMD is enabled.
pub fn print_enabled() {
    #[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), feature = "use-simd"))]
    {
        if is_simd_enabled() {
            println!("SIMD Enabled");
            return;
        }
        panic!("SIMD Not Supported (no avx2)");
    }
    #[cfg(not(all(any(target_arch = "x86", target_arch = "x86_64"), feature = "use-simd")))]
    {
        println!("SIMD Disabled");
    }
}

/// Check if `0 = (a & b)` for `v * 256` edges.
#[inline(always)]
pub fn simd_and_eq_zero(a: [u64; 32], b: [u64; 32], v: usize) -> bool {
    #[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), feature = "use-simd"))]
    unsafe {
        simd_and_eq_zero_x86(a, b, v)
    }
    #[cfg(not(all(any(target_arch = "x86", target_arch = "x86_64"), feature = "use-simd")))]
    {
        simd_and_eq_zero_fallback(a, b, v)
    }
}

/// Check if `a = (a & b)` for `v * 256` edges.
#[inline(always)]
pub fn simd_and_eq(a: [u64; 32], b: [u64; 32], v: usize) -> bool {
    #[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), feature = "use-simd"))]
    unsafe {
        simd_and_eq_x86(a, b, v)
    }
    #[cfg(not(all(any(target_arch = "x86", target_arch = "x86_64"), feature = "use-simd")))]
    {
        simd_and_eq_fallback(a, b, v)
    }
}

#[target_feature(enable = "avx,avx2,sse4.1")]
unsafe fn simd_and_eq_x86(a: [u64; 32], b: [u64; 32], v: usize) -> bool {
    for i in (0..v).step_by(4) {
        // Build SIMD types.
        let a = asm::_mm256_lddqu_si256(&a[i] as *const _ as *const _);
        let b = asm::_mm256_lddqu_si256(&b[i] as *const _ as *const _);
        // Will be zero when equal.
        let b = asm::_mm256_xor_si256(a, b);
        // And, then compare.  If `c` does not equal zero, return false.
        if asm::_mm256_testz_si256(b, a) == 0 {
            return false;
        }
    }

    return true;
}

/// And, then equals.
#[cfg(not(all(any(target_arch = "x86", target_arch = "x86_64"), feature = "use-simd")))]
fn simd_and_eq_fallback(a: [u64; 32], mut b: [u64; 32], v: usize) -> bool {
    for i in 0..32 {
        b[i] &= a[i];
    }

    let integers = v / 64;
    let bitsleft = v % 64;

    for i in 0..32 {
        // Will be zero when equal.
        let c = a[i] ^ b[i];

        if i == integers {
            let one = 0b11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111u64;
            let trim = one >> (64 - bitsleft);

            if c & trim != 0 {
                return false;
            } else {
                return true;
            }
        } else if c != 0 {
            return false;
        }
    }

    return true;
}

#[target_feature(enable = "avx,avx2,sse4.1")]
unsafe fn simd_and_eq_zero_x86(a: [u64; 32], b: [u64; 32], v: usize) -> bool {
    for i in (0..v).step_by(4) {
        // Build SIMD types.
        let a = asm::_mm256_lddqu_si256(&a[i] as *const _ as *const _);
        let b = asm::_mm256_lddqu_si256(&b[i] as *const _ as *const _);
        // And, then compare.  If `c` does not equal zero, return false.
        if asm::_mm256_testz_si256(b, a) == 0 {
            return false;
        }
    }

    return true;
}

/// And, then equals.
#[cfg(not(all(any(target_arch = "x86", target_arch = "x86_64"), feature = "use-simd")))]
fn simd_and_eq_zero_fallback(a: [u64; 32], mut b: [u64; 32], v: usize) -> bool {
    for i in 0..32 {
        b[i] &= a[i];
    }

    let integers = v / 64;
    let bitsleft = v % 64;

    for i in 0..32 {
        // Will be zero when equal.
        let c = 0 ^ b[i];

        if i == integers {
            let one = 0b11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111u64;
            let trim = one >> (64 - bitsleft);

            if c & trim != 0 {
                return false;
            } else {
                return true;
            }
        } else if c != 0 {
            return false;
        }
    }

    return true;
}
