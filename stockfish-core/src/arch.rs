#[cfg(target_pointer_width = "64")]
pub const TARGET_BITS: usize = 64;

#[cfg(target_pointer_width = "32")]
pub const TARGET_BITS: usize = 32;

#[cfg(target_endian = "little")]
pub const TARGET_ENDIAN: &str = "le";

#[cfg(target_endian = "big")]
pub const TARGET_ENDIAN: &str = "be";

#[cfg(use_pext)]
pub const USE_PEXT: bool = true;

#[cfg(not(use_pext))]
pub const USE_PEXT: bool = false;

#[inline]
#[must_use]
pub const fn pext_status() -> &'static str {
    if USE_PEXT { "pext_on" } else { "pext_off" }
}
