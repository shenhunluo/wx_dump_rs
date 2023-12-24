#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_scale_copy_vector16(
    mut data_out: *mut libc::c_short,
    mut data_in: *const libc::c_short,
    mut gain_Q16: libc::c_int,
    dataSize: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut tmp32: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < dataSize {
        tmp32 = (gain_Q16 >> 16 as libc::c_int)
            * *data_in.offset(i as isize) as libc::c_int
            + ((gain_Q16 & 0xffff as libc::c_int)
                * *data_in.offset(i as isize) as libc::c_int >> 16 as libc::c_int);
        *data_out.offset(i as isize) = tmp32 as libc::c_short;
        i += 1;
        i;
    }
}
