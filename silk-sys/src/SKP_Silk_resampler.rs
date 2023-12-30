use crate::skp_silk_resampler_private_up4::skp_silk_resampler_private_up4;
use crate::skp_silk_resampler_private_down4::skp_silk_resampler_private_down4;
use crate::skp_silk_resampler_down2::skp_silk_resampler_down2;
use crate::skp_silk_resampler_private_up2_hq::{skp_silk_resampler_private_up2_hq_wrapper,skp_silk_resampler_private_up2_hq};
use crate::skp_silk_resampler_private_down_fir::skp_silk_resampler_private_down_fir;
use crate::skp_silk_resampler_private_copy::skp_silk_resampler_private_copy;
use crate::{skp_l_shift, skp_s_mul_w_w};
use crate::skp_silk_resampler_rom::{SKP_SILK_RESAMPLER_3_4_COEFS, SKP_SILK_RESAMPLER_2_3_COEFS, SKP_SILK_RESAMPLER_1_2_COEFS, SKP_SILK_RESAMPLER_3_8_COEFS, SKP_SILK_RESAMPLER_1_3_COEFS, SKP_SILK_RESAMPLER_80_441_ARMA4_COEFS, SKP_SILK_RESAMPLER_120_441_ARMA4_COEFS, SKP_SILK_RESAMPLER_160_441_ARMA4_COEFS, SKP_SILK_RESAMPLER_240_441_ARMA4_COEFS, SKP_SILK_RESAMPLER_320_441_ARMA4_COEFS};
use crate::skp_silk_resampler_up2::skp_silk_resampler_up2;
use crate::skp_silk_resampler_private_iir_fir::skp_silk_resampler_private_iir_fir;

#[derive(Clone)]
pub struct SKP_Silk_resampler_state_struct {
    pub sIIR: [libc::c_int; 6],
    pub sFIR: [libc::c_int; 16],
    pub sDown2: [libc::c_int; 2],
    pub resampler_function: Option::<
        fn(
            &mut SKP_Silk_resampler_state_struct,
            &mut [i16],
            &[i16],
            usize,
        ) -> (),
    >,
    pub up2_function: Option::<
        fn(
            &mut [i32],
            &mut [i16],
            & [i16],
            usize,
        ) -> (),
    >,
    pub batchSize: libc::c_int,
    pub invRatio_Q16: libc::c_int,
    pub FIR_Fracs: libc::c_int,
    pub input2x: libc::c_int,
    pub Coefs: Option<&'static [i16]>,
    pub sDownPre: [libc::c_int; 2],
    pub sUpPost: [libc::c_int; 2],
    pub down_pre_function: Option::<
        fn(
            &mut [i32],
            &mut [i16],
            & [i16],
            usize,
        ) -> (),
    >,
    pub up_post_function: Option::<
        fn(
            &mut [i32],
            &mut [i16],
            & [i16],
            usize,
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
        Self { sIIR: Default::default(), sFIR: Default::default(), sDown2: Default::default(), resampler_function: Default::default(), up2_function: Default::default(), batchSize: Default::default(), invRatio_Q16: Default::default(), FIR_Fracs: Default::default(), input2x: Default::default(), Coefs: Default::default(), sDownPre: Default::default(), sUpPost: Default::default(), down_pre_function: Default::default(), up_post_function: Default::default(), batchSizePrePost: Default::default(), ratio_Q16: Default::default(), nPreDownsamplers: Default::default(), nPostUpsamplers: Default::default(), magic_number: Default::default() }
    }
}

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b > 0 {
        let tmp = a - b * (a / b);
        a = b;
        b = tmp;
    }
    return a;
}

pub fn SKP_Silk_resampler_init(
    mut S: &mut SKP_Silk_resampler_state_struct,
    mut Fs_Hz_in: libc::c_int,
    mut Fs_Hz_out: libc::c_int,
) -> libc::c_int {
    let mut up2 = 0;
    let mut down2 = 0;
    *S = SKP_Silk_resampler_state_struct::default();
    if Fs_Hz_in < 8000 || Fs_Hz_in > 192000 || Fs_Hz_out < 8000 || Fs_Hz_out > 192000
    {
        return -1;
    }
    if Fs_Hz_in > 96000 {
        S.nPreDownsamplers = 2 as libc::c_int;
        S.down_pre_function = Some(skp_silk_resampler_private_down4);
    } else if Fs_Hz_in > 48000 as libc::c_int {
        (*S).nPreDownsamplers = 1 as libc::c_int;
        (*S)
            .down_pre_function = Some(skp_silk_resampler_down2);
    } else {
        (*S).nPreDownsamplers = 0 as libc::c_int;
    }
    if Fs_Hz_out > 96000 as libc::c_int {
        (*S).nPostUpsamplers = 2 as libc::c_int;
        (*S)
            .up_post_function = Some(
            skp_silk_resampler_private_up4
        );
    } else if Fs_Hz_out > 48000 as libc::c_int {
        (*S).nPostUpsamplers = 1 as libc::c_int;
        (*S)
            .up_post_function = Some(
            skp_silk_resampler_up2
        );
    } else {
        (*S).nPostUpsamplers = 0 as libc::c_int;
    }
    if (*S).nPreDownsamplers + (*S).nPostUpsamplers > 0 as libc::c_int {
        (*S).ratio_Q16 = (Fs_Hz_out << 13 as libc::c_int) / Fs_Hz_in << 3 as libc::c_int;
        while skp_s_mul_w_w!( S.ratio_Q16, Fs_Hz_in ) < Fs_Hz_out {
            S.ratio_Q16 += 1;
        }
        (*S).batchSizePrePost = Fs_Hz_in / 100 as libc::c_int;
        Fs_Hz_in = Fs_Hz_in >> (*S).nPreDownsamplers;
        Fs_Hz_out = Fs_Hz_out >> (*S).nPostUpsamplers;
    }
    (*S).batchSize = Fs_Hz_in / 100 as libc::c_int;
    if (*S).batchSize * 100 as libc::c_int != Fs_Hz_in
        || Fs_Hz_in % 100 as libc::c_int != 0 as libc::c_int
    {
        let cycleLen = Fs_Hz_in / gcd(Fs_Hz_in, Fs_Hz_out);
        let cyclesPerBatch = 480 as libc::c_int / cycleLen;
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
                skp_silk_resampler_private_up2_hq_wrapper,
            );
        } else {
            (*S)
                .resampler_function = Some(
                skp_silk_resampler_private_iir_fir
            );
            up2 = 1 as libc::c_int;
            if Fs_Hz_in > 24000 as libc::c_int {
                (*S)
                    .up2_function = Some(
                    skp_silk_resampler_up2
                );
            } else {
                (*S)
                    .up2_function = Some(
                    skp_silk_resampler_private_up2_hq
                );
            }
        }
    } else if Fs_Hz_out < Fs_Hz_in {
        if Fs_Hz_out * 4 as libc::c_int == Fs_Hz_in * 3 as libc::c_int {
            (*S).FIR_Fracs = 3 as libc::c_int;
            (*S).Coefs = Some(&SKP_SILK_RESAMPLER_3_4_COEFS);
            (*S)
                .resampler_function = Some(
                skp_silk_resampler_private_down_fir
            );
        } else if Fs_Hz_out * 3 as libc::c_int == Fs_Hz_in * 2 as libc::c_int {
            (*S).FIR_Fracs = 2 as libc::c_int;
            (*S).Coefs = Some(&SKP_SILK_RESAMPLER_2_3_COEFS);
            (*S)
                .resampler_function = Some(
                skp_silk_resampler_private_down_fir
            );
        } else if Fs_Hz_out * 2 as libc::c_int == Fs_Hz_in {
            (*S).FIR_Fracs = 1 as libc::c_int;
            (*S).Coefs = Some(&SKP_SILK_RESAMPLER_1_2_COEFS);
            (*S)
                .resampler_function = Some(
                skp_silk_resampler_private_down_fir
            );
        } else if Fs_Hz_out * 8 as libc::c_int == Fs_Hz_in * 3 as libc::c_int {
            (*S).FIR_Fracs = 3 as libc::c_int;
            (*S).Coefs = Some(&SKP_SILK_RESAMPLER_3_8_COEFS);
            (*S)
                .resampler_function = Some(
                skp_silk_resampler_private_down_fir
            );
        } else if Fs_Hz_out * 3 as libc::c_int == Fs_Hz_in {
            (*S).FIR_Fracs = 1 as libc::c_int;
            (*S).Coefs = Some(&SKP_SILK_RESAMPLER_1_3_COEFS);
            (*S)
                .resampler_function = Some(
                skp_silk_resampler_private_down_fir
            );
        } else if Fs_Hz_out * 4 as libc::c_int == Fs_Hz_in {
            (*S).FIR_Fracs = 1 as libc::c_int;
            down2 = 1 as libc::c_int;
            (*S).Coefs = Some(&SKP_SILK_RESAMPLER_1_2_COEFS);
            (*S)
                .resampler_function = Some(
                skp_silk_resampler_private_down_fir
            );
        } else if Fs_Hz_out * 6 as libc::c_int == Fs_Hz_in {
            (*S).FIR_Fracs = 1 as libc::c_int;
            down2 = 1 as libc::c_int;
            (*S).Coefs = Some(&SKP_SILK_RESAMPLER_1_3_COEFS);
            (*S)
                .resampler_function = Some(
                skp_silk_resampler_private_down_fir
            );
        } else if Fs_Hz_out * 441 as libc::c_int == Fs_Hz_in * 80 as libc::c_int {
            (*S).Coefs = Some(&SKP_SILK_RESAMPLER_80_441_ARMA4_COEFS);
            (*S)
                .resampler_function = Some(
                skp_silk_resampler_private_iir_fir
            );
        } else if Fs_Hz_out * 441 as libc::c_int == Fs_Hz_in * 120 as libc::c_int {
            (*S).Coefs = Some(&SKP_SILK_RESAMPLER_120_441_ARMA4_COEFS);
            (*S)
                .resampler_function = Some(
                skp_silk_resampler_private_iir_fir
            );
        } else if Fs_Hz_out * 441 as libc::c_int == Fs_Hz_in * 160 as libc::c_int {
            (*S).Coefs = Some(&SKP_SILK_RESAMPLER_160_441_ARMA4_COEFS);
            (*S)
                .resampler_function = Some(
                skp_silk_resampler_private_iir_fir
            );
        } else if Fs_Hz_out * 441 as libc::c_int == Fs_Hz_in * 240 as libc::c_int {
            (*S).Coefs = Some(&SKP_SILK_RESAMPLER_240_441_ARMA4_COEFS);
            (*S)
                .resampler_function = Some(
                skp_silk_resampler_private_iir_fir
            );
        } else if Fs_Hz_out * 441 as libc::c_int == Fs_Hz_in * 320 as libc::c_int {
            (*S).Coefs = Some(&SKP_SILK_RESAMPLER_320_441_ARMA4_COEFS);
            (*S)
                .resampler_function = Some(
                skp_silk_resampler_private_iir_fir
            );
        } else {
            (*S)
                .resampler_function = Some(
                skp_silk_resampler_private_iir_fir
            );
            up2 = 1 as libc::c_int;
            if Fs_Hz_in > 24000 as libc::c_int {
                (*S)
                    .up2_function = Some(
                    skp_silk_resampler_up2,
                );
            } else {
                (*S)
                    .up2_function = Some(
                    skp_silk_resampler_private_up2_hq
                );
            }
        }
    } else {
        (*S)
            .resampler_function = Some(
            skp_silk_resampler_private_copy
        );
    }
    (*S).input2x = up2 | down2;
    (*S)
        .invRatio_Q16 = (Fs_Hz_in << 14 as libc::c_int + up2 - down2) / Fs_Hz_out
        << 2 as libc::c_int;
    while skp_s_mul_w_w!( S.invRatio_Q16,skp_l_shift!( Fs_Hz_out, down2 ) ) < skp_l_shift!( Fs_Hz_in, up2 ) {
        S.invRatio_Q16 += 1;
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
    mut S: &mut SKP_Silk_resampler_state_struct,
    mut out: &mut [i16],
    mut in_0: &[i16],
    mut inLen: usize,
) -> libc::c_int {
    if (*S).magic_number != 123456789 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if (*S).nPreDownsamplers + (*S).nPostUpsamplers > 0 as libc::c_int {
        let mut nSamplesIn = 0;
        let mut nSamplesOut: libc::c_int = 0;
        let mut in_buf: [libc::c_short; 480] = [0; 480];
        let mut out_buf: [libc::c_short; 480] = [0; 480];
        while inLen > 0 {
            nSamplesIn = if inLen < (*S).batchSizePrePost as usize {
                inLen
            } else {
                (*S).batchSizePrePost as usize
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
                    &mut S.sDownPre,
                    &mut in_buf,
                    in_0,
                    nSamplesIn,
                );
                if (*S).nPostUpsamplers > 0 as libc::c_int {
                    ((*S).resampler_function)
                        .expect(
                            "non-null function pointer",
                        )(
                        S,
                        &mut out_buf,
                        &in_buf,
                        nSamplesIn >> S.nPreDownsamplers,
                    );
                    ((*S).up_post_function)
                        .expect(
                            "non-null function pointer",
                        )(
                        &mut S.sUpPost,
                        out,
                        &out_buf,
                        (nSamplesOut >> (*S).nPostUpsamplers) as usize,
                    );
                } else {
                    ((*S).resampler_function)
                        .expect(
                            "non-null function pointer",
                        )(
                        S,
                        out,
                        &in_buf,
                        nSamplesIn >> (*S).nPreDownsamplers,
                    );
                }
            } else {
                ((*S).resampler_function)
                    .expect(
                        "non-null function pointer",
                    )(
                    S,
                    &mut out_buf,
                    &in_0,
                    (nSamplesIn >> (*S).nPreDownsamplers) as usize,
                );
                ((*S).up_post_function)
                    .expect(
                        "non-null function pointer",
                    )(
                    &mut S.sUpPost,
                    out,
                    &out_buf,
                    (nSamplesOut >> (*S).nPostUpsamplers) as usize,
                );
            }
            in_0 = &in_0[nSamplesIn as usize..];
            out = &mut out[nSamplesOut as usize..];
            inLen -= nSamplesIn;
        }
    } else {
        ((*S).resampler_function)
            .expect(
                "non-null function pointer",
            )(S, out, in_0, inLen as usize);
    }
    return 0 as libc::c_int;
}
