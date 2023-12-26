#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]

use crate::skp_silk_resampler_rom::{SKP_SILK_RESAMPLER_UP2_HQ_0, SKP_SILK_RESAMPLER_UP2_HQ_NOTCH, SKP_SILK_RESAMPLER_UP2_HQ_1};
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _SKP_Silk_resampler_state_struct {
    pub sIIR: [libc::c_int; 6],
    pub sFIR: [libc::c_int; 16],
    pub sDown2: [libc::c_int; 2],
    pub resampler_function: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut libc::c_short,
            *const libc::c_short,
            libc::c_int,
        ) -> (),
    >,
    pub up2_function: Option::<
        unsafe extern "C" fn(
            *mut libc::c_int,
            *mut libc::c_short,
            *const libc::c_short,
            libc::c_int,
        ) -> (),
    >,
    pub batchSize: libc::c_int,
    pub invRatio_Q16: libc::c_int,
    pub FIR_Fracs: libc::c_int,
    pub input2x: libc::c_int,
    pub Coefs: *const libc::c_short,
    pub sDownPre: [libc::c_int; 2],
    pub sUpPost: [libc::c_int; 2],
    pub down_pre_function: Option::<
        unsafe extern "C" fn(
            *mut libc::c_int,
            *mut libc::c_short,
            *const libc::c_short,
            libc::c_int,
        ) -> (),
    >,
    pub up_post_function: Option::<
        unsafe extern "C" fn(
            *mut libc::c_int,
            *mut libc::c_short,
            *const libc::c_short,
            libc::c_int,
        ) -> (),
    >,
    pub batchSizePrePost: libc::c_int,
    pub ratio_Q16: libc::c_int,
    pub nPreDownsamplers: libc::c_int,
    pub nPostUpsamplers: libc::c_int,
    pub magic_number: libc::c_int,
}
pub type SKP_Silk_resampler_state_struct = _SKP_Silk_resampler_state_struct;
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_resampler_private_up2_HQ(
    mut S: *mut libc::c_int,
    mut out: *mut libc::c_short,
    mut in_0: *const libc::c_short,
    mut len: libc::c_int,
) {
    let mut k: libc::c_int = 0;
    let mut in32: libc::c_int = 0;
    let mut out32_1: libc::c_int = 0;
    let mut out32_2: libc::c_int = 0;
    let mut Y: libc::c_int = 0;
    let mut X: libc::c_int = 0;
    k = 0 as libc::c_int;
    while k < len {
        in32 = (*in_0.offset(k as isize) as libc::c_int) << 10 as libc::c_int;
        Y = in32 - *S.offset(0 as libc::c_int as isize);
        X = (Y >> 16 as libc::c_int)
            * SKP_SILK_RESAMPLER_UP2_HQ_0[0 as libc::c_int as usize] as libc::c_int
            + ((Y & 0xffff as libc::c_int)
                * SKP_SILK_RESAMPLER_UP2_HQ_0[0 as libc::c_int as usize] as libc::c_int
                >> 16 as libc::c_int);
        out32_1 = *S.offset(0 as libc::c_int as isize) + X;
        *S.offset(0 as libc::c_int as isize) = in32 + X;
        Y = out32_1 - *S.offset(1 as libc::c_int as isize);
        X = Y
            + ((Y >> 16 as libc::c_int)
                * SKP_SILK_RESAMPLER_UP2_HQ_0[1 as libc::c_int as usize] as libc::c_int
                + ((Y & 0xffff as libc::c_int)
                    * SKP_SILK_RESAMPLER_UP2_HQ_0[1 as libc::c_int as usize]
                        as libc::c_int >> 16 as libc::c_int));
        out32_2 = *S.offset(1 as libc::c_int as isize) + X;
        *S.offset(1 as libc::c_int as isize) = out32_1 + X;
        out32_2 = out32_2
            + ((*S.offset(5 as libc::c_int as isize) >> 16 as libc::c_int)
                * SKP_SILK_RESAMPLER_UP2_HQ_NOTCH[2 as libc::c_int as usize]
                    as libc::c_int
                + ((*S.offset(5 as libc::c_int as isize) & 0xffff as libc::c_int)
                    * SKP_SILK_RESAMPLER_UP2_HQ_NOTCH[2 as libc::c_int as usize]
                        as libc::c_int >> 16 as libc::c_int));
        out32_2 = out32_2
            + ((*S.offset(4 as libc::c_int as isize) >> 16 as libc::c_int)
                * SKP_SILK_RESAMPLER_UP2_HQ_NOTCH[1 as libc::c_int as usize]
                    as libc::c_int
                + ((*S.offset(4 as libc::c_int as isize) & 0xffff as libc::c_int)
                    * SKP_SILK_RESAMPLER_UP2_HQ_NOTCH[1 as libc::c_int as usize]
                        as libc::c_int >> 16 as libc::c_int));
        out32_1 = out32_2
            + ((*S.offset(4 as libc::c_int as isize) >> 16 as libc::c_int)
                * SKP_SILK_RESAMPLER_UP2_HQ_NOTCH[0 as libc::c_int as usize]
                    as libc::c_int
                + ((*S.offset(4 as libc::c_int as isize) & 0xffff as libc::c_int)
                    * SKP_SILK_RESAMPLER_UP2_HQ_NOTCH[0 as libc::c_int as usize]
                        as libc::c_int >> 16 as libc::c_int));
        *S
            .offset(
                5 as libc::c_int as isize,
            ) = out32_2 - *S.offset(5 as libc::c_int as isize);
        *out
            .offset(
                (2 as libc::c_int * k) as isize,
            ) = (if 256 as libc::c_int
            + ((out32_1 >> 16 as libc::c_int)
                * SKP_SILK_RESAMPLER_UP2_HQ_NOTCH[3 as libc::c_int as usize]
                    as libc::c_int
                + ((out32_1 & 0xffff as libc::c_int)
                    * SKP_SILK_RESAMPLER_UP2_HQ_NOTCH[3 as libc::c_int as usize]
                        as libc::c_int >> 16 as libc::c_int)) >> 9 as libc::c_int
            > 0x7fff as libc::c_int
        {
            0x7fff as libc::c_int
        } else if (256 as libc::c_int
            + ((out32_1 >> 16 as libc::c_int)
                * SKP_SILK_RESAMPLER_UP2_HQ_NOTCH[3 as libc::c_int as usize]
                    as libc::c_int
                + ((out32_1 & 0xffff as libc::c_int)
                    * SKP_SILK_RESAMPLER_UP2_HQ_NOTCH[3 as libc::c_int as usize]
                        as libc::c_int >> 16 as libc::c_int)) >> 9 as libc::c_int)
            < 0x8000 as libc::c_int as libc::c_short as libc::c_int
        {
            0x8000 as libc::c_int as libc::c_short as libc::c_int
        } else {
            256 as libc::c_int
                + ((out32_1 >> 16 as libc::c_int)
                    * SKP_SILK_RESAMPLER_UP2_HQ_NOTCH[3 as libc::c_int as usize]
                        as libc::c_int
                    + ((out32_1 & 0xffff as libc::c_int)
                        * SKP_SILK_RESAMPLER_UP2_HQ_NOTCH[3 as libc::c_int as usize]
                            as libc::c_int >> 16 as libc::c_int)) >> 9 as libc::c_int
        }) as libc::c_short;
        Y = in32 - *S.offset(2 as libc::c_int as isize);
        X = (Y >> 16 as libc::c_int)
            * SKP_SILK_RESAMPLER_UP2_HQ_1[0 as libc::c_int as usize] as libc::c_int
            + ((Y & 0xffff as libc::c_int)
                * SKP_SILK_RESAMPLER_UP2_HQ_1[0 as libc::c_int as usize] as libc::c_int
                >> 16 as libc::c_int);
        out32_1 = *S.offset(2 as libc::c_int as isize) + X;
        *S.offset(2 as libc::c_int as isize) = in32 + X;
        Y = out32_1 - *S.offset(3 as libc::c_int as isize);
        X = Y
            + ((Y >> 16 as libc::c_int)
                * SKP_SILK_RESAMPLER_UP2_HQ_1[1 as libc::c_int as usize] as libc::c_int
                + ((Y & 0xffff as libc::c_int)
                    * SKP_SILK_RESAMPLER_UP2_HQ_1[1 as libc::c_int as usize]
                        as libc::c_int >> 16 as libc::c_int));
        out32_2 = *S.offset(3 as libc::c_int as isize) + X;
        *S.offset(3 as libc::c_int as isize) = out32_1 + X;
        out32_2 = out32_2
            + ((*S.offset(4 as libc::c_int as isize) >> 16 as libc::c_int)
                * SKP_SILK_RESAMPLER_UP2_HQ_NOTCH[2 as libc::c_int as usize]
                    as libc::c_int
                + ((*S.offset(4 as libc::c_int as isize) & 0xffff as libc::c_int)
                    * SKP_SILK_RESAMPLER_UP2_HQ_NOTCH[2 as libc::c_int as usize]
                        as libc::c_int >> 16 as libc::c_int));
        out32_2 = out32_2
            + ((*S.offset(5 as libc::c_int as isize) >> 16 as libc::c_int)
                * SKP_SILK_RESAMPLER_UP2_HQ_NOTCH[1 as libc::c_int as usize]
                    as libc::c_int
                + ((*S.offset(5 as libc::c_int as isize) & 0xffff as libc::c_int)
                    * SKP_SILK_RESAMPLER_UP2_HQ_NOTCH[1 as libc::c_int as usize]
                        as libc::c_int >> 16 as libc::c_int));
        out32_1 = out32_2
            + ((*S.offset(5 as libc::c_int as isize) >> 16 as libc::c_int)
                * SKP_SILK_RESAMPLER_UP2_HQ_NOTCH[0 as libc::c_int as usize]
                    as libc::c_int
                + ((*S.offset(5 as libc::c_int as isize) & 0xffff as libc::c_int)
                    * SKP_SILK_RESAMPLER_UP2_HQ_NOTCH[0 as libc::c_int as usize]
                        as libc::c_int >> 16 as libc::c_int));
        *S
            .offset(
                4 as libc::c_int as isize,
            ) = out32_2 - *S.offset(4 as libc::c_int as isize);
        *out
            .offset(
                (2 as libc::c_int * k + 1 as libc::c_int) as isize,
            ) = (if 256 as libc::c_int
            + ((out32_1 >> 16 as libc::c_int)
                * SKP_SILK_RESAMPLER_UP2_HQ_NOTCH[3 as libc::c_int as usize]
                    as libc::c_int
                + ((out32_1 & 0xffff as libc::c_int)
                    * SKP_SILK_RESAMPLER_UP2_HQ_NOTCH[3 as libc::c_int as usize]
                        as libc::c_int >> 16 as libc::c_int)) >> 9 as libc::c_int
            > 0x7fff as libc::c_int
        {
            0x7fff as libc::c_int
        } else if (256 as libc::c_int
            + ((out32_1 >> 16 as libc::c_int)
                * SKP_SILK_RESAMPLER_UP2_HQ_NOTCH[3 as libc::c_int as usize]
                    as libc::c_int
                + ((out32_1 & 0xffff as libc::c_int)
                    * SKP_SILK_RESAMPLER_UP2_HQ_NOTCH[3 as libc::c_int as usize]
                        as libc::c_int >> 16 as libc::c_int)) >> 9 as libc::c_int)
            < 0x8000 as libc::c_int as libc::c_short as libc::c_int
        {
            0x8000 as libc::c_int as libc::c_short as libc::c_int
        } else {
            256 as libc::c_int
                + ((out32_1 >> 16 as libc::c_int)
                    * SKP_SILK_RESAMPLER_UP2_HQ_NOTCH[3 as libc::c_int as usize]
                        as libc::c_int
                    + ((out32_1 & 0xffff as libc::c_int)
                        * SKP_SILK_RESAMPLER_UP2_HQ_NOTCH[3 as libc::c_int as usize]
                            as libc::c_int >> 16 as libc::c_int)) >> 9 as libc::c_int
        }) as libc::c_short;
        k += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_resampler_private_up2_HQ_wrapper(
    mut SS: *mut libc::c_void,
    mut out: *mut libc::c_short,
    mut in_0: *const libc::c_short,
    mut len: libc::c_int,
) {
    let mut S: *mut SKP_Silk_resampler_state_struct = SS
        as *mut SKP_Silk_resampler_state_struct;
    SKP_Silk_resampler_private_up2_HQ(((*S).sIIR).as_mut_ptr(), out, in_0, len);
}
