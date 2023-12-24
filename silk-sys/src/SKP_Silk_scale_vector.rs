#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
pub type __int64_t = libc::c_longlong;
pub type int64_t = __int64_t;
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_scale_vector32_Q26_lshift_18(
    mut data1: *mut libc::c_int,
    mut gain_Q26: libc::c_int,
    mut dataSize: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < dataSize {
        *data1
            .offset(
                i as isize,
            ) = (*data1.offset(i as isize) as int64_t * gain_Q26 as int64_t
            >> 8 as libc::c_int) as libc::c_int;
        i += 1;
        i;
    }
}
