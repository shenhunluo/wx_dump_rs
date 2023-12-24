#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_interpolate(
    mut xi: *mut libc::c_int,
    mut x0: *const libc::c_int,
    mut x1: *const libc::c_int,
    ifact_Q2: libc::c_int,
    d: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < d {
        *xi
            .offset(
                i as isize,
            ) = *x0.offset(i as isize)
            + ((*x1.offset(i as isize) - *x0.offset(i as isize)) * ifact_Q2
                >> 2 as libc::c_int);
        i += 1;
        i;
    }
}
