#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_regularize_correlations_FIX(
    mut XX: *mut libc::c_int,
    mut xx: *mut libc::c_int,
    mut noise: libc::c_int,
    mut D: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < D {
        *(&mut *XX.offset(0 as libc::c_int as isize) as *mut libc::c_int)
            .offset(
                (i * D + i) as isize,
            ) = *(&mut *XX.offset(0 as libc::c_int as isize) as *mut libc::c_int)
            .offset((i * D + i) as isize) + noise;
        i += 1;
        i;
    }
    *xx.offset(0 as libc::c_int as isize) += noise;
}
