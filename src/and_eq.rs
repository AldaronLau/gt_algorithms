use std::arch::x86_64 as asm;

// A type that is four 64 bit integers packed together (256 bits).
pub use asm::__m256i as i256;

/// Print whether or not SIMD is enabled.
#[inline(always)]
pub fn print_enabled() {
    #[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), feature = "use-simd"))]
    {
        if is_x86_feature_detected!("avx")
            && is_x86_feature_detected!("avx2")
            && is_x86_feature_detected!("sse4.1")
        {
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
pub fn simd_and_eq_zero(a: &[u64; 32], b: &[u64; 32], v: usize) -> bool {
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
pub fn simd_and_eq(a: &[u64; 32], b: &[u64; 32], v: usize) -> bool {
    #[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), feature = "use-simd"))]
    unsafe {
        simd_and_eq_x86(a, b, v)
    }
    #[cfg(not(all(any(target_arch = "x86", target_arch = "x86_64"), feature = "use-simd")))]
    {
        simd_and_eq_fallback(a, b, v)
    }
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), feature = "use-simd"))]
#[target_feature(enable = "avx,avx2,sse4.1")]
unsafe fn simd_and_eq_x86(a: &[u64; 32], b: &[u64; 32], v: usize) -> bool {
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

    true
}

/// And, then equals.
#[cfg(not(all(any(target_arch = "x86", target_arch = "x86_64"), feature = "use-simd")))]
fn simd_and_eq_fallback(a: &[u64; 32], b: &[u64; 32], v: usize) -> bool {
    let integers = v * 4;

    for i in 0..32 {
        // Will be zero when equal.
        let c = a[i] ^ (b[i] & a[i]);

        if i == integers {
            if c != 0 {
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

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), feature = "use-simd"))]
#[target_feature(enable = "avx,avx2,sse4.1")]
unsafe fn simd_and_eq_zero_x86(a: &[u64; 32], b: &[u64; 32], v: usize) -> bool {
    for i in (0..v).step_by(4) {
        // Build SIMD types.
        let a = asm::_mm256_lddqu_si256(&a[i] as *const _ as *const _);
        let b = asm::_mm256_lddqu_si256(&b[i] as *const _ as *const _);
        // And, then compare.  If `c` does not equal zero, return false.
        if asm::_mm256_testz_si256(b, a) == 0 {
            return false;
        }
    }

    true
}

/// And, then equals.
#[cfg(not(all(any(target_arch = "x86", target_arch = "x86_64"), feature = "use-simd")))]
fn simd_and_eq_zero_fallback(a: &[u64; 32], b: &[u64; 32], v: usize) -> bool {
    let integers = v * 4;

    for i in 0..32 {
        // Will be zero when equal.
        let c = 0 ^ (b[i] & a[i]);

        if i == integers {
            if c != 0 {
                return false;
            } else {
                return true;
            }
        } else if c != 0 {
            return false;
        }
    }

    true
}
