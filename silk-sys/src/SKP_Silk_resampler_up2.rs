#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]

use crate::SKP_Silk_resampler_rom::{SKP_Silk_resampler_up2_lq_0, SKP_Silk_resampler_up2_lq_1};
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_resampler_up2(
    mut S: *mut libc::c_int,
    mut out: *mut libc::c_short,
    mut in_0: *const libc::c_short,
    mut len: libc::c_int,
) {
    let mut k: libc::c_int = 0;
    let mut in32: libc::c_int = 0;
    let mut out32: libc::c_int = 0;
    let mut Y: libc::c_int = 0;
    let mut X: libc::c_int = 0;
    k = 0 as libc::c_int;
    while k < len {
        in32 = (*in_0.offset(k as isize) as libc::c_int) << 10 as libc::c_int;
        Y = in32 - *S.offset(0 as libc::c_int as isize);
        X = (Y >> 16 as libc::c_int) * SKP_Silk_resampler_up2_lq_0 as libc::c_int
            + ((Y & 0xffff as libc::c_int) * SKP_Silk_resampler_up2_lq_0 as libc::c_int
                >> 16 as libc::c_int);
        out32 = *S.offset(0 as libc::c_int as isize) + X;
        *S.offset(0 as libc::c_int as isize) = in32 + X;
        *out
            .offset(
                (2 as libc::c_int * k) as isize,
            ) = (if (if 10 as libc::c_int == 1 as libc::c_int {
            (out32 >> 1 as libc::c_int) + (out32 & 1 as libc::c_int)
        } else {
            (out32 >> 10 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                >> 1 as libc::c_int
        }) > 0x7fff as libc::c_int
        {
            0x7fff as libc::c_int
        } else if (if 10 as libc::c_int == 1 as libc::c_int {
            (out32 >> 1 as libc::c_int) + (out32 & 1 as libc::c_int)
        } else {
            (out32 >> 10 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                >> 1 as libc::c_int
        }) < 0x8000 as libc::c_int as libc::c_short as libc::c_int
        {
            0x8000 as libc::c_int as libc::c_short as libc::c_int
        } else if 10 as libc::c_int == 1 as libc::c_int {
            (out32 >> 1 as libc::c_int) + (out32 & 1 as libc::c_int)
        } else {
            (out32 >> 10 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                >> 1 as libc::c_int
        }) as libc::c_short;
        Y = in32 - *S.offset(1 as libc::c_int as isize);
        X = Y
            + ((Y >> 16 as libc::c_int) * SKP_Silk_resampler_up2_lq_1 as libc::c_int
                + ((Y & 0xffff as libc::c_int)
                    * SKP_Silk_resampler_up2_lq_1 as libc::c_int >> 16 as libc::c_int));
        out32 = *S.offset(1 as libc::c_int as isize) + X;
        *S.offset(1 as libc::c_int as isize) = in32 + X;
        *out
            .offset(
                (2 as libc::c_int * k + 1 as libc::c_int) as isize,
            ) = (if (if 10 as libc::c_int == 1 as libc::c_int {
            (out32 >> 1 as libc::c_int) + (out32 & 1 as libc::c_int)
        } else {
            (out32 >> 10 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                >> 1 as libc::c_int
        }) > 0x7fff as libc::c_int
        {
            0x7fff as libc::c_int
        } else if (if 10 as libc::c_int == 1 as libc::c_int {
            (out32 >> 1 as libc::c_int) + (out32 & 1 as libc::c_int)
        } else {
            (out32 >> 10 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                >> 1 as libc::c_int
        }) < 0x8000 as libc::c_int as libc::c_short as libc::c_int
        {
            0x8000 as libc::c_int as libc::c_short as libc::c_int
        } else if 10 as libc::c_int == 1 as libc::c_int {
            (out32 >> 1 as libc::c_int) + (out32 & 1 as libc::c_int)
        } else {
            (out32 >> 10 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                >> 1 as libc::c_int
        }) as libc::c_short;
        k += 1;
    }
}
