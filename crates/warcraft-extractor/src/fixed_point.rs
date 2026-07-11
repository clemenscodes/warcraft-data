//! `f32` → fixed-point conversions used when building the domain quantity value
//! objects from the parsed CASC data (and when emitting their constructors). The
//! game's values are fractional but bounded, so rounding to milli/permille is
//! exact for the data we ship.

/// Round a floating-point value to signed thousandths (milli).
pub fn milli_i32(value: f32) -> i32 {
    (value * 1000.0).round() as i32
}

/// Round a floating-point value to unsigned thousandths (milli).
pub fn milli_u32(value: f32) -> u32 {
    (value.max(0.0) * 1000.0).round() as u32
}

/// Round a fractional probability (0.0..=1.0) to permille (0..=1000).
pub fn permille_u16(value: f32) -> u16 {
    (value.clamp(0.0, 1.0) * 1000.0).round() as u16
}
