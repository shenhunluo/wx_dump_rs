#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}

use crate::SKP_Silk_resampler_private_up4::SKP_Silk_resampler_private_up4;
use crate::SKP_Silk_resampler_private_down4::SKP_Silk_resampler_private_down4;
use crate::SKP_Silk_resampler_down2::SKP_Silk_resampler_down2;
use crate::SKP_Silk_resampler_private_up2_HQ::{SKP_Silk_resampler_private_up2_HQ_wrapper,SKP_Silk_resampler_private_up2_HQ};
use crate::SKP_Silk_resampler_private_down_FIR::SKP_Silk_resampler_private_down_FIR;
use crate::SKP_Silk_resampler_private_copy::SKP_Silk_resampler_private_copy;
use crate::skp_silk_resampler_rom::{SKP_SILK_RESAMPLER_3_4_COEFS, SKP_SILK_RESAMPLER_2_3_COEFS, SKP_SILK_RESAMPLER_1_2_COEFS, SKP_SILK_RESAMPLER_3_8_COEFS, SKP_SILK_RESAMPLER_1_3_COEFS, SKP_SILK_RESAMPLER_80_441_ARMA4_COEFS, SKP_SILK_RESAMPLER_120_441_ARMA4_COEFS, SKP_SILK_RESAMPLER_160_441_ARMA4_COEFS, SKP_SILK_RESAMPLER_240_441_ARMA4_COEFS, SKP_SILK_RESAMPLER_320_441_ARMA4_COEFS};
use crate::SKP_Silk_resampler_up2::SKP_Silk_resampler_up2;
use crate::SKP_Silk_resampler_private_IIR_FIR::SKP_Silk_resampler_private_IIR_FIR;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct SKP_Silk_resampler_state_struct {
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

impl Default for SKP_Silk_resampler_state_struct {
    fn default() -> Self {
        Self { sIIR: Default::default(), sFIR: Default::default(), sDown2: Default::default(), resampler_function: Default::default(), up2_function: Default::default(), batchSize: Default::default(), invRatio_Q16: Default::default(), FIR_Fracs: Default::default(), input2x: Default::default(), Coefs: 0 as *const i16, sDownPre: Default::default(), sUpPost: Default::default(), down_pre_function: Default::default(), up_post_function: Default::default(), batchSizePrePost: Default::default(), ratio_Q16: Default::default(), nPreDownsamplers: Default::default(), nPostUpsamplers: Default::default(), magic_number: Default::default() }
    }
}

unsafe extern "C" fn gcd(mut a: libc::c_int, mut b: libc::c_int) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    while b > 0 as libc::c_int {
        tmp = a - b * (a / b);
        a = b;
        b = tmp;
    }
    return a;
}
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_resampler_init(
    mut S: *mut SKP_Silk_resampler_state_struct,
    mut Fs_Hz_in: libc::c_int,
    mut Fs_Hz_out: libc::c_int,
) -> libc::c_int {
    let mut cycleLen: libc::c_int = 0;
    let mut cyclesPerBatch: libc::c_int = 0;
    let mut up2: libc::c_int = 0 as libc::c_int;
    let mut down2: libc::c_int = 0 as libc::c_int;
    memset(
        S as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<SKP_Silk_resampler_state_struct>() as libc::c_ulong,
    );
    if Fs_Hz_in < 8000 as libc::c_int || Fs_Hz_in > 192000 as libc::c_int
        || Fs_Hz_out < 8000 as libc::c_int || Fs_Hz_out > 192000 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    if Fs_Hz_in > 96000 as libc::c_int {
        (*S).nPreDownsamplers = 2 as libc::c_int;
        (*S)
            .down_pre_function = Some(
            SKP_Silk_resampler_private_down4
                as unsafe extern "C" fn(
                    *mut libc::c_int,
                    *mut libc::c_short,
                    *const libc::c_short,
                    libc::c_int,
                ) -> (),
        );
    } else if Fs_Hz_in > 48000 as libc::c_int {
        (*S).nPreDownsamplers = 1 as libc::c_int;
        (*S)
            .down_pre_function = Some(
            SKP_Silk_resampler_down2
                as unsafe extern "C" fn(
                    *mut libc::c_int,
                    *mut libc::c_short,
                    *const libc::c_short,
                    libc::c_int,
                ) -> (),
        );
    } else {
        (*S).nPreDownsamplers = 0 as libc::c_int;
    }
    if Fs_Hz_out > 96000 as libc::c_int {
        (*S).nPostUpsamplers = 2 as libc::c_int;
        (*S)
            .up_post_function = Some(
            SKP_Silk_resampler_private_up4
                as unsafe extern "C" fn(
                    *mut libc::c_int,
                    *mut libc::c_short,
                    *const libc::c_short,
                    libc::c_int,
                ) -> (),
        );
    } else if Fs_Hz_out > 48000 as libc::c_int {
        (*S).nPostUpsamplers = 1 as libc::c_int;
        (*S)
            .up_post_function = Some(
            SKP_Silk_resampler_up2
                as unsafe extern "C" fn(
                    *mut libc::c_int,
                    *mut libc::c_short,
                    *const libc::c_short,
                    libc::c_int,
                ) -> (),
        );
    } else {
        (*S).nPostUpsamplers = 0 as libc::c_int;
    }
    if (*S).nPreDownsamplers + (*S).nPostUpsamplers > 0 as libc::c_int {
        (*S).ratio_Q16 = (Fs_Hz_out << 13 as libc::c_int) / Fs_Hz_in << 3 as libc::c_int;
        while ((*S).ratio_Q16 >> 16 as libc::c_int)
            * Fs_Hz_in as libc::c_short as libc::c_int
            + (((*S).ratio_Q16 & 0xffff as libc::c_int)
                * Fs_Hz_in as libc::c_short as libc::c_int >> 16 as libc::c_int)
            + (*S).ratio_Q16
                * (if 16 as libc::c_int == 1 as libc::c_int {
                    (Fs_Hz_in >> 1 as libc::c_int) + (Fs_Hz_in & 1 as libc::c_int)
                } else {
                    (Fs_Hz_in >> 16 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                        >> 1 as libc::c_int
                }) < Fs_Hz_out
        {
            (*S).ratio_Q16 += 1;
            (*S).ratio_Q16;
        }
        (*S).batchSizePrePost = Fs_Hz_in / 100 as libc::c_int;
        Fs_Hz_in = Fs_Hz_in >> (*S).nPreDownsamplers;
        Fs_Hz_out = Fs_Hz_out >> (*S).nPostUpsamplers;
    }
    (*S).batchSize = Fs_Hz_in / 100 as libc::c_int;
    if (*S).batchSize * 100 as libc::c_int != Fs_Hz_in
        || Fs_Hz_in % 100 as libc::c_int != 0 as libc::c_int
    {
        cycleLen = Fs_Hz_in / gcd(Fs_Hz_in, Fs_Hz_out);
        cyclesPerBatch = 480 as libc::c_int / cycleLen;
        if cyclesPerBatch == 0 as libc::c_int {
            (*S).batchSize = 480 as libc::c_int;
        } else {
            (*S).batchSize = cyclesPerBatch * cycleLen;
        }
    }
    if Fs_Hz_out > Fs_Hz_in {
        if Fs_Hz_out == Fs_Hz_in * 2 as libc::c_int {
            (*S)
                .resampler_function = Some(
                SKP_Silk_resampler_private_up2_HQ_wrapper
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut libc::c_short,
                        *const libc::c_short,
                        libc::c_int,
                    ) -> (),
            );
        } else {
            (*S)
                .resampler_function = Some(
                SKP_Silk_resampler_private_IIR_FIR
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut libc::c_short,
                        *const libc::c_short,
                        libc::c_int,
                    ) -> (),
            );
            up2 = 1 as libc::c_int;
            if Fs_Hz_in > 24000 as libc::c_int {
                (*S)
                    .up2_function = Some(
                    SKP_Silk_resampler_up2
                        as unsafe extern "C" fn(
                            *mut libc::c_int,
                            *mut libc::c_short,
                            *const libc::c_short,
                            libc::c_int,
                        ) -> (),
                );
            } else {
                (*S)
                    .up2_function = Some(
                    SKP_Silk_resampler_private_up2_HQ
                        as unsafe extern "C" fn(
                            *mut libc::c_int,
                            *mut libc::c_short,
                            *const libc::c_short,
                            libc::c_int,
                        ) -> (),
                );
            }
        }
    } else if Fs_Hz_out < Fs_Hz_in {
        if Fs_Hz_out * 4 as libc::c_int == Fs_Hz_in * 3 as libc::c_int {
            (*S).FIR_Fracs = 3 as libc::c_int;
            (*S).Coefs = SKP_SILK_RESAMPLER_3_4_COEFS.as_ptr();
            (*S)
                .resampler_function = Some(
                SKP_Silk_resampler_private_down_FIR
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut libc::c_short,
                        *const libc::c_short,
                        libc::c_int,
                    ) -> (),
            );
        } else if Fs_Hz_out * 3 as libc::c_int == Fs_Hz_in * 2 as libc::c_int {
            (*S).FIR_Fracs = 2 as libc::c_int;
            (*S).Coefs = SKP_SILK_RESAMPLER_2_3_COEFS.as_ptr();
            (*S)
                .resampler_function = Some(
                SKP_Silk_resampler_private_down_FIR
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut libc::c_short,
                        *const libc::c_short,
                        libc::c_int,
                    ) -> (),
            );
        } else if Fs_Hz_out * 2 as libc::c_int == Fs_Hz_in {
            (*S).FIR_Fracs = 1 as libc::c_int;
            (*S).Coefs = SKP_SILK_RESAMPLER_1_2_COEFS.as_ptr();
            (*S)
                .resampler_function = Some(
                SKP_Silk_resampler_private_down_FIR
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut libc::c_short,
                        *const libc::c_short,
                        libc::c_int,
                    ) -> (),
            );
        } else if Fs_Hz_out * 8 as libc::c_int == Fs_Hz_in * 3 as libc::c_int {
            (*S).FIR_Fracs = 3 as libc::c_int;
            (*S).Coefs = SKP_SILK_RESAMPLER_3_8_COEFS.as_ptr();
            (*S)
                .resampler_function = Some(
                SKP_Silk_resampler_private_down_FIR
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut libc::c_short,
                        *const libc::c_short,
                        libc::c_int,
                    ) -> (),
            );
        } else if Fs_Hz_out * 3 as libc::c_int == Fs_Hz_in {
            (*S).FIR_Fracs = 1 as libc::c_int;
            (*S).Coefs = SKP_SILK_RESAMPLER_1_3_COEFS.as_ptr();
            (*S)
                .resampler_function = Some(
                SKP_Silk_resampler_private_down_FIR
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut libc::c_short,
                        *const libc::c_short,
                        libc::c_int,
                    ) -> (),
            );
        } else if Fs_Hz_out * 4 as libc::c_int == Fs_Hz_in {
            (*S).FIR_Fracs = 1 as libc::c_int;
            down2 = 1 as libc::c_int;
            (*S).Coefs = SKP_SILK_RESAMPLER_1_2_COEFS.as_ptr();
            (*S)
                .resampler_function = Some(
                SKP_Silk_resampler_private_down_FIR
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut libc::c_short,
                        *const libc::c_short,
                        libc::c_int,
                    ) -> (),
            );
        } else if Fs_Hz_out * 6 as libc::c_int == Fs_Hz_in {
            (*S).FIR_Fracs = 1 as libc::c_int;
            down2 = 1 as libc::c_int;
            (*S).Coefs = SKP_SILK_RESAMPLER_1_3_COEFS.as_ptr();
            (*S)
                .resampler_function = Some(
                SKP_Silk_resampler_private_down_FIR
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut libc::c_short,
                        *const libc::c_short,
                        libc::c_int,
                    ) -> (),
            );
        } else if Fs_Hz_out * 441 as libc::c_int == Fs_Hz_in * 80 as libc::c_int {
            (*S).Coefs = SKP_SILK_RESAMPLER_80_441_ARMA4_COEFS.as_ptr();
            (*S)
                .resampler_function = Some(
                SKP_Silk_resampler_private_IIR_FIR
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut libc::c_short,
                        *const libc::c_short,
                        libc::c_int,
                    ) -> (),
            );
        } else if Fs_Hz_out * 441 as libc::c_int == Fs_Hz_in * 120 as libc::c_int {
            (*S).Coefs = SKP_SILK_RESAMPLER_120_441_ARMA4_COEFS.as_ptr();
            (*S)
                .resampler_function = Some(
                SKP_Silk_resampler_private_IIR_FIR
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut libc::c_short,
                        *const libc::c_short,
                        libc::c_int,
                    ) -> (),
            );
        } else if Fs_Hz_out * 441 as libc::c_int == Fs_Hz_in * 160 as libc::c_int {
            (*S).Coefs = SKP_SILK_RESAMPLER_160_441_ARMA4_COEFS.as_ptr();
            (*S)
                .resampler_function = Some(
                SKP_Silk_resampler_private_IIR_FIR
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut libc::c_short,
                        *const libc::c_short,
                        libc::c_int,
                    ) -> (),
            );
        } else if Fs_Hz_out * 441 as libc::c_int == Fs_Hz_in * 240 as libc::c_int {
            (*S).Coefs = SKP_SILK_RESAMPLER_240_441_ARMA4_COEFS.as_ptr();
            (*S)
                .resampler_function = Some(
                SKP_Silk_resampler_private_IIR_FIR
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut libc::c_short,
                        *const libc::c_short,
                        libc::c_int,
                    ) -> (),
            );
        } else if Fs_Hz_out * 441 as libc::c_int == Fs_Hz_in * 320 as libc::c_int {
            (*S).Coefs = SKP_SILK_RESAMPLER_320_441_ARMA4_COEFS.as_ptr();
            (*S)
                .resampler_function = Some(
                SKP_Silk_resampler_private_IIR_FIR
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut libc::c_short,
                        *const libc::c_short,
                        libc::c_int,
                    ) -> (),
            );
        } else {
            (*S)
                .resampler_function = Some(
                SKP_Silk_resampler_private_IIR_FIR
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut libc::c_short,
                        *const libc::c_short,
                        libc::c_int,
                    ) -> (),
            );
            up2 = 1 as libc::c_int;
            if Fs_Hz_in > 24000 as libc::c_int {
                (*S)
                    .up2_function = Some(
                    SKP_Silk_resampler_up2
                        as unsafe extern "C" fn(
                            *mut libc::c_int,
                            *mut libc::c_short,
                            *const libc::c_short,
                            libc::c_int,
                        ) -> (),
                );
            } else {
                (*S)
                    .up2_function = Some(
                    SKP_Silk_resampler_private_up2_HQ
                        as unsafe extern "C" fn(
                            *mut libc::c_int,
                            *mut libc::c_short,
                            *const libc::c_short,
                            libc::c_int,
                        ) -> (),
                );
            }
        }
    } else {
        (*S)
            .resampler_function = Some(
            SKP_Silk_resampler_private_copy
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_short,
                    *const libc::c_short,
                    libc::c_int,
                ) -> (),
        );
    }
    (*S).input2x = up2 | down2;
    (*S)
        .invRatio_Q16 = (Fs_Hz_in << 14 as libc::c_int + up2 - down2) / Fs_Hz_out
        << 2 as libc::c_int;
    while ((*S).invRatio_Q16 >> 16 as libc::c_int)
        * (Fs_Hz_out << down2) as libc::c_short as libc::c_int
        + (((*S).invRatio_Q16 & 0xffff as libc::c_int)
            * (Fs_Hz_out << down2) as libc::c_short as libc::c_int >> 16 as libc::c_int)
        + (*S).invRatio_Q16
            * (if 16 as libc::c_int == 1 as libc::c_int {
                (Fs_Hz_out << down2 >> 1 as libc::c_int)
                    + (Fs_Hz_out << down2 & 1 as libc::c_int)
            } else {
                (Fs_Hz_out << down2 >> 16 as libc::c_int - 1 as libc::c_int)
                    + 1 as libc::c_int >> 1 as libc::c_int
            }) < Fs_Hz_in << up2
    {
        (*S).invRatio_Q16 += 1;
        (*S).invRatio_Q16;
    }
    (*S).magic_number = 123456789 as libc::c_int;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_resampler_clear(
    mut S: *mut SKP_Silk_resampler_state_struct,
) -> libc::c_int {
    memset(
        ((*S).sDown2).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<[libc::c_int; 2]>() as libc::c_ulong,
    );
    memset(
        ((*S).sIIR).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<[libc::c_int; 6]>() as libc::c_ulong,
    );
    memset(
        ((*S).sFIR).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<[libc::c_int; 16]>() as libc::c_ulong,
    );
    memset(
        ((*S).sDownPre).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<[libc::c_int; 2]>() as libc::c_ulong,
    );
    memset(
        ((*S).sUpPost).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<[libc::c_int; 2]>() as libc::c_ulong,
    );
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_resampler(
    mut S: *mut SKP_Silk_resampler_state_struct,
    mut out: *mut libc::c_short,
    mut in_0: *const libc::c_short,
    mut inLen: libc::c_int,
) -> libc::c_int {
    if (*S).magic_number != 123456789 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if (*S).nPreDownsamplers + (*S).nPostUpsamplers > 0 as libc::c_int {
        let mut nSamplesIn: libc::c_int = 0;
        let mut nSamplesOut: libc::c_int = 0;
        let mut in_buf: [libc::c_short; 480] = [0; 480];
        let mut out_buf: [libc::c_short; 480] = [0; 480];
        while inLen > 0 as libc::c_int {
            nSamplesIn = if inLen < (*S).batchSizePrePost {
                inLen
            } else {
                (*S).batchSizePrePost
            };
            nSamplesOut = ((*S).ratio_Q16 >> 16 as libc::c_int)
                * nSamplesIn as libc::c_short as libc::c_int
                + (((*S).ratio_Q16 & 0xffff as libc::c_int)
                    * nSamplesIn as libc::c_short as libc::c_int >> 16 as libc::c_int);
            if (*S).nPreDownsamplers > 0 as libc::c_int {
                ((*S).down_pre_function)
                    .expect(
                        "non-null function pointer",
                    )(
                    ((*S).sDownPre).as_mut_ptr(),
                    in_buf.as_mut_ptr(),
                    in_0,
                    nSamplesIn,
                );
                if (*S).nPostUpsamplers > 0 as libc::c_int {
                    ((*S).resampler_function)
                        .expect(
                            "non-null function pointer",
                        )(
                        S as *mut libc::c_void,
                        out_buf.as_mut_ptr(),
                        in_buf.as_mut_ptr(),
                        nSamplesIn >> (*S).nPreDownsamplers,
                    );
                    ((*S).up_post_function)
                        .expect(
                            "non-null function pointer",
                        )(
                        ((*S).sUpPost).as_mut_ptr(),
                        out,
                        out_buf.as_mut_ptr(),
                        nSamplesOut >> (*S).nPostUpsamplers,
                    );
                } else {
                    ((*S).resampler_function)
                        .expect(
                            "non-null function pointer",
                        )(
                        S as *mut libc::c_void,
                        out,
                        in_buf.as_mut_ptr(),
                        nSamplesIn >> (*S).nPreDownsamplers,
                    );
                }
            } else {
                ((*S).resampler_function)
                    .expect(
                        "non-null function pointer",
                    )(
                    S as *mut libc::c_void,
                    out_buf.as_mut_ptr(),
                    in_0,
                    nSamplesIn >> (*S).nPreDownsamplers,
                );
                ((*S).up_post_function)
                    .expect(
                        "non-null function pointer",
                    )(
                    ((*S).sUpPost).as_mut_ptr(),
                    out,
                    out_buf.as_mut_ptr(),
                    nSamplesOut >> (*S).nPostUpsamplers,
                );
            }
            in_0 = in_0.offset(nSamplesIn as isize);
            out = out.offset(nSamplesOut as isize);
            inLen -= nSamplesIn;
        }
    } else {
        ((*S).resampler_function)
            .expect(
                "non-null function pointer",
            )(S as *mut libc::c_void, out, in_0, inLen);
    }
    return 0 as libc::c_int;
}
