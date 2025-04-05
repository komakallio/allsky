#![allow(
    non_upper_case_globals,
    non_camel_case_types,
    non_snake_case,
    unsafe_op_in_unsafe_fn
)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

/// Calculate bytes per image width, making sure the width
/// is aligned to the closest 32-bit (4-byte) boundary, to be
/// compatible with the Device Independent Bitmap (DIB) format.
/// This is equal to the TDIBWIDTHBYTES macro in the the
/// Toupcam SDK.
///
/// # Arguments
///
/// * `width_bits` - The width of the image in bits.
///
/// # Returns
///
/// * The width of the image in bytes, aligned to the closest 32-bit (4-byte) boundary.
pub fn dib_width_bytes(width_bits: i32) -> i32 {
    ((width_bits + 31) / 32) * 4
}
