#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]

use crate::skp_silk_resampler_rom::{SKP_SILK_RESAMPLER_DOWN2_1, SKP_SILK_RESAMPLER_DOWN2_0};
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_resampler_private_down4(
    mut S: *mut libc::c_int,
    mut out: *mut libc::c_short,
    mut in_0: *const libc::c_short,
    mut inLen: libc::c_int,
) {
    let mut k: libc::c_int = 0;
    let mut len4: libc::c_int = inLen >> 2 as libc::c_int;
    let mut in32: libc::c_int = 0;
    let mut out32: libc::c_int = 0;
    let mut Y: libc::c_int = 0;
    let mut X: libc::c_int = 0;
    k = 0 as libc::c_int;
    while k < len4 {
        in32 = (*in_0.offset((4 as libc::c_int * k) as isize) as libc::c_int
            + *in_0.offset((4 as libc::c_int * k + 1 as libc::c_int) as isize)
                as libc::c_int) << 9 as libc::c_int;
        Y = in32 - *S.offset(0 as libc::c_int as isize);
        X = Y
            + ((Y >> 16 as libc::c_int) * SKP_SILK_RESAMPLER_DOWN2_1 as libc::c_int
                + ((Y & 0xffff as libc::c_int)
                    * SKP_SILK_RESAMPLER_DOWN2_1 as libc::c_int >> 16 as libc::c_int));
        out32 = *S.offset(0 as libc::c_int as isize) + X;
        *S.offset(0 as libc::c_int as isize) = in32 + X;
        in32 = (*in_0.offset((4 as libc::c_int * k + 2 as libc::c_int) as isize)
            as libc::c_int
            + *in_0.offset((4 as libc::c_int * k + 3 as libc::c_int) as isize)
                as libc::c_int) << 9 as libc::c_int;
        Y = in32 - *S.offset(1 as libc::c_int as isize);
        X = (Y >> 16 as libc::c_int) * SKP_SILK_RESAMPLER_DOWN2_0 as libc::c_int
            + ((Y & 0xffff as libc::c_int) * SKP_SILK_RESAMPLER_DOWN2_0 as libc::c_int
                >> 16 as libc::c_int);
        out32 = out32 + *S.offset(1 as libc::c_int as isize);
        out32 = out32 + X;
        *S.offset(1 as libc::c_int as isize) = in32 + X;
        *out
            .offset(
                k as isize,
            ) = (if (if 11 as libc::c_int == 1 as libc::c_int {
            (out32 >> 1 as libc::c_int) + (out32 & 1 as libc::c_int)
        } else {
            (out32 >> 11 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                >> 1 as libc::c_int
        }) > 0x7fff as libc::c_int
        {
            0x7fff as libc::c_int
        } else if (if 11 as libc::c_int == 1 as libc::c_int {
            (out32 >> 1 as libc::c_int) + (out32 & 1 as libc::c_int)
        } else {
            (out32 >> 11 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                >> 1 as libc::c_int
        }) < 0x8000 as libc::c_int as libc::c_short as libc::c_int
        {
            0x8000 as libc::c_int as libc::c_short as libc::c_int
        } else if 11 as libc::c_int == 1 as libc::c_int {
            (out32 >> 1 as libc::c_int) + (out32 & 1 as libc::c_int)
        } else {
            (out32 >> 11 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                >> 1 as libc::c_int
        }) as libc::c_short;
        k += 1;
    }
}
