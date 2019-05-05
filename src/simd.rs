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
        println!("SIMD Not Supported (no avx2)");
        return;
    }
    #[cfg(not(all(any(target_arch = "x86", target_arch = "x86_64"), feature = "use-simd")))]
    {
        println!("SIMD Disabled");
    }
}

/// Calculate `a & b` for at least `v` bits.
pub fn simd_and(a: [u64; 32], b: [u64; 32], v: usize) -> [u64; 32] {
    #[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), feature = "use-simd"))]
    {
        if is_simd_enabled() {
            return simd_and_x86(a, b, v);
        }
    }
    simd_and_fallback(a, b, v)
}

/// Check if `a = b` for exactly `v` bits.
pub fn simd_eq(a: [u64; 32], b: [u64; 32], v: usize) -> bool {
    #[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), feature = "use-simd"))]
    {
        if is_simd_enabled() {
            return simd_eq_x86(a, b, v);
        }
    }
    simd_eq_fallback(a, b, v)
}

pub fn simd_is_zero(a: [u64; 32], v: usize) -> bool {
    simd_eq(a, [0; 32], v)
}

/// & on X86 SIMD
#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), feature = "use-simd"))]
#[inline(always)]
fn simd_and_x86(a: [u64; 32], b: [u64; 32], v: usize) -> [u64; 32] {
    #[target_feature(enable = "avx,avx2,sse4.1")]
    unsafe fn internal(mut a: [u64; 32], b: [u64; 32], _v: usize) -> [u64; 32] {
        for i in 0..8 {
            let j = i << 2; // multiply by 4.
            // Build SIMD types.
            let c = asm::_mm256_loadu_si256(&a[j] as *const _ as *const _);
            let d = asm::_mm256_loadu_si256(&b[j] as *const _ as *const _);
            // And The Values together.
            let e = asm::_mm256_and_si256(c, d);
            // Write back to a
            asm::_mm256_storeu_si256(&mut a[j] as *mut _ as *mut _, e)
        }

        a
    }

    unsafe {
        internal(a, b, v)
    }
}

/// Fallback & on X86 SIMD
#[inline(always)]
fn simd_and_fallback(mut a: [u64; 32], b: [u64; 32], _v: usize) -> [u64; 32] {
    for i in 0..32 {
        a[i] &= b[i];
    }

    a
}

/// == on X86 SIMD
#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), feature = "use-simd"))]
#[inline(always)]
fn simd_eq_x86(a: [u64; 32], b: [u64; 32], v: usize) -> bool {
    // Build the Mask from V.
    let mut mask = [0x0u64; 32];
    let mask_index = v >> 6;
    let mask_last = 0xFFFFFFFF_FFFFFFFF >> (64 - (v & 63));
    for i in 0..mask_index {
        mask[i] = 0xFFFFFFFF_FFFFFFFF;
    }
    mask[mask_index] = mask_last;

    #[target_feature(enable = "avx,avx2,sse4.1")]
    unsafe fn internal(a: [u64; 32], b: [u64; 32], mask: [u64; 32]) -> bool {
        for i in 0..8 {
            let j = i << 2; // multiply by 4.
            // Build SIMD types.
            let aa = asm::_mm256_loadu_si256(&a[j] as *const _ as *const _);
            let bb = asm::_mm256_loadu_si256(&b[j] as *const _ as *const _);
            // Will be zero when equal.
            let c = asm::_mm256_xor_si256(aa, bb);

            let mask = asm::_mm256_loadu_si256(&mask[i * 4] as *const _ as *const _);

            // If `c` does not equal zero, return false.
            if asm::_mm256_testz_si256(c, mask) == 0 {
                return false;
            }
        }

        return true;
    }

    unsafe {
        internal(a, b, mask)
    }
}

/// Fallback == on X86 SIMD
#[inline(always)]
fn simd_eq_fallback(a: [u64; 32], b: [u64; 32], v: usize) -> bool {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_simd_eq_fallback() {
        let mut a = [0xFFFFFFFF_FFFFFFFFu64; 32];
        let mut b = [0u64; 32];

        a[0] = 0b11100011_11111111_11111111_10000000_11111111_11111111_10001101_11111111u64;
        a[1] = 0b11110000_11111111_11111111_11111111_11111111_11111111_11111111_11110000u64;
        b[0] = 0b11100011_11111111_11111111_10000000_11111111_11111111_10001101_11111111u64;
        b[1] = 0b00001111_11111111_11111111_11111111_11111111_11111111_11111111_11110000u64;

        assert_eq!(simd_eq_fallback(a, b, 84), true);

        a[0] = 0b11100011_11111111_11111111_10000000_11111111_11111111_10001101_11111110u64;

        assert_eq!(simd_eq_fallback(a, b, 84), false);
    }

    #[test]
    fn check_trivial_simd_eq_fallback() {
        let mut a = [0u64; 32];
        let mut b = [0u64; 32];

        a[0] = 0b111;
        b[0] = 0b101;

        assert_eq!(simd_eq_fallback(a, b, 3), false);
    }

    #[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), feature = "use-simd"))]
    #[test]
    fn check_simd_eq() {
        let mut a = [0xFFFFFFFF_FFFFFFFFu64; 32];
        let mut b = [0u64; 32];

        a[0] = 0b11100011_11111111_11111111_10000000_11111111_11111111_10001101_11111111u64;
        a[1] = 0b11110000_11111111_11111111_11111111_11111111_11111111_11111111_11110000u64;
        b[0] = 0b11100011_11111111_11111111_10000000_11111111_11111111_10001101_11111111u64;
        b[1] = 0b00001111_11111111_11111111_11111111_11111111_11111111_11111111_11110000u64;

        assert_eq!(simd_eq_x86(a, b, 84), true);

        a[0] = 0b11100011_11111111_11111111_10000000_11111111_11111111_10001101_11111110u64;

        assert_eq!(simd_eq_x86(a, b, 84), false);
    }

    #[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), feature = "use-simd"))]
    #[test]
    fn check_trivial_simd_eq() {
        let mut a = [0u64; 32];
        let mut b = [0u64; 32];

        a[0] = 0b111;
        b[0] = 0b101;

        assert_eq!(simd_eq_x86(a, b, 3), false);

        a[0] = 0b101;
        b[0] = 0b101;

        assert_eq!(simd_eq_x86(a, b, 3), true);
    }

    #[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), feature = "use-simd"))]
    #[test]
    fn check_simd_and() {
        let mut a = [0u64; 32];
        let mut b = [0u64; 32];

        a[0] = 0b111;
        b[0] = 0b101;

        a = simd_and_x86(a, b, 3);

        assert_eq!(a[0], 0b101);

        b[0] = 0b010;

        a = simd_and_x86(a, b, 3);

        assert_eq!(a[0], 0b000);
    }

    #[test]
    fn check_simd_and_fallback() {
        let mut a = [0u64; 32];
        let mut b = [0u64; 32];

        a[0] = 0b111;
        b[0] = 0b101;

        a = simd_and_fallback(a, b, 3);

        assert_eq!(a[0], 0b101);
    }

    #[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), feature = "use-simd"))]
    #[test]
    fn check_weird_simd_eq() {
        let a = [0b1011_11000u64; 32];
        let b = [0b1011_11001u64; 32];

        assert!(!simd_eq_x86(a, b, 5));
    }

    #[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), feature = "use-simd"))]
    #[test]
    fn check_simd_zero() {
        let mut a = [0; 32];

        a[0] = 0b110000;

        assert!(!simd_eq_x86(a, [0;32], 6));
    }
}
