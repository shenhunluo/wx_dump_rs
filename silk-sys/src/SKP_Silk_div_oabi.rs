#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#[no_mangle]
pub unsafe extern "C" fn SKP_DIV32_arm(
    mut a32: libc::c_int,
    mut b32: libc::c_int,
) -> libc::c_int {
    return a32 / b32;
}
