#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_resampler_private_AR2(
    mut S: *mut libc::c_int,
    mut out_Q8: *mut libc::c_int,
    mut in_0: *const libc::c_short,
    mut A_Q14: *const libc::c_short,
    mut len: libc::c_int,
) {
    let mut k: libc::c_int = 0;
    let mut out32: libc::c_int = 0;
    k = 0 as libc::c_int;
    while k < len {
        out32 = *S.offset(0 as libc::c_int as isize)
            + ((*in_0.offset(k as isize) as libc::c_int) << 8 as libc::c_int);
        *out_Q8.offset(k as isize) = out32;
        out32 = out32 << 2 as libc::c_int;
        *S
            .offset(
                0 as libc::c_int as isize,
            ) = *S.offset(1 as libc::c_int as isize)
            + ((out32 >> 16 as libc::c_int)
                * *A_Q14.offset(0 as libc::c_int as isize) as libc::c_int
                + ((out32 & 0xffff as libc::c_int)
                    * *A_Q14.offset(0 as libc::c_int as isize) as libc::c_int
                    >> 16 as libc::c_int));
        *S
            .offset(
                1 as libc::c_int as isize,
            ) = (out32 >> 16 as libc::c_int)
            * *A_Q14.offset(1 as libc::c_int as isize) as libc::c_int
            + ((out32 & 0xffff as libc::c_int)
                * *A_Q14.offset(1 as libc::c_int as isize) as libc::c_int
                >> 16 as libc::c_int);
        k += 1;
    }
}
