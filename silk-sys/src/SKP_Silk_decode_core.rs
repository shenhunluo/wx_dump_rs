#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use crate::SKP_Silk_tables_other::SKP_Silk_Quantization_Offsets_Q10;
#[deny(arithmetic_overflow)]

use crate::{SKP_Silk_dec_API::{SKP_Silk_decoder_state, SKP_Silk_decoder_control}, SKP_Silk_MA::SKP_Silk_MA_Prediction};
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
pub type __int64_t = libc::c_longlong;
pub type int64_t = __int64_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SKP_Silk_range_coder_state {
    pub bufferLength: libc::c_int,
    pub bufferIx: libc::c_int,
    pub base_Q32: libc::c_uint,
    pub range_Q16: libc::c_uint,
    pub error: libc::c_int,
    pub buffer: [libc::c_uchar; 1024],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SKP_Silk_NLSF_CBS {
    pub nVectors: libc::c_int,
    pub CB_NLSF_Q15: *const libc::c_short,
    pub Rates_Q5: *const libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SKP_Silk_NLSF_CB_struct {
    pub nStages: libc::c_int,
    pub CBStages: *const SKP_Silk_NLSF_CBS,
    pub NDeltaMin_Q15: *const libc::c_int,
    pub CDF: *const libc::c_ushort,
    pub StartPtr: *const *const libc::c_ushort,
    pub MiddleIx: *const libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SKP_Silk_PLC_struct {
    pub pitchL_Q8: libc::c_int,
    pub LTPCoef_Q14: [libc::c_short; 5],
    pub prevLPC_Q12: [libc::c_short; 16],
    pub last_frame_lost: libc::c_int,
    pub rand_seed: libc::c_int,
    pub randScale_Q14: libc::c_short,
    pub conc_energy: libc::c_int,
    pub conc_energy_shift: libc::c_int,
    pub prevLTP_scale_Q14: libc::c_short,
    pub prevGain_Q16: [libc::c_int; 4],
    pub fs_kHz: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SKP_Silk_CNG_struct {
    pub CNG_exc_buf_Q10: [libc::c_int; 480],
    pub CNG_smth_NLSF_Q15: [libc::c_int; 16],
    pub CNG_synth_state: [libc::c_int; 16],
    pub CNG_smth_Gain_Q16: libc::c_int,
    pub rand_seed: libc::c_int,
    pub fs_kHz: libc::c_int,
}
#[inline]
unsafe extern "C" fn SKP_Silk_CLZ16(mut in16: libc::c_short) -> libc::c_int {
    let mut out32: libc::c_int = 0 as libc::c_int;
    if in16 as libc::c_int == 0 as libc::c_int {
        return 16 as libc::c_int;
    }
    if in16 as libc::c_int & 0xff00 as libc::c_int != 0 {
        if in16 as libc::c_int & 0xf000 as libc::c_int != 0 {
            in16 = (in16 as libc::c_int >> 12 as libc::c_int) as libc::c_short;
        } else {
            out32 += 4 as libc::c_int;
            in16 = (in16 as libc::c_int >> 8 as libc::c_int) as libc::c_short;
        }
    } else if in16 as libc::c_int & 0xfff0 as libc::c_int != 0 {
        out32 += 8 as libc::c_int;
        in16 = (in16 as libc::c_int >> 4 as libc::c_int) as libc::c_short;
    } else {
        out32 += 12 as libc::c_int;
    }
    if in16 as libc::c_int & 0xc as libc::c_int != 0 {
        if in16 as libc::c_int & 0x8 as libc::c_int != 0 {
            return out32 + 0 as libc::c_int
        } else {
            return out32 + 1 as libc::c_int
        }
    } else if in16 as libc::c_int & 0xe as libc::c_int != 0 {
        return out32 + 2 as libc::c_int
    } else {
        return out32 + 3 as libc::c_int
    };
}
#[inline]
unsafe extern "C" fn SKP_Silk_CLZ32(mut in32: libc::c_int) -> libc::c_int {
    if in32 as libc::c_uint & 0xffff0000 as libc::c_uint != 0 {
        return SKP_Silk_CLZ16((in32 >> 16 as libc::c_int) as libc::c_short)
    } else {
        return SKP_Silk_CLZ16(in32 as libc::c_short) + 16 as libc::c_int
    };
}
#[inline]
unsafe extern "C" fn SKP_DIV32_varQ(
    a32: libc::c_int,
    b32: libc::c_int,
    Qres: libc::c_int,
) -> libc::c_int {
    let mut a_headrm: libc::c_int = 0;
    let mut b_headrm: libc::c_int = 0;
    let mut lshift: libc::c_int = 0;
    let mut b32_inv: libc::c_int = 0;
    let mut a32_nrm: libc::c_int = 0;
    let mut b32_nrm: libc::c_int = 0;
    let mut result: libc::c_int = 0;
    a_headrm = SKP_Silk_CLZ32(if a32 > 0 as libc::c_int { a32 } else { -a32 })
        - 1 as libc::c_int;
    a32_nrm = a32 << a_headrm;
    b_headrm = SKP_Silk_CLZ32(if b32 > 0 as libc::c_int { b32 } else { -b32 })
        - 1 as libc::c_int;
    b32_nrm = b32 << b_headrm;
    b32_inv = (0x7fffffff as libc::c_int >> 2 as libc::c_int)
        / (b32_nrm >> 16 as libc::c_int);
    result = (a32_nrm >> 16 as libc::c_int) * b32_inv as libc::c_short as libc::c_int
        + ((a32_nrm & 0xffff as libc::c_int) * b32_inv as libc::c_short as libc::c_int
            >> 16 as libc::c_int);
    a32_nrm
        -= (((b32_nrm as int64_t * result as int64_t).wrapping_shr(32) as libc::c_int) as libc::c_int)
            << 3 as libc::c_int;
    result = result
        + ((a32_nrm >> 16 as libc::c_int) * b32_inv as libc::c_short as libc::c_int
            + ((a32_nrm & 0xffff as libc::c_int)
                * b32_inv as libc::c_short as libc::c_int >> 16 as libc::c_int));
    lshift = 29 as libc::c_int + a_headrm - b_headrm - Qres;
    if lshift <= 0 as libc::c_int {
        return (if 0x80000000 as libc::c_uint as libc::c_int >> -lshift
            > 0x7fffffff as libc::c_int >> -lshift
        {
            if result > 0x80000000 as libc::c_uint as libc::c_int >> -lshift {
                0x80000000 as libc::c_uint as libc::c_int >> -lshift
            } else {
                if result < 0x7fffffff as libc::c_int >> -lshift {
                    0x7fffffff as libc::c_int >> -lshift
                } else {
                    result
                }
            }
        } else {
            if result > 0x7fffffff as libc::c_int >> -lshift {
                0x7fffffff as libc::c_int >> -lshift
            } else {
                if result < 0x80000000 as libc::c_uint as libc::c_int >> -lshift {
                    0x80000000 as libc::c_uint as libc::c_int >> -lshift
                } else {
                    result
                }
            }
        }) << -lshift
    } else if lshift < 32 as libc::c_int {
        return result >> lshift
    } else {
        return 0 as libc::c_int
    };
}
#[inline]
unsafe extern "C" fn SKP_INVERSE32_varQ(
    b32: libc::c_int,
    Qres: libc::c_int,
) -> libc::c_int {
    let mut b_headrm: libc::c_int = 0;
    let mut lshift: libc::c_int = 0;
    let mut b32_inv: libc::c_int = 0;
    let mut b32_nrm: libc::c_int = 0;
    let mut err_Q32: libc::c_int = 0;
    let mut result: libc::c_int = 0;
    b_headrm = SKP_Silk_CLZ32(if b32 > 0 as libc::c_int { b32 } else { -b32 })
        - 1 as libc::c_int;
    b32_nrm = b32 << b_headrm;
    b32_inv = (0x7fffffff as libc::c_int >> 2 as libc::c_int)
        / (b32_nrm >> 16 as libc::c_int);
    result = b32_inv << 16 as libc::c_int;
    err_Q32 = -((b32_nrm >> 16 as libc::c_int) * b32_inv as libc::c_short as libc::c_int
        + ((b32_nrm & 0xffff as libc::c_int) * b32_inv as libc::c_short as libc::c_int
            >> 16 as libc::c_int)) << 3 as libc::c_int;
    result = result
        + ((err_Q32 >> 16 as libc::c_int) * b32_inv as libc::c_short as libc::c_int
            + ((err_Q32 & 0xffff as libc::c_int)
                * b32_inv as libc::c_short as libc::c_int >> 16 as libc::c_int))
        + err_Q32
            * (if 16 as libc::c_int == 1 as libc::c_int {
                (b32_inv >> 1 as libc::c_int) + (b32_inv & 1 as libc::c_int)
            } else {
                (b32_inv >> 16 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                    >> 1 as libc::c_int
            });
    lshift = 61 as libc::c_int - b_headrm - Qres;
    if lshift <= 0 as libc::c_int {
        return (if 0x80000000 as libc::c_uint as libc::c_int >> -lshift
            > 0x7fffffff as libc::c_int >> -lshift
        {
            if result > 0x80000000 as libc::c_uint as libc::c_int >> -lshift {
                0x80000000 as libc::c_uint as libc::c_int >> -lshift
            } else {
                if result < 0x7fffffff as libc::c_int >> -lshift {
                    0x7fffffff as libc::c_int >> -lshift
                } else {
                    result
                }
            }
        } else {
            if result > 0x7fffffff as libc::c_int >> -lshift {
                0x7fffffff as libc::c_int >> -lshift
            } else {
                if result < 0x80000000 as libc::c_uint as libc::c_int >> -lshift {
                    0x80000000 as libc::c_uint as libc::c_int >> -lshift
                } else {
                    result
                }
            }
        }) << -lshift
    } else if lshift < 32 as libc::c_int {
        return result >> lshift
    } else {
        return 0 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_decode_core(
    mut psDec: *mut SKP_Silk_decoder_state,
    mut psDecCtrl: *mut SKP_Silk_decoder_control,
    mut xq: *mut libc::c_short,
    mut q: *const libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut lag: libc::c_int = 0 as libc::c_int;
    let mut start_idx: libc::c_int = 0;
    let mut sLTP_buf_idx: libc::c_int = 0;
    let mut NLSF_interpolation_flag: libc::c_int = 0;
    let mut sigtype: libc::c_int = 0;
    let mut A_Q12: *mut libc::c_short = 0 as *mut libc::c_short;
    let mut B_Q14: *mut libc::c_short = 0 as *mut libc::c_short;
    let mut pxq: *mut libc::c_short = 0 as *mut libc::c_short;
    let mut A_Q12_tmp: [libc::c_short; 16] = [0; 16];
    let mut sLTP: [libc::c_short; 480] = [0; 480];
    let mut LTP_pred_Q14: libc::c_int = 0;
    let mut Gain_Q16: libc::c_int = 0;
    let mut inv_gain_Q16: libc::c_int = 0;
    let mut inv_gain_Q32: libc::c_int = 0;
    let mut gain_adj_Q16: libc::c_int = 0;
    let mut rand_seed: libc::c_int = 0;
    let mut offset_Q10: libc::c_int = 0;
    let mut dither: libc::c_int = 0;
    let mut pred_lag_ptr: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut pexc_Q10: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut pres_Q10: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut vec_Q10: [libc::c_int; 120] = [0; 120];
    let mut FiltState: [libc::c_int; 16] = [0; 16];
    offset_Q10 = SKP_Silk_Quantization_Offsets_Q10[(*psDecCtrl).sigtype
        as usize][(*psDecCtrl).QuantOffsetType as usize] as libc::c_int;
    if (*psDecCtrl).NLSFInterpCoef_Q2 < (1 as libc::c_int) << 2 as libc::c_int {
        NLSF_interpolation_flag = 1 as libc::c_int;
    } else {
        NLSF_interpolation_flag = 0 as libc::c_int;
    }
    rand_seed = (*psDecCtrl).Seed;
    i = 0 as libc::c_int;
    while i < (*psDec).frame_length {
        rand_seed = (907633515 as libc::c_int as libc::c_uint)
            .wrapping_add(
                (rand_seed as libc::c_uint)
                    .wrapping_mul(196314165 as libc::c_int as libc::c_uint),
            ) as libc::c_int;
        dither = rand_seed >> 31 as libc::c_int;
        (*psDec)
            .exc_Q10[i
            as usize] = (*q.offset(i as isize) << 10 as libc::c_int) + offset_Q10;
        (*psDec).exc_Q10[i as usize] = ((*psDec).exc_Q10[i as usize] ^ dither) - dither;
        rand_seed += *q.offset(i as isize);
        i += 1;
    }
    pexc_Q10 = ((*psDec).exc_Q10).as_mut_ptr();
    pres_Q10 = ((*psDec).res_Q10).as_mut_ptr();
    pxq = &mut *((*psDec).outBuf).as_mut_ptr().offset((*psDec).frame_length as isize)
        as *mut libc::c_short;
    sLTP_buf_idx = (*psDec).frame_length;
    k = 0 as libc::c_int;
    while k < 4 as libc::c_int {
        A_Q12 = ((*psDecCtrl).PredCoef_Q12[(k >> 1 as libc::c_int) as usize])
            .as_mut_ptr();
        memcpy(
            A_Q12_tmp.as_mut_ptr() as *mut libc::c_void,
            A_Q12 as *const libc::c_void,
            ((*psDec).LPC_order as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_short>() as libc::c_ulong),
        );
        B_Q14 = &mut *((*psDecCtrl).LTPCoef_Q14)
            .as_mut_ptr()
            .offset((k * 5 as libc::c_int) as isize) as *mut libc::c_short;
        Gain_Q16 = (*psDecCtrl).Gains_Q16[k as usize];
        sigtype = (*psDecCtrl).sigtype;
        inv_gain_Q16 = SKP_INVERSE32_varQ(
            if Gain_Q16 > 1 as libc::c_int { Gain_Q16 } else { 1 as libc::c_int },
            32 as libc::c_int,
        );
        inv_gain_Q16 = if inv_gain_Q16 < 0x7fff as libc::c_int {
            inv_gain_Q16
        } else {
            0x7fff as libc::c_int
        };
        gain_adj_Q16 = (1 as libc::c_int) << 16 as libc::c_int;
        if inv_gain_Q16 != (*psDec).prev_inv_gain_Q16 {
            gain_adj_Q16 = SKP_DIV32_varQ(
                inv_gain_Q16,
                (*psDec).prev_inv_gain_Q16,
                16 as libc::c_int,
            );
        }
        if (*psDec).lossCnt != 0 && (*psDec).prev_sigtype == 0 as libc::c_int
            && (*psDecCtrl).sigtype == 1 as libc::c_int
            && k < 4 as libc::c_int >> 1 as libc::c_int
        {
            memset(
                B_Q14 as *mut libc::c_void,
                0 as libc::c_int,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(
                        ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                    ),
            );
            *B_Q14
                .offset(
                    (5 as libc::c_int / 2 as libc::c_int) as isize,
                ) = ((1 as libc::c_int as libc::c_short as libc::c_int)
                << 12 as libc::c_int) as libc::c_short;
            sigtype = 0 as libc::c_int;
            (*psDecCtrl).pitchL[k as usize] = (*psDec).lagPrev;
        }
        if sigtype == 0 as libc::c_int {
            lag = (*psDecCtrl).pitchL[k as usize];
            if k & 3 as libc::c_int - (NLSF_interpolation_flag << 1 as libc::c_int)
                == 0 as libc::c_int
            {
                start_idx = (*psDec).frame_length - lag - (*psDec).LPC_order
                    - 5 as libc::c_int / 2 as libc::c_int;
                memset(
                    FiltState.as_mut_ptr() as *mut libc::c_void,
                    0 as libc::c_int,
                    ((*psDec).LPC_order as libc::c_ulong)
                        .wrapping_mul(
                            ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                        ),
                );
                SKP_Silk_MA_Prediction(
                    &mut *((*psDec).outBuf)
                        .as_mut_ptr()
                        .offset(
                            (start_idx + k * ((*psDec).frame_length >> 2 as libc::c_int))
                                as isize,
                        ),
                    A_Q12,
                    FiltState.as_mut_ptr(),
                    sLTP.as_mut_ptr().offset(start_idx as isize),
                    (*psDec).frame_length - start_idx,
                    (*psDec).LPC_order,
                );
                inv_gain_Q32 = inv_gain_Q16 << 16 as libc::c_int;
                if k == 0 as libc::c_int {
                    inv_gain_Q32 = (inv_gain_Q32 >> 16 as libc::c_int)
                        * (*psDecCtrl).LTP_scale_Q14 as libc::c_short as libc::c_int
                        + ((inv_gain_Q32 & 0xffff as libc::c_int)
                            * (*psDecCtrl).LTP_scale_Q14 as libc::c_short as libc::c_int
                            >> 16 as libc::c_int) << 2 as libc::c_int;
                }
                i = 0 as libc::c_int;
                while i < lag + 5 as libc::c_int / 2 as libc::c_int {
                    (*psDec)
                        .sLTP_Q16[(sLTP_buf_idx - i - 1 as libc::c_int)
                        as usize] = (inv_gain_Q32 >> 16 as libc::c_int)
                        * sLTP[((*psDec).frame_length - i - 1 as libc::c_int) as usize]
                            as libc::c_int
                        + ((inv_gain_Q32 & 0xffff as libc::c_int)
                            * sLTP[((*psDec).frame_length - i - 1 as libc::c_int)
                                as usize] as libc::c_int >> 16 as libc::c_int);
                    i += 1;
                }
            } else if gain_adj_Q16 != (1 as libc::c_int) << 16 as libc::c_int {
                i = 0 as libc::c_int;
                while i < lag + 5 as libc::c_int / 2 as libc::c_int {
                    (*psDec)
                        .sLTP_Q16[(sLTP_buf_idx - i - 1 as libc::c_int)
                        as usize] = (gain_adj_Q16 >> 16 as libc::c_int)
                        * (*psDec)
                            .sLTP_Q16[(sLTP_buf_idx - i - 1 as libc::c_int) as usize]
                            as libc::c_short as libc::c_int
                        + ((gain_adj_Q16 & 0xffff as libc::c_int)
                            * (*psDec)
                                .sLTP_Q16[(sLTP_buf_idx - i - 1 as libc::c_int) as usize]
                                as libc::c_short as libc::c_int >> 16 as libc::c_int)
                        + gain_adj_Q16
                            * (if 16 as libc::c_int == 1 as libc::c_int {
                                ((*psDec)
                                    .sLTP_Q16[(sLTP_buf_idx - i - 1 as libc::c_int) as usize]
                                    >> 1 as libc::c_int)
                                    + ((*psDec)
                                        .sLTP_Q16[(sLTP_buf_idx - i - 1 as libc::c_int) as usize]
                                        & 1 as libc::c_int)
                            } else {
                                ((*psDec)
                                    .sLTP_Q16[(sLTP_buf_idx - i - 1 as libc::c_int) as usize]
                                    >> 16 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                                    >> 1 as libc::c_int
                            });
                    i += 1;
                }
            }
        }
        i = 0 as libc::c_int;
        while i < 16 as libc::c_int {
            (*psDec)
                .sLPC_Q14[i
                as usize] = (gain_adj_Q16 >> 16 as libc::c_int)
                * (*psDec).sLPC_Q14[i as usize] as libc::c_short as libc::c_int
                + ((gain_adj_Q16 & 0xffff as libc::c_int)
                    * (*psDec).sLPC_Q14[i as usize] as libc::c_short as libc::c_int
                    >> 16 as libc::c_int)
                + gain_adj_Q16
                    * (if 16 as libc::c_int == 1 as libc::c_int {
                        ((*psDec).sLPC_Q14[i as usize] >> 1 as libc::c_int)
                            + ((*psDec).sLPC_Q14[i as usize] & 1 as libc::c_int)
                    } else {
                        ((*psDec).sLPC_Q14[i as usize]
                            >> 16 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                            >> 1 as libc::c_int
                    });
            i += 1;
        }
        (*psDec).prev_inv_gain_Q16 = inv_gain_Q16;
        if sigtype == 0 as libc::c_int {
            pred_lag_ptr = &mut *((*psDec).sLTP_Q16)
                .as_mut_ptr()
                .offset(
                    (sLTP_buf_idx - lag + 5 as libc::c_int / 2 as libc::c_int) as isize,
                ) as *mut libc::c_int;
            i = 0 as libc::c_int;
            while i < (*psDec).subfr_length {
                LTP_pred_Q14 = (*pred_lag_ptr.offset(0 as libc::c_int as isize)
                    >> 16 as libc::c_int)
                    * *B_Q14.offset(0 as libc::c_int as isize) as libc::c_int
                    + ((*pred_lag_ptr.offset(0 as libc::c_int as isize)
                        & 0xffff as libc::c_int)
                        * *B_Q14.offset(0 as libc::c_int as isize) as libc::c_int
                        >> 16 as libc::c_int);
                LTP_pred_Q14 = LTP_pred_Q14
                    + ((*pred_lag_ptr.offset(-(1 as libc::c_int) as isize)
                        >> 16 as libc::c_int)
                        * *B_Q14.offset(1 as libc::c_int as isize) as libc::c_int
                        + ((*pred_lag_ptr.offset(-(1 as libc::c_int) as isize)
                            & 0xffff as libc::c_int)
                            * *B_Q14.offset(1 as libc::c_int as isize) as libc::c_int
                            >> 16 as libc::c_int));
                LTP_pred_Q14 = LTP_pred_Q14
                    + ((*pred_lag_ptr.offset(-(2 as libc::c_int) as isize)
                        >> 16 as libc::c_int)
                        * *B_Q14.offset(2 as libc::c_int as isize) as libc::c_int
                        + ((*pred_lag_ptr.offset(-(2 as libc::c_int) as isize)
                            & 0xffff as libc::c_int)
                            * *B_Q14.offset(2 as libc::c_int as isize) as libc::c_int
                            >> 16 as libc::c_int));
                LTP_pred_Q14 = LTP_pred_Q14
                    + ((*pred_lag_ptr.offset(-(3 as libc::c_int) as isize)
                        >> 16 as libc::c_int)
                        * *B_Q14.offset(3 as libc::c_int as isize) as libc::c_int
                        + ((*pred_lag_ptr.offset(-(3 as libc::c_int) as isize)
                            & 0xffff as libc::c_int)
                            * *B_Q14.offset(3 as libc::c_int as isize) as libc::c_int
                            >> 16 as libc::c_int));
                LTP_pred_Q14 = LTP_pred_Q14
                    + ((*pred_lag_ptr.offset(-(4 as libc::c_int) as isize)
                        >> 16 as libc::c_int)
                        * *B_Q14.offset(4 as libc::c_int as isize) as libc::c_int
                        + ((*pred_lag_ptr.offset(-(4 as libc::c_int) as isize)
                            & 0xffff as libc::c_int)
                            * *B_Q14.offset(4 as libc::c_int as isize) as libc::c_int
                            >> 16 as libc::c_int));
                pred_lag_ptr = pred_lag_ptr.offset(1);
                *pres_Q10
                    .offset(
                        i as isize,
                    ) = *pexc_Q10.offset(i as isize)
                    + (if 4 as libc::c_int == 1 as libc::c_int {
                        (LTP_pred_Q14 >> 1 as libc::c_int)
                            + (LTP_pred_Q14 & 1 as libc::c_int)
                    } else {
                        (LTP_pred_Q14 >> 4 as libc::c_int - 1 as libc::c_int)
                            + 1 as libc::c_int >> 1 as libc::c_int
                    });
                (*psDec)
                    .sLTP_Q16[sLTP_buf_idx
                    as usize] = *pres_Q10.offset(i as isize) << 6 as libc::c_int;
                sLTP_buf_idx += 1;
                i += 1;
            }
        } else {
            memcpy(
                pres_Q10 as *mut libc::c_void,
                pexc_Q10 as *const libc::c_void,
                ((*psDec).subfr_length as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
            );
        }
        SKP_Silk_decode_short_term_prediction(
            vec_Q10.as_mut_ptr(),
            pres_Q10,
            ((*psDec).sLPC_Q14).as_mut_ptr(),
            A_Q12_tmp.as_mut_ptr(),
            (*psDec).LPC_order,
            (*psDec).subfr_length,
        );
        i = 0 as libc::c_int;
        while i < (*psDec).subfr_length {
            *pxq
                .offset(
                    i as isize,
                ) = (if (if 10 as libc::c_int == 1 as libc::c_int {
                ((vec_Q10[i as usize] >> 16 as libc::c_int)
                    * Gain_Q16 as libc::c_short as libc::c_int
                    + ((vec_Q10[i as usize] & 0xffff as libc::c_int)
                        * Gain_Q16 as libc::c_short as libc::c_int >> 16 as libc::c_int)
                    + vec_Q10[i as usize]
                        * (if 16 as libc::c_int == 1 as libc::c_int {
                            (Gain_Q16 >> 1 as libc::c_int)
                                + (Gain_Q16 & 1 as libc::c_int)
                        } else {
                            (Gain_Q16 >> 16 as libc::c_int - 1 as libc::c_int)
                                + 1 as libc::c_int >> 1 as libc::c_int
                        }) >> 1 as libc::c_int)
                    + ((vec_Q10[i as usize] >> 16 as libc::c_int)
                        * Gain_Q16 as libc::c_short as libc::c_int
                        + ((vec_Q10[i as usize] & 0xffff as libc::c_int)
                            * Gain_Q16 as libc::c_short as libc::c_int
                            >> 16 as libc::c_int)
                        + vec_Q10[i as usize]
                            * (if 16 as libc::c_int == 1 as libc::c_int {
                                (Gain_Q16 >> 1 as libc::c_int)
                                    + (Gain_Q16 & 1 as libc::c_int)
                            } else {
                                (Gain_Q16 >> 16 as libc::c_int - 1 as libc::c_int)
                                    + 1 as libc::c_int >> 1 as libc::c_int
                            }) & 1 as libc::c_int)
            } else {
                ((vec_Q10[i as usize] >> 16 as libc::c_int)
                    * Gain_Q16 as libc::c_short as libc::c_int
                    + ((vec_Q10[i as usize] & 0xffff as libc::c_int)
                        * Gain_Q16 as libc::c_short as libc::c_int >> 16 as libc::c_int)
                    + vec_Q10[i as usize]
                        * (if 16 as libc::c_int == 1 as libc::c_int {
                            (Gain_Q16 >> 1 as libc::c_int)
                                + (Gain_Q16 & 1 as libc::c_int)
                        } else {
                            (Gain_Q16 >> 16 as libc::c_int - 1 as libc::c_int)
                                + 1 as libc::c_int >> 1 as libc::c_int
                        }) >> 10 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                    >> 1 as libc::c_int
            }) > 0x7fff as libc::c_int
            {
                0x7fff as libc::c_int
            } else if (if 10 as libc::c_int == 1 as libc::c_int {
                ((vec_Q10[i as usize] >> 16 as libc::c_int)
                    * Gain_Q16 as libc::c_short as libc::c_int
                    + ((vec_Q10[i as usize] & 0xffff as libc::c_int)
                        * Gain_Q16 as libc::c_short as libc::c_int >> 16 as libc::c_int)
                    + vec_Q10[i as usize]
                        * (if 16 as libc::c_int == 1 as libc::c_int {
                            (Gain_Q16 >> 1 as libc::c_int)
                                + (Gain_Q16 & 1 as libc::c_int)
                        } else {
                            (Gain_Q16 >> 16 as libc::c_int - 1 as libc::c_int)
                                + 1 as libc::c_int >> 1 as libc::c_int
                        }) >> 1 as libc::c_int)
                    + ((vec_Q10[i as usize] >> 16 as libc::c_int)
                        * Gain_Q16 as libc::c_short as libc::c_int
                        + ((vec_Q10[i as usize] & 0xffff as libc::c_int)
                            * Gain_Q16 as libc::c_short as libc::c_int
                            >> 16 as libc::c_int)
                        + vec_Q10[i as usize]
                            * (if 16 as libc::c_int == 1 as libc::c_int {
                                (Gain_Q16 >> 1 as libc::c_int)
                                    + (Gain_Q16 & 1 as libc::c_int)
                            } else {
                                (Gain_Q16 >> 16 as libc::c_int - 1 as libc::c_int)
                                    + 1 as libc::c_int >> 1 as libc::c_int
                            }) & 1 as libc::c_int)
            } else {
                ((vec_Q10[i as usize] >> 16 as libc::c_int)
                    * Gain_Q16 as libc::c_short as libc::c_int
                    + ((vec_Q10[i as usize] & 0xffff as libc::c_int)
                        * Gain_Q16 as libc::c_short as libc::c_int >> 16 as libc::c_int)
                    + vec_Q10[i as usize]
                        * (if 16 as libc::c_int == 1 as libc::c_int {
                            (Gain_Q16 >> 1 as libc::c_int)
                                + (Gain_Q16 & 1 as libc::c_int)
                        } else {
                            (Gain_Q16 >> 16 as libc::c_int - 1 as libc::c_int)
                                + 1 as libc::c_int >> 1 as libc::c_int
                        }) >> 10 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                    >> 1 as libc::c_int
            }) < 0x8000 as libc::c_int as libc::c_short as libc::c_int
            {
                0x8000 as libc::c_int as libc::c_short as libc::c_int
            } else if 10 as libc::c_int == 1 as libc::c_int {
                ((vec_Q10[i as usize] >> 16 as libc::c_int)
                    * Gain_Q16 as libc::c_short as libc::c_int
                    + ((vec_Q10[i as usize] & 0xffff as libc::c_int)
                        * Gain_Q16 as libc::c_short as libc::c_int >> 16 as libc::c_int)
                    + vec_Q10[i as usize]
                        * (if 16 as libc::c_int == 1 as libc::c_int {
                            (Gain_Q16 >> 1 as libc::c_int)
                                + (Gain_Q16 & 1 as libc::c_int)
                        } else {
                            (Gain_Q16 >> 16 as libc::c_int - 1 as libc::c_int)
                                + 1 as libc::c_int >> 1 as libc::c_int
                        }) >> 1 as libc::c_int)
                    + ((vec_Q10[i as usize] >> 16 as libc::c_int)
                        * Gain_Q16 as libc::c_short as libc::c_int
                        + ((vec_Q10[i as usize] & 0xffff as libc::c_int)
                            * Gain_Q16 as libc::c_short as libc::c_int
                            >> 16 as libc::c_int)
                        + vec_Q10[i as usize]
                            * (if 16 as libc::c_int == 1 as libc::c_int {
                                (Gain_Q16 >> 1 as libc::c_int)
                                    + (Gain_Q16 & 1 as libc::c_int)
                            } else {
                                (Gain_Q16 >> 16 as libc::c_int - 1 as libc::c_int)
                                    + 1 as libc::c_int >> 1 as libc::c_int
                            }) & 1 as libc::c_int)
            } else {
                ((vec_Q10[i as usize] >> 16 as libc::c_int)
                    * Gain_Q16 as libc::c_short as libc::c_int
                    + ((vec_Q10[i as usize] & 0xffff as libc::c_int)
                        * Gain_Q16 as libc::c_short as libc::c_int >> 16 as libc::c_int)
                    + vec_Q10[i as usize]
                        * (if 16 as libc::c_int == 1 as libc::c_int {
                            (Gain_Q16 >> 1 as libc::c_int)
                                + (Gain_Q16 & 1 as libc::c_int)
                        } else {
                            (Gain_Q16 >> 16 as libc::c_int - 1 as libc::c_int)
                                + 1 as libc::c_int >> 1 as libc::c_int
                        }) >> 10 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                    >> 1 as libc::c_int
            }) as libc::c_short;
            i += 1;
        }
        memcpy(
            ((*psDec).sLPC_Q14).as_mut_ptr() as *mut libc::c_void,
            &mut *((*psDec).sLPC_Q14).as_mut_ptr().offset((*psDec).subfr_length as isize)
                as *mut libc::c_int as *const libc::c_void,
            (16 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
        );
        pexc_Q10 = pexc_Q10.offset((*psDec).subfr_length as isize);
        pres_Q10 = pres_Q10.offset((*psDec).subfr_length as isize);
        pxq = pxq.offset((*psDec).subfr_length as isize);
        k += 1;
    }
    memcpy(
        xq as *mut libc::c_void,
        &mut *((*psDec).outBuf).as_mut_ptr().offset((*psDec).frame_length as isize)
            as *mut libc::c_short as *const libc::c_void,
        ((*psDec).frame_length as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as libc::c_ulong),
    );
}
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_decode_short_term_prediction(
    mut vec_Q10: *mut libc::c_int,
    mut pres_Q10: *mut libc::c_int,
    mut sLPC_Q14: *mut libc::c_int,
    mut A_Q12_tmp: *mut libc::c_short,
    mut LPC_order: libc::c_int,
    mut subfr_length: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut LPC_pred_Q10: libc::c_int = 0;
    let mut Atmp: libc::c_int = 0;
    if LPC_order == 16 as libc::c_int {
        i = 0 as libc::c_int;
        while i < subfr_length {
            Atmp = *(&mut *A_Q12_tmp.offset(0 as libc::c_int as isize)
                as *mut libc::c_short as *mut libc::c_int);
            LPC_pred_Q10 = (*sLPC_Q14
                .offset((16 as libc::c_int + i - 1 as libc::c_int) as isize)
                >> 16 as libc::c_int) * Atmp as libc::c_short as libc::c_int
                + ((*sLPC_Q14.offset((16 as libc::c_int + i - 1 as libc::c_int) as isize)
                    & 0xffff as libc::c_int) * Atmp as libc::c_short as libc::c_int
                    >> 16 as libc::c_int);
            LPC_pred_Q10 = LPC_pred_Q10
                + (*sLPC_Q14.offset((16 as libc::c_int + i - 2 as libc::c_int) as isize)
                    >> 16 as libc::c_int) * (Atmp >> 16 as libc::c_int)
                + ((*sLPC_Q14.offset((16 as libc::c_int + i - 2 as libc::c_int) as isize)
                    & 0xffff as libc::c_int) * (Atmp >> 16 as libc::c_int)
                    >> 16 as libc::c_int);
            Atmp = *(&mut *A_Q12_tmp.offset(2 as libc::c_int as isize)
                as *mut libc::c_short as *mut libc::c_int);
            LPC_pred_Q10 = LPC_pred_Q10
                + ((*sLPC_Q14.offset((16 as libc::c_int + i - 3 as libc::c_int) as isize)
                    >> 16 as libc::c_int) * Atmp as libc::c_short as libc::c_int
                    + ((*sLPC_Q14
                        .offset((16 as libc::c_int + i - 3 as libc::c_int) as isize)
                        & 0xffff as libc::c_int) * Atmp as libc::c_short as libc::c_int
                        >> 16 as libc::c_int));
            LPC_pred_Q10 = LPC_pred_Q10
                + (*sLPC_Q14.offset((16 as libc::c_int + i - 4 as libc::c_int) as isize)
                    >> 16 as libc::c_int) * (Atmp >> 16 as libc::c_int)
                + ((*sLPC_Q14.offset((16 as libc::c_int + i - 4 as libc::c_int) as isize)
                    & 0xffff as libc::c_int) * (Atmp >> 16 as libc::c_int)
                    >> 16 as libc::c_int);
            Atmp = *(&mut *A_Q12_tmp.offset(4 as libc::c_int as isize)
                as *mut libc::c_short as *mut libc::c_int);
            LPC_pred_Q10 = LPC_pred_Q10
                + ((*sLPC_Q14.offset((16 as libc::c_int + i - 5 as libc::c_int) as isize)
                    >> 16 as libc::c_int) * Atmp as libc::c_short as libc::c_int
                    + ((*sLPC_Q14
                        .offset((16 as libc::c_int + i - 5 as libc::c_int) as isize)
                        & 0xffff as libc::c_int) * Atmp as libc::c_short as libc::c_int
                        >> 16 as libc::c_int));
            LPC_pred_Q10 = LPC_pred_Q10
                + (*sLPC_Q14.offset((16 as libc::c_int + i - 6 as libc::c_int) as isize)
                    >> 16 as libc::c_int) * (Atmp >> 16 as libc::c_int)
                + ((*sLPC_Q14.offset((16 as libc::c_int + i - 6 as libc::c_int) as isize)
                    & 0xffff as libc::c_int) * (Atmp >> 16 as libc::c_int)
                    >> 16 as libc::c_int);
            Atmp = *(&mut *A_Q12_tmp.offset(6 as libc::c_int as isize)
                as *mut libc::c_short as *mut libc::c_int);
            LPC_pred_Q10 = LPC_pred_Q10
                + ((*sLPC_Q14.offset((16 as libc::c_int + i - 7 as libc::c_int) as isize)
                    >> 16 as libc::c_int) * Atmp as libc::c_short as libc::c_int
                    + ((*sLPC_Q14
                        .offset((16 as libc::c_int + i - 7 as libc::c_int) as isize)
                        & 0xffff as libc::c_int) * Atmp as libc::c_short as libc::c_int
                        >> 16 as libc::c_int));
            LPC_pred_Q10 = LPC_pred_Q10
                + (*sLPC_Q14.offset((16 as libc::c_int + i - 8 as libc::c_int) as isize)
                    >> 16 as libc::c_int) * (Atmp >> 16 as libc::c_int)
                + ((*sLPC_Q14.offset((16 as libc::c_int + i - 8 as libc::c_int) as isize)
                    & 0xffff as libc::c_int) * (Atmp >> 16 as libc::c_int)
                    >> 16 as libc::c_int);
            Atmp = *(&mut *A_Q12_tmp.offset(8 as libc::c_int as isize)
                as *mut libc::c_short as *mut libc::c_int);
            LPC_pred_Q10 = LPC_pred_Q10
                + ((*sLPC_Q14.offset((16 as libc::c_int + i - 9 as libc::c_int) as isize)
                    >> 16 as libc::c_int) * Atmp as libc::c_short as libc::c_int
                    + ((*sLPC_Q14
                        .offset((16 as libc::c_int + i - 9 as libc::c_int) as isize)
                        & 0xffff as libc::c_int) * Atmp as libc::c_short as libc::c_int
                        >> 16 as libc::c_int));
            LPC_pred_Q10 = LPC_pred_Q10
                + (*sLPC_Q14.offset((16 as libc::c_int + i - 10 as libc::c_int) as isize)
                    >> 16 as libc::c_int) * (Atmp >> 16 as libc::c_int)
                + ((*sLPC_Q14
                    .offset((16 as libc::c_int + i - 10 as libc::c_int) as isize)
                    & 0xffff as libc::c_int) * (Atmp >> 16 as libc::c_int)
                    >> 16 as libc::c_int);
            Atmp = *(&mut *A_Q12_tmp.offset(10 as libc::c_int as isize)
                as *mut libc::c_short as *mut libc::c_int);
            LPC_pred_Q10 = LPC_pred_Q10
                + ((*sLPC_Q14
                    .offset((16 as libc::c_int + i - 11 as libc::c_int) as isize)
                    >> 16 as libc::c_int) * Atmp as libc::c_short as libc::c_int
                    + ((*sLPC_Q14
                        .offset((16 as libc::c_int + i - 11 as libc::c_int) as isize)
                        & 0xffff as libc::c_int) * Atmp as libc::c_short as libc::c_int
                        >> 16 as libc::c_int));
            LPC_pred_Q10 = LPC_pred_Q10
                + (*sLPC_Q14.offset((16 as libc::c_int + i - 12 as libc::c_int) as isize)
                    >> 16 as libc::c_int) * (Atmp >> 16 as libc::c_int)
                + ((*sLPC_Q14
                    .offset((16 as libc::c_int + i - 12 as libc::c_int) as isize)
                    & 0xffff as libc::c_int) * (Atmp >> 16 as libc::c_int)
                    >> 16 as libc::c_int);
            Atmp = *(&mut *A_Q12_tmp.offset(12 as libc::c_int as isize)
                as *mut libc::c_short as *mut libc::c_int);
            LPC_pred_Q10 = LPC_pred_Q10
                + ((*sLPC_Q14
                    .offset((16 as libc::c_int + i - 13 as libc::c_int) as isize)
                    >> 16 as libc::c_int) * Atmp as libc::c_short as libc::c_int
                    + ((*sLPC_Q14
                        .offset((16 as libc::c_int + i - 13 as libc::c_int) as isize)
                        & 0xffff as libc::c_int) * Atmp as libc::c_short as libc::c_int
                        >> 16 as libc::c_int));
            LPC_pred_Q10 = LPC_pred_Q10
                + (*sLPC_Q14.offset((16 as libc::c_int + i - 14 as libc::c_int) as isize)
                    >> 16 as libc::c_int) * (Atmp >> 16 as libc::c_int)
                + ((*sLPC_Q14
                    .offset((16 as libc::c_int + i - 14 as libc::c_int) as isize)
                    & 0xffff as libc::c_int) * (Atmp >> 16 as libc::c_int)
                    >> 16 as libc::c_int);
            Atmp = *(&mut *A_Q12_tmp.offset(14 as libc::c_int as isize)
                as *mut libc::c_short as *mut libc::c_int);
            LPC_pred_Q10 = LPC_pred_Q10
                + ((*sLPC_Q14
                    .offset((16 as libc::c_int + i - 15 as libc::c_int) as isize)
                    >> 16 as libc::c_int) * Atmp as libc::c_short as libc::c_int
                    + ((*sLPC_Q14
                        .offset((16 as libc::c_int + i - 15 as libc::c_int) as isize)
                        & 0xffff as libc::c_int) * Atmp as libc::c_short as libc::c_int
                        >> 16 as libc::c_int));
            LPC_pred_Q10 = LPC_pred_Q10
                + (*sLPC_Q14.offset((16 as libc::c_int + i - 16 as libc::c_int) as isize)
                    >> 16 as libc::c_int) * (Atmp >> 16 as libc::c_int)
                + ((*sLPC_Q14
                    .offset((16 as libc::c_int + i - 16 as libc::c_int) as isize)
                    & 0xffff as libc::c_int) * (Atmp >> 16 as libc::c_int)
                    >> 16 as libc::c_int);
            *vec_Q10.offset(i as isize) = *pres_Q10.offset(i as isize) + LPC_pred_Q10;
            *sLPC_Q14
                .offset(
                    (16 as libc::c_int + i) as isize,
                ) = *vec_Q10.offset(i as isize) << 4 as libc::c_int;
            i += 1;
        }
    } else {
        i = 0 as libc::c_int;
        while i < subfr_length {
            Atmp = *(&mut *A_Q12_tmp.offset(0 as libc::c_int as isize)
                as *mut libc::c_short as *mut libc::c_int);
            LPC_pred_Q10 = (*sLPC_Q14
                .offset((16 as libc::c_int + i - 1 as libc::c_int) as isize)
                >> 16 as libc::c_int) * Atmp as libc::c_short as libc::c_int
                + ((*sLPC_Q14.offset((16 as libc::c_int + i - 1 as libc::c_int) as isize)
                    & 0xffff as libc::c_int) * Atmp as libc::c_short as libc::c_int
                    >> 16 as libc::c_int);
            LPC_pred_Q10 = LPC_pred_Q10
                + (*sLPC_Q14.offset((16 as libc::c_int + i - 2 as libc::c_int) as isize)
                    >> 16 as libc::c_int) * (Atmp >> 16 as libc::c_int)
                + ((*sLPC_Q14.offset((16 as libc::c_int + i - 2 as libc::c_int) as isize)
                    & 0xffff as libc::c_int) * (Atmp >> 16 as libc::c_int)
                    >> 16 as libc::c_int);
            Atmp = *(&mut *A_Q12_tmp.offset(2 as libc::c_int as isize)
                as *mut libc::c_short as *mut libc::c_int);
            LPC_pred_Q10 = LPC_pred_Q10
                + ((*sLPC_Q14.offset((16 as libc::c_int + i - 3 as libc::c_int) as isize)
                    >> 16 as libc::c_int) * Atmp as libc::c_short as libc::c_int
                    + ((*sLPC_Q14
                        .offset((16 as libc::c_int + i - 3 as libc::c_int) as isize)
                        & 0xffff as libc::c_int) * Atmp as libc::c_short as libc::c_int
                        >> 16 as libc::c_int));
            LPC_pred_Q10 = LPC_pred_Q10
                + (*sLPC_Q14.offset((16 as libc::c_int + i - 4 as libc::c_int) as isize)
                    >> 16 as libc::c_int) * (Atmp >> 16 as libc::c_int)
                + ((*sLPC_Q14.offset((16 as libc::c_int + i - 4 as libc::c_int) as isize)
                    & 0xffff as libc::c_int) * (Atmp >> 16 as libc::c_int)
                    >> 16 as libc::c_int);
            Atmp = *(&mut *A_Q12_tmp.offset(4 as libc::c_int as isize)
                as *mut libc::c_short as *mut libc::c_int);
            LPC_pred_Q10 = LPC_pred_Q10
                + ((*sLPC_Q14.offset((16 as libc::c_int + i - 5 as libc::c_int) as isize)
                    >> 16 as libc::c_int) * Atmp as libc::c_short as libc::c_int
                    + ((*sLPC_Q14
                        .offset((16 as libc::c_int + i - 5 as libc::c_int) as isize)
                        & 0xffff as libc::c_int) * Atmp as libc::c_short as libc::c_int
                        >> 16 as libc::c_int));
            LPC_pred_Q10 = LPC_pred_Q10
                + (*sLPC_Q14.offset((16 as libc::c_int + i - 6 as libc::c_int) as isize)
                    >> 16 as libc::c_int) * (Atmp >> 16 as libc::c_int)
                + ((*sLPC_Q14.offset((16 as libc::c_int + i - 6 as libc::c_int) as isize)
                    & 0xffff as libc::c_int) * (Atmp >> 16 as libc::c_int)
                    >> 16 as libc::c_int);
            Atmp = *(&mut *A_Q12_tmp.offset(6 as libc::c_int as isize)
                as *mut libc::c_short as *mut libc::c_int);
            LPC_pred_Q10 = LPC_pred_Q10
                + ((*sLPC_Q14.offset((16 as libc::c_int + i - 7 as libc::c_int) as isize)
                    >> 16 as libc::c_int) * Atmp as libc::c_short as libc::c_int
                    + ((*sLPC_Q14
                        .offset((16 as libc::c_int + i - 7 as libc::c_int) as isize)
                        & 0xffff as libc::c_int) * Atmp as libc::c_short as libc::c_int
                        >> 16 as libc::c_int));
            LPC_pred_Q10 = LPC_pred_Q10
                + (*sLPC_Q14.offset((16 as libc::c_int + i - 8 as libc::c_int) as isize)
                    >> 16 as libc::c_int) * (Atmp >> 16 as libc::c_int)
                + ((*sLPC_Q14.offset((16 as libc::c_int + i - 8 as libc::c_int) as isize)
                    & 0xffff as libc::c_int) * (Atmp >> 16 as libc::c_int)
                    >> 16 as libc::c_int);
            Atmp = *(&mut *A_Q12_tmp.offset(8 as libc::c_int as isize)
                as *mut libc::c_short as *mut libc::c_int);
            LPC_pred_Q10 = LPC_pred_Q10
                + ((*sLPC_Q14.offset((16 as libc::c_int + i - 9 as libc::c_int) as isize)
                    >> 16 as libc::c_int) * Atmp as libc::c_short as libc::c_int
                    + ((*sLPC_Q14
                        .offset((16 as libc::c_int + i - 9 as libc::c_int) as isize)
                        & 0xffff as libc::c_int) * Atmp as libc::c_short as libc::c_int
                        >> 16 as libc::c_int));
            LPC_pred_Q10 = LPC_pred_Q10
                + (*sLPC_Q14.offset((16 as libc::c_int + i - 10 as libc::c_int) as isize)
                    >> 16 as libc::c_int) * (Atmp >> 16 as libc::c_int)
                + ((*sLPC_Q14
                    .offset((16 as libc::c_int + i - 10 as libc::c_int) as isize)
                    & 0xffff as libc::c_int) * (Atmp >> 16 as libc::c_int)
                    >> 16 as libc::c_int);
            *vec_Q10.offset(i as isize) = *pres_Q10.offset(i as isize) + LPC_pred_Q10;
            *sLPC_Q14
                .offset(
                    (16 as libc::c_int + i) as isize,
                ) = *vec_Q10.offset(i as isize) << 4 as libc::c_int;
            i += 1;
        }
    };
}
