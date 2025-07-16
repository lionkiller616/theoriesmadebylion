// For CRC checks
use crc32fast::Hasher;

pub fn calculate_crc32(data: &[u8]) -> u32 {
    let mut hasher = Hasher::new();
    hasher.update(data);
    hasher.finalize()
}

// TODO: SIMD accelerated functions (e.g. for CRC, bulk XOR, specialized parsing)
// This would typically involve using core::arch or Rust's SIMD crates (e.g. packed_simd_2, std::simd - unstable)
// or calling C functions via FFI.

#[cfg(target_arch = "x86_64")]
pub fn _simd_example_x86_64() {
    // Example using core::arch for x86_64
    #[cfg(target_feature = "sse2")]
    unsafe {
        // use std::arch::x86_64::*;
        // let a = _mm_set_epi32(1, 2, 3, 4);
        // let b = _mm_set_epi32(5, 6, 7, 8);
        // let _result = _mm_add_epi32(a, b);
        // ... use result
    }
}

// Endianness helpers might be useful if not using byteorder crate directly everywhere.
// pub fn to_le_bytes_u32(val: u32) -> [u8; 4] { val.to_le_bytes() }
// pub fn from_le_bytes_u32(bytes: [u8; 4]) -> u32 { u32::from_le_bytes(bytes) }

// Versioning helpers
pub fn _parse_version_str(version_str: &str) -> Option<(u16, u16, u16)> {
    let parts: Vec<&str> = version_str.trim_start_matches('v').split('.').collect();
    if parts.len() == 3 {
        let major = parts[0].parse::<u16>().ok()?;
        let minor = parts[1].parse::<u16>().ok()?;
        let patch = parts[2].parse::<u16>().ok()?;
        Some((major, minor, patch))
    } else {
        None
    }
}