#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]

use crate::{SKP_Silk_dec_API::{SKP_Silk_decoder_state, SKP_Silk_decoder_control}, skp_silk_bwexpander::skp_silk_bwexpander, SKP_Silk_sum_sqr_shift::SKP_Silk_sum_sqr_shift, skp_silk_lpc_inv_pred_gain::skp_silk_lpc_inverse_pred_gain};
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
pub struct SkpSilkNlsfCbStruct {
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

impl Default for SKP_Silk_PLC_struct {
    fn default() -> Self {
        Self { pitchL_Q8: Default::default(), LTPCoef_Q14: Default::default(), prevLPC_Q12: Default::default(), last_frame_lost: Default::default(), rand_seed: Default::default(), randScale_Q14: Default::default(), conc_energy: Default::default(), conc_energy_shift: Default::default(), prevLTP_scale_Q14: Default::default(), prevGain_Q16: Default::default(), fs_kHz: Default::default() }
    }
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub as_int16: [libc::c_short; 16],
    pub as_int32: [libc::c_int; 8],
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
unsafe extern "C" fn SKP_ROR32(
    mut a32: libc::c_int,
    mut rot: libc::c_int,
) -> libc::c_int {
    let mut x: libc::c_uint = a32 as libc::c_uint;
    let mut r: libc::c_uint = rot as libc::c_uint;
    let mut m: libc::c_uint = -rot as libc::c_uint;
    if rot <= 0 as libc::c_int {
        return (x << m | x >> (32 as libc::c_int as libc::c_uint).wrapping_sub(m))
            as libc::c_int
    } else {
        return (x << (32 as libc::c_int as libc::c_uint).wrapping_sub(r) | x >> r)
            as libc::c_int
    };
}
#[inline]
unsafe extern "C" fn SKP_min_int(mut a: libc::c_int, mut b: libc::c_int) -> libc::c_int {
    return if a < b { a } else { b };
}
#[inline]
unsafe extern "C" fn SKP_min_32(mut a: libc::c_int, mut b: libc::c_int) -> libc::c_int {
    return if a < b { a } else { b };
}
#[inline]
unsafe extern "C" fn SKP_max_int(mut a: libc::c_int, mut b: libc::c_int) -> libc::c_int {
    return if a > b { a } else { b };
}
#[inline]
unsafe extern "C" fn SKP_max_16(
    mut a: libc::c_short,
    mut b: libc::c_short,
) -> libc::c_short {
    return (if a as libc::c_int > b as libc::c_int {
        a as libc::c_int
    } else {
        b as libc::c_int
    }) as libc::c_short;
}
#[inline]
unsafe extern "C" fn SKP_max_32(mut a: libc::c_int, mut b: libc::c_int) -> libc::c_int {
    return if a > b { a } else { b };
}
#[inline]
unsafe extern "C" fn SKP_Silk_CLZ_FRAC(
    mut in_0: libc::c_int,
    mut lz: *mut libc::c_int,
    mut frac_Q7: *mut libc::c_int,
) {
    let mut lzeros: libc::c_int = SKP_Silk_CLZ32(in_0);
    *lz = lzeros;
    *frac_Q7 = SKP_ROR32(in_0, 24 as libc::c_int - lzeros) & 0x7f as libc::c_int;
}
#[inline]
unsafe extern "C" fn SKP_Silk_SQRT_APPROX(mut x: libc::c_int) -> libc::c_int {
    let mut y: libc::c_int = 0;
    let mut lz: libc::c_int = 0;
    let mut frac_Q7: libc::c_int = 0;
    if x <= 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    SKP_Silk_CLZ_FRAC(x, &mut lz, &mut frac_Q7);
    if lz & 1 as libc::c_int != 0 {
        y = 32768 as libc::c_int;
    } else {
        y = 46214 as libc::c_int;
    }
    y >>= lz >> 1 as libc::c_int;
    y = y
        + ((y >> 16 as libc::c_int)
            * (213 as libc::c_int as libc::c_short as libc::c_int
                * frac_Q7 as libc::c_short as libc::c_int) as libc::c_short
                as libc::c_int
            + ((y & 0xffff as libc::c_int)
                * (213 as libc::c_int as libc::c_short as libc::c_int
                    * frac_Q7 as libc::c_short as libc::c_int) as libc::c_short
                    as libc::c_int >> 16 as libc::c_int));
    return y;
}
static mut HARM_ATT_Q15: [libc::c_short; 2] = [
    32440 as libc::c_int as libc::c_short,
    31130 as libc::c_int as libc::c_short,
];
static mut PLC_RAND_ATTENUATE_V_Q15: [libc::c_short; 2] = [
    31130 as libc::c_int as libc::c_short,
    26214 as libc::c_int as libc::c_short,
];
static mut PLC_RAND_ATTENUATE_UV_Q15: [libc::c_short; 2] = [
    32440 as libc::c_int as libc::c_short,
    29491 as libc::c_int as libc::c_short,
];

pub fn SKP_Silk_PLC_Reset(psDec: &mut SKP_Silk_decoder_state) {
    psDec.sPLC.pitchL_Q8 = psDec.frame_length >> 1;
}

#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_PLC(
    mut psDec: &mut SKP_Silk_decoder_state,
    mut psDecCtrl: *mut SKP_Silk_decoder_control,
    mut signal: *mut libc::c_short,
    mut length: libc::c_int,
    mut lost: libc::c_int,
) {
    if (*psDec).fs_kHz != (*psDec).sPLC.fs_kHz {
        SKP_Silk_PLC_Reset(psDec);
        (*psDec).sPLC.fs_kHz = (*psDec).fs_kHz;
    }
    if lost != 0 {
        SKP_Silk_PLC_conceal(psDec, psDecCtrl, signal, length);
        (*psDec).lossCnt += 1;
        (*psDec).lossCnt;
    } else {
        SKP_Silk_PLC_update(psDec, psDecCtrl, signal, length);
    };
}
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_PLC_update(
    mut psDec: *mut SKP_Silk_decoder_state,
    mut psDecCtrl: *mut SKP_Silk_decoder_control,
    mut _signal: *mut libc::c_short,
    mut _length: libc::c_int,
) {
    let mut LTP_Gain_Q14: libc::c_int = 0;
    let mut temp_LTP_Gain_Q14: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut psPLC: *mut SKP_Silk_PLC_struct = 0 as *mut SKP_Silk_PLC_struct;
    psPLC = &mut (*psDec).sPLC;
    (*psDec).prev_sigtype = (*psDecCtrl).sig_type;
    LTP_Gain_Q14 = 0 as libc::c_int;
    if (*psDecCtrl).sig_type == 0 as libc::c_int {
        j = 0 as libc::c_int;
        while j * (*psDec).subfr_length
            < (*psDecCtrl).pitchL[(4 as libc::c_int - 1 as libc::c_int) as usize]
        {
            temp_LTP_Gain_Q14 = 0 as libc::c_int;
            i = 0 as libc::c_int;
            while i < 5 as libc::c_int {
                temp_LTP_Gain_Q14
                    += (*psDecCtrl)
                        .LTPCoef_Q14[((4 as libc::c_int - 1 as libc::c_int - j)
                        * 5 as libc::c_int + i) as usize] as libc::c_int;
                i += 1;
            }
            if temp_LTP_Gain_Q14 > LTP_Gain_Q14 {
                LTP_Gain_Q14 = temp_LTP_Gain_Q14;
                memcpy(
                    ((*psPLC).LTPCoef_Q14).as_mut_ptr() as *mut libc::c_void,
                    &mut *((*psDecCtrl).LTPCoef_Q14)
                        .as_mut_ptr()
                        .offset(
                            ((4 as libc::c_int - 1 as libc::c_int - j) as libc::c_short
                                as libc::c_int
                                * 5 as libc::c_int as libc::c_short as libc::c_int) as isize,
                        ) as *mut libc::c_short as *const libc::c_void,
                    (5 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(
                            ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                        ),
                );
                (*psPLC)
                    .pitchL_Q8 = (*psDecCtrl)
                    .pitchL[(4 as libc::c_int - 1 as libc::c_int - j) as usize]
                    << 8 as libc::c_int;
            }
            j += 1;
        }
        memset(
            ((*psPLC).LTPCoef_Q14).as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            (5 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_short>() as libc::c_ulong),
        );
        (*psPLC)
            .LTPCoef_Q14[(5 as libc::c_int / 2 as libc::c_int)
            as usize] = LTP_Gain_Q14 as libc::c_short;
        if LTP_Gain_Q14 < 11469 as libc::c_int {
            let mut scale_Q10: libc::c_int = 0;
            let mut tmp: libc::c_int = 0;
            tmp = (11469 as libc::c_int) << 10 as libc::c_int;
            scale_Q10 = tmp
                / (if LTP_Gain_Q14 > 1 as libc::c_int {
                    LTP_Gain_Q14
                } else {
                    1 as libc::c_int
                });
            i = 0 as libc::c_int;
            while i < 5 as libc::c_int {
                (*psPLC)
                    .LTPCoef_Q14[i
                    as usize] = ((*psPLC).LTPCoef_Q14[i as usize] as libc::c_int
                    * scale_Q10 as libc::c_short as libc::c_int >> 10 as libc::c_int)
                    as libc::c_short;
                i += 1;
            }
        } else if LTP_Gain_Q14 > 15565 as libc::c_int {
            let mut scale_Q14: libc::c_int = 0;
            let mut tmp_0: libc::c_int = 0;
            tmp_0 = (15565 as libc::c_int) << 14 as libc::c_int;
            scale_Q14 = tmp_0
                / (if LTP_Gain_Q14 > 1 as libc::c_int {
                    LTP_Gain_Q14
                } else {
                    1 as libc::c_int
                });
            i = 0 as libc::c_int;
            while i < 5 as libc::c_int {
                (*psPLC)
                    .LTPCoef_Q14[i
                    as usize] = ((*psPLC).LTPCoef_Q14[i as usize] as libc::c_int
                    * scale_Q14 as libc::c_short as libc::c_int >> 14 as libc::c_int)
                    as libc::c_short;
                i += 1;
            }
        }
    } else {
        (*psPLC)
            .pitchL_Q8 = ((*psDec).fs_kHz as libc::c_short as libc::c_int
            * 18 as libc::c_int as libc::c_short as libc::c_int) << 8 as libc::c_int;
        memset(
            ((*psPLC).LTPCoef_Q14).as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            (5 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_short>() as libc::c_ulong),
        );
    }
    memcpy(
        ((*psPLC).prevLPC_Q12).as_mut_ptr() as *mut libc::c_void,
        ((*psDecCtrl).PredCoef_Q12[1 as libc::c_int as usize]).as_mut_ptr()
            as *const libc::c_void,
        ((*psDec).LPC_order as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as libc::c_ulong),
    );
    (*psPLC).prevLTP_scale_Q14 = (*psDecCtrl).LTP_scale_Q14 as libc::c_short;
    memcpy(
        ((*psPLC).prevGain_Q16).as_mut_ptr() as *mut libc::c_void,
        ((*psDecCtrl).Gains_Q16).as_mut_ptr() as *const libc::c_void,
        (4 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
}
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_PLC_conceal(
    mut psDec: *mut SKP_Silk_decoder_state,
    mut psDecCtrl: *mut SKP_Silk_decoder_control,
    mut signal: *mut libc::c_short,
    mut _length: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut B_Q14: *mut libc::c_short = 0 as *mut libc::c_short;
    let mut exc_buf: [libc::c_short; 480] = [0; 480];
    let mut exc_buf_ptr: *mut libc::c_short = 0 as *mut libc::c_short;
    let mut rand_scale_Q14: libc::c_short = 0;
    let mut A_Q12_tmp: C2RustUnnamed = C2RustUnnamed { as_int16: [0; 16] };
    let mut rand_seed: libc::c_int = 0;
    let mut harm_Gain_Q15: libc::c_int = 0;
    let mut rand_Gain_Q15: libc::c_int = 0;
    let mut lag: libc::c_int = 0;
    let mut idx: libc::c_int = 0;
    let mut sLTP_buf_idx: libc::c_int = 0;
    let mut shift1: libc::c_int = 0;
    let mut shift2: libc::c_int = 0;
    let mut energy1: libc::c_int = 0;
    let mut energy2: libc::c_int = 0;
    let mut rand_ptr: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut pred_lag_ptr: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut sig_Q10: [libc::c_int; 480] = [0; 480];
    let mut sig_Q10_ptr: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut LPC_exc_Q10: libc::c_int = 0;
    let mut LPC_pred_Q10: libc::c_int = 0;
    let mut LTP_pred_Q14: libc::c_int = 0;
    let mut psPLC: *mut SKP_Silk_PLC_struct = 0 as *mut SKP_Silk_PLC_struct;
    let mut Atmp: libc::c_int = 0;
    psPLC = &mut (*psDec).sPLC;
    memcpy(
        ((*psDec).sLTP_Q16).as_mut_ptr() as *mut libc::c_void,
        &mut *((*psDec).sLTP_Q16).as_mut_ptr().offset((*psDec).frame_length as isize)
            as *mut libc::c_int as *const libc::c_void,
        ((*psDec).frame_length as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    skp_silk_bwexpander(
        &mut ((*psPLC).prevLPC_Q12),
        (*psDec).LPC_order as usize,
        64880 as libc::c_int,
    );
    exc_buf_ptr = exc_buf.as_mut_ptr();
    k = 4 as libc::c_int >> 1 as libc::c_int;
    while k < 4 as libc::c_int {
        i = 0 as libc::c_int;
        while i < (*psDec).subfr_length {
            *exc_buf_ptr
                .offset(
                    i as isize,
                ) = (((*psDec).exc_Q10[(i + k * (*psDec).subfr_length) as usize]
                >> 16 as libc::c_int)
                * (*psPLC).prevGain_Q16[k as usize] as libc::c_short as libc::c_int
                + (((*psDec).exc_Q10[(i + k * (*psDec).subfr_length) as usize]
                    & 0xffff as libc::c_int)
                    * (*psPLC).prevGain_Q16[k as usize] as libc::c_short as libc::c_int
                    >> 16 as libc::c_int)
                + (*psDec).exc_Q10[(i + k * (*psDec).subfr_length) as usize]
                    * (if 16 as libc::c_int == 1 as libc::c_int {
                        ((*psPLC).prevGain_Q16[k as usize] >> 1 as libc::c_int)
                            + ((*psPLC).prevGain_Q16[k as usize] & 1 as libc::c_int)
                    } else {
                        ((*psPLC).prevGain_Q16[k as usize]
                            >> 16 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                            >> 1 as libc::c_int
                    }) >> 10 as libc::c_int) as libc::c_short;
            i += 1;
        }
        exc_buf_ptr = exc_buf_ptr.offset((*psDec).subfr_length as isize);
        k += 1;
    }
    SKP_Silk_sum_sqr_shift(
        &mut energy1,
        &mut shift1,
        exc_buf.as_mut_ptr(),
        (*psDec).subfr_length,
    );
    SKP_Silk_sum_sqr_shift(
        &mut energy2,
        &mut shift2,
        &mut *exc_buf.as_mut_ptr().offset((*psDec).subfr_length as isize),
        (*psDec).subfr_length,
    );
    if energy1 >> shift2 < energy2 >> shift1 {
        rand_ptr = &mut *((*psDec).exc_Q10)
            .as_mut_ptr()
            .offset(
                (SKP_max_int
                    as unsafe extern "C" fn(
                        libc::c_int,
                        libc::c_int,
                    ) -> libc::c_int)(
                    0 as libc::c_int,
                    3 as libc::c_int * (*psDec).subfr_length - 128 as libc::c_int,
                ) as isize,
            ) as *mut libc::c_int;
    } else {
        rand_ptr = &mut *((*psDec).exc_Q10)
            .as_mut_ptr()
            .offset(
                (SKP_max_int
                    as unsafe extern "C" fn(
                        libc::c_int,
                        libc::c_int,
                    ) -> libc::c_int)(
                    0 as libc::c_int,
                    (*psDec).frame_length - 128 as libc::c_int,
                ) as isize,
            ) as *mut libc::c_int;
    }
    B_Q14 = ((*psPLC).LTPCoef_Q14).as_mut_ptr();
    rand_scale_Q14 = (*psPLC).randScale_Q14;
    harm_Gain_Q15 = HARM_ATT_Q15[SKP_min_int(
        2 as libc::c_int - 1 as libc::c_int,
        (*psDec).lossCnt,
    ) as usize] as libc::c_int;
    if (*psDec).prev_sigtype == 0 as libc::c_int {
        rand_Gain_Q15 = PLC_RAND_ATTENUATE_V_Q15[SKP_min_int(
            2 as libc::c_int - 1 as libc::c_int,
            (*psDec).lossCnt,
        ) as usize] as libc::c_int;
    } else {
        rand_Gain_Q15 = PLC_RAND_ATTENUATE_UV_Q15[SKP_min_int(
            2 as libc::c_int - 1 as libc::c_int,
            (*psDec).lossCnt,
        ) as usize] as libc::c_int;
    }
    if (*psDec).lossCnt == 0 as libc::c_int {
        rand_scale_Q14 = ((1 as libc::c_int) << 14 as libc::c_int) as libc::c_short;
        if (*psDec).prev_sigtype == 0 as libc::c_int {
            i = 0 as libc::c_int;
            while i < 5 as libc::c_int {
                rand_scale_Q14 = (rand_scale_Q14 as libc::c_int
                    - *B_Q14.offset(i as isize) as libc::c_int) as libc::c_short;
                i += 1;
            }
            rand_scale_Q14 = SKP_max_16(
                3277 as libc::c_int as libc::c_short,
                rand_scale_Q14,
            );
            rand_scale_Q14 = (rand_scale_Q14 as libc::c_int
                * (*psPLC).prevLTP_scale_Q14 as libc::c_int >> 14 as libc::c_int)
                as libc::c_short;
        }
        if (*psDec).prev_sigtype == 1 as libc::c_int {
            let mut invGain_Q30: libc::c_int = 0;
            let mut down_scale_Q30: libc::c_int = 0;
            skp_silk_lpc_inverse_pred_gain(
                &mut invGain_Q30,
                &mut ((*psPLC).prevLPC_Q12),
                (*psDec).LPC_order as usize,
            );
            down_scale_Q30 = SKP_min_32(
                (1 as libc::c_int) << 30 as libc::c_int >> 3 as libc::c_int,
                invGain_Q30,
            );
            down_scale_Q30 = SKP_max_32(
                (1 as libc::c_int) << 30 as libc::c_int >> 8 as libc::c_int,
                down_scale_Q30,
            );
            down_scale_Q30 = down_scale_Q30 << 3 as libc::c_int;
            rand_Gain_Q15 = (down_scale_Q30 >> 16 as libc::c_int)
                * rand_Gain_Q15 as libc::c_short as libc::c_int
                + ((down_scale_Q30 & 0xffff as libc::c_int)
                    * rand_Gain_Q15 as libc::c_short as libc::c_int >> 16 as libc::c_int)
                >> 14 as libc::c_int;
        }
    }
    rand_seed = (*psPLC).rand_seed;
    lag = if 8 as libc::c_int == 1 as libc::c_int {
        ((*psPLC).pitchL_Q8 >> 1 as libc::c_int)
            + ((*psPLC).pitchL_Q8 & 1 as libc::c_int)
    } else {
        ((*psPLC).pitchL_Q8 >> 8 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
            >> 1 as libc::c_int
    };
    sLTP_buf_idx = (*psDec).frame_length;
    sig_Q10_ptr = sig_Q10.as_mut_ptr();
    k = 0 as libc::c_int;
    while k < 4 as libc::c_int {
        pred_lag_ptr = &mut *((*psDec).sLTP_Q16)
            .as_mut_ptr()
            .offset((sLTP_buf_idx - lag + 5 as libc::c_int / 2 as libc::c_int) as isize)
            as *mut libc::c_int;
        i = 0 as libc::c_int;
        while i < (*psDec).subfr_length {
            rand_seed = (907633515 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    (rand_seed as libc::c_uint)
                        .wrapping_mul(196314165 as libc::c_int as libc::c_uint),
                ) as libc::c_int;
            idx = rand_seed >> 25 as libc::c_int & 128 as libc::c_int - 1 as libc::c_int;
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
            LPC_exc_Q10 = (*rand_ptr.offset(idx as isize) >> 16 as libc::c_int)
                * rand_scale_Q14 as libc::c_int
                + ((*rand_ptr.offset(idx as isize) & 0xffff as libc::c_int)
                    * rand_scale_Q14 as libc::c_int >> 16 as libc::c_int)
                << 2 as libc::c_int;
            LPC_exc_Q10 = LPC_exc_Q10
                + (if 4 as libc::c_int == 1 as libc::c_int {
                    (LTP_pred_Q14 >> 1 as libc::c_int)
                        + (LTP_pred_Q14 & 1 as libc::c_int)
                } else {
                    (LTP_pred_Q14 >> 4 as libc::c_int - 1 as libc::c_int)
                        + 1 as libc::c_int >> 1 as libc::c_int
                });
            (*psDec).sLTP_Q16[sLTP_buf_idx as usize] = LPC_exc_Q10 << 6 as libc::c_int;
            sLTP_buf_idx += 1;
            *sig_Q10_ptr.offset(i as isize) = LPC_exc_Q10;
            i += 1;
        }
        sig_Q10_ptr = sig_Q10_ptr.offset((*psDec).subfr_length as isize);
        j = 0 as libc::c_int;
        while j < 5 as libc::c_int {
            *B_Q14
                .offset(
                    j as isize,
                ) = (harm_Gain_Q15 as libc::c_short as libc::c_int
                * *B_Q14.offset(j as isize) as libc::c_int >> 15 as libc::c_int)
                as libc::c_short;
            j += 1;
        }
        rand_scale_Q14 = (rand_scale_Q14 as libc::c_int
            * rand_Gain_Q15 as libc::c_short as libc::c_int >> 15 as libc::c_int)
            as libc::c_short;
        (*psPLC).pitchL_Q8
            += ((*psPLC).pitchL_Q8 >> 16 as libc::c_int)
                * 655 as libc::c_int as libc::c_short as libc::c_int
                + (((*psPLC).pitchL_Q8 & 0xffff as libc::c_int)
                    * 655 as libc::c_int as libc::c_short as libc::c_int
                    >> 16 as libc::c_int);
        (*psPLC)
            .pitchL_Q8 = SKP_min_32(
            (*psPLC).pitchL_Q8,
            (18 as libc::c_int as libc::c_short as libc::c_int
                * (*psDec).fs_kHz as libc::c_short as libc::c_int) << 8 as libc::c_int,
        );
        lag = if 8 as libc::c_int == 1 as libc::c_int {
            ((*psPLC).pitchL_Q8 >> 1 as libc::c_int)
                + ((*psPLC).pitchL_Q8 & 1 as libc::c_int)
        } else {
            ((*psPLC).pitchL_Q8 >> 8 as libc::c_int - 1 as libc::c_int)
                + 1 as libc::c_int >> 1 as libc::c_int
        };
        k += 1;
    }
    sig_Q10_ptr = sig_Q10.as_mut_ptr();
    memcpy(
        (A_Q12_tmp.as_int16).as_mut_ptr() as *mut libc::c_void,
        ((*psPLC).prevLPC_Q12).as_mut_ptr() as *const libc::c_void,
        ((*psDec).LPC_order as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as libc::c_ulong),
    );
    k = 0 as libc::c_int;
    while k < 4 as libc::c_int {
        i = 0 as libc::c_int;
        while i < (*psDec).subfr_length {
            Atmp = A_Q12_tmp.as_int32[0 as libc::c_int as usize];
            LPC_pred_Q10 = ((*psDec)
                .sLPC_Q14[(16 as libc::c_int + i - 1 as libc::c_int) as usize]
                >> 16 as libc::c_int) * Atmp as libc::c_short as libc::c_int
                + (((*psDec)
                    .sLPC_Q14[(16 as libc::c_int + i - 1 as libc::c_int) as usize]
                    & 0xffff as libc::c_int) * Atmp as libc::c_short as libc::c_int
                    >> 16 as libc::c_int);
            LPC_pred_Q10 = LPC_pred_Q10
                + ((*psDec).sLPC_Q14[(16 as libc::c_int + i - 2 as libc::c_int) as usize]
                    >> 16 as libc::c_int) * (Atmp >> 16 as libc::c_int)
                + (((*psDec)
                    .sLPC_Q14[(16 as libc::c_int + i - 2 as libc::c_int) as usize]
                    & 0xffff as libc::c_int) * (Atmp >> 16 as libc::c_int)
                    >> 16 as libc::c_int);
            Atmp = A_Q12_tmp.as_int32[1 as libc::c_int as usize];
            LPC_pred_Q10 = LPC_pred_Q10
                + (((*psDec)
                    .sLPC_Q14[(16 as libc::c_int + i - 3 as libc::c_int) as usize]
                    >> 16 as libc::c_int) * Atmp as libc::c_short as libc::c_int
                    + (((*psDec)
                        .sLPC_Q14[(16 as libc::c_int + i - 3 as libc::c_int) as usize]
                        & 0xffff as libc::c_int) * Atmp as libc::c_short as libc::c_int
                        >> 16 as libc::c_int));
            LPC_pred_Q10 = LPC_pred_Q10
                + ((*psDec).sLPC_Q14[(16 as libc::c_int + i - 4 as libc::c_int) as usize]
                    >> 16 as libc::c_int) * (Atmp >> 16 as libc::c_int)
                + (((*psDec)
                    .sLPC_Q14[(16 as libc::c_int + i - 4 as libc::c_int) as usize]
                    & 0xffff as libc::c_int) * (Atmp >> 16 as libc::c_int)
                    >> 16 as libc::c_int);
            Atmp = A_Q12_tmp.as_int32[2 as libc::c_int as usize];
            LPC_pred_Q10 = LPC_pred_Q10
                + (((*psDec)
                    .sLPC_Q14[(16 as libc::c_int + i - 5 as libc::c_int) as usize]
                    >> 16 as libc::c_int) * Atmp as libc::c_short as libc::c_int
                    + (((*psDec)
                        .sLPC_Q14[(16 as libc::c_int + i - 5 as libc::c_int) as usize]
                        & 0xffff as libc::c_int) * Atmp as libc::c_short as libc::c_int
                        >> 16 as libc::c_int));
            LPC_pred_Q10 = LPC_pred_Q10
                + ((*psDec).sLPC_Q14[(16 as libc::c_int + i - 6 as libc::c_int) as usize]
                    >> 16 as libc::c_int) * (Atmp >> 16 as libc::c_int)
                + (((*psDec)
                    .sLPC_Q14[(16 as libc::c_int + i - 6 as libc::c_int) as usize]
                    & 0xffff as libc::c_int) * (Atmp >> 16 as libc::c_int)
                    >> 16 as libc::c_int);
            Atmp = A_Q12_tmp.as_int32[3 as libc::c_int as usize];
            LPC_pred_Q10 = LPC_pred_Q10
                + (((*psDec)
                    .sLPC_Q14[(16 as libc::c_int + i - 7 as libc::c_int) as usize]
                    >> 16 as libc::c_int) * Atmp as libc::c_short as libc::c_int
                    + (((*psDec)
                        .sLPC_Q14[(16 as libc::c_int + i - 7 as libc::c_int) as usize]
                        & 0xffff as libc::c_int) * Atmp as libc::c_short as libc::c_int
                        >> 16 as libc::c_int));
            LPC_pred_Q10 = LPC_pred_Q10
                + ((*psDec).sLPC_Q14[(16 as libc::c_int + i - 8 as libc::c_int) as usize]
                    >> 16 as libc::c_int) * (Atmp >> 16 as libc::c_int)
                + (((*psDec)
                    .sLPC_Q14[(16 as libc::c_int + i - 8 as libc::c_int) as usize]
                    & 0xffff as libc::c_int) * (Atmp >> 16 as libc::c_int)
                    >> 16 as libc::c_int);
            Atmp = A_Q12_tmp.as_int32[4 as libc::c_int as usize];
            LPC_pred_Q10 = LPC_pred_Q10
                + (((*psDec)
                    .sLPC_Q14[(16 as libc::c_int + i - 9 as libc::c_int) as usize]
                    >> 16 as libc::c_int) * Atmp as libc::c_short as libc::c_int
                    + (((*psDec)
                        .sLPC_Q14[(16 as libc::c_int + i - 9 as libc::c_int) as usize]
                        & 0xffff as libc::c_int) * Atmp as libc::c_short as libc::c_int
                        >> 16 as libc::c_int));
            LPC_pred_Q10 = LPC_pred_Q10
                + ((*psDec)
                    .sLPC_Q14[(16 as libc::c_int + i - 10 as libc::c_int) as usize]
                    >> 16 as libc::c_int) * (Atmp >> 16 as libc::c_int)
                + (((*psDec)
                    .sLPC_Q14[(16 as libc::c_int + i - 10 as libc::c_int) as usize]
                    & 0xffff as libc::c_int) * (Atmp >> 16 as libc::c_int)
                    >> 16 as libc::c_int);
            j = 10 as libc::c_int;
            while j < (*psDec).LPC_order {
                Atmp = A_Q12_tmp.as_int32[(j / 2 as libc::c_int) as usize];
                LPC_pred_Q10 = LPC_pred_Q10
                    + (((*psDec)
                        .sLPC_Q14[(16 as libc::c_int + i - 1 as libc::c_int - j)
                        as usize] >> 16 as libc::c_int)
                        * Atmp as libc::c_short as libc::c_int
                        + (((*psDec)
                            .sLPC_Q14[(16 as libc::c_int + i - 1 as libc::c_int - j)
                            as usize] & 0xffff as libc::c_int)
                            * Atmp as libc::c_short as libc::c_int
                            >> 16 as libc::c_int));
                LPC_pred_Q10 = LPC_pred_Q10
                    + ((*psDec)
                        .sLPC_Q14[(16 as libc::c_int + i - 2 as libc::c_int - j)
                        as usize] >> 16 as libc::c_int) * (Atmp >> 16 as libc::c_int)
                    + (((*psDec)
                        .sLPC_Q14[(16 as libc::c_int + i - 2 as libc::c_int - j)
                        as usize] & 0xffff as libc::c_int) * (Atmp >> 16 as libc::c_int)
                        >> 16 as libc::c_int);
                j += 2 as libc::c_int;
            }
            *sig_Q10_ptr
                .offset(i as isize) = *sig_Q10_ptr.offset(i as isize) + LPC_pred_Q10;
            (*psDec)
                .sLPC_Q14[(16 as libc::c_int + i)
                as usize] = *sig_Q10_ptr.offset(i as isize) << 4 as libc::c_int;
            i += 1;
        }
        sig_Q10_ptr = sig_Q10_ptr.offset((*psDec).subfr_length as isize);
        memcpy(
            ((*psDec).sLPC_Q14).as_mut_ptr() as *mut libc::c_void,
            &mut *((*psDec).sLPC_Q14).as_mut_ptr().offset((*psDec).subfr_length as isize)
                as *mut libc::c_int as *const libc::c_void,
            (16 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
        );
        k += 1;
    }
    i = 0 as libc::c_int;
    while i < (*psDec).frame_length {
        *signal
            .offset(
                i as isize,
            ) = (if (if 10 as libc::c_int == 1 as libc::c_int {
            ((sig_Q10[i as usize] >> 16 as libc::c_int)
                * (*psPLC).prevGain_Q16[(4 as libc::c_int - 1 as libc::c_int) as usize]
                    as libc::c_short as libc::c_int
                + ((sig_Q10[i as usize] & 0xffff as libc::c_int)
                    * (*psPLC)
                        .prevGain_Q16[(4 as libc::c_int - 1 as libc::c_int) as usize]
                        as libc::c_short as libc::c_int >> 16 as libc::c_int)
                + sig_Q10[i as usize]
                    * (if 16 as libc::c_int == 1 as libc::c_int {
                        ((*psPLC)
                            .prevGain_Q16[(4 as libc::c_int - 1 as libc::c_int) as usize]
                            >> 1 as libc::c_int)
                            + ((*psPLC)
                                .prevGain_Q16[(4 as libc::c_int - 1 as libc::c_int)
                                as usize] & 1 as libc::c_int)
                    } else {
                        ((*psPLC)
                            .prevGain_Q16[(4 as libc::c_int - 1 as libc::c_int) as usize]
                            >> 16 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                            >> 1 as libc::c_int
                    }) >> 1 as libc::c_int)
                + ((sig_Q10[i as usize] >> 16 as libc::c_int)
                    * (*psPLC)
                        .prevGain_Q16[(4 as libc::c_int - 1 as libc::c_int) as usize]
                        as libc::c_short as libc::c_int
                    + ((sig_Q10[i as usize] & 0xffff as libc::c_int)
                        * (*psPLC)
                            .prevGain_Q16[(4 as libc::c_int - 1 as libc::c_int) as usize]
                            as libc::c_short as libc::c_int >> 16 as libc::c_int)
                    + sig_Q10[i as usize]
                        * (if 16 as libc::c_int == 1 as libc::c_int {
                            ((*psPLC)
                                .prevGain_Q16[(4 as libc::c_int - 1 as libc::c_int)
                                as usize] >> 1 as libc::c_int)
                                + ((*psPLC)
                                    .prevGain_Q16[(4 as libc::c_int - 1 as libc::c_int)
                                    as usize] & 1 as libc::c_int)
                        } else {
                            ((*psPLC)
                                .prevGain_Q16[(4 as libc::c_int - 1 as libc::c_int)
                                as usize] >> 16 as libc::c_int - 1 as libc::c_int)
                                + 1 as libc::c_int >> 1 as libc::c_int
                        }) & 1 as libc::c_int)
        } else {
            ((sig_Q10[i as usize] >> 16 as libc::c_int)
                * (*psPLC).prevGain_Q16[(4 as libc::c_int - 1 as libc::c_int) as usize]
                    as libc::c_short as libc::c_int
                + ((sig_Q10[i as usize] & 0xffff as libc::c_int)
                    * (*psPLC)
                        .prevGain_Q16[(4 as libc::c_int - 1 as libc::c_int) as usize]
                        as libc::c_short as libc::c_int >> 16 as libc::c_int)
                + sig_Q10[i as usize]
                    * (if 16 as libc::c_int == 1 as libc::c_int {
                        ((*psPLC)
                            .prevGain_Q16[(4 as libc::c_int - 1 as libc::c_int) as usize]
                            >> 1 as libc::c_int)
                            + ((*psPLC)
                                .prevGain_Q16[(4 as libc::c_int - 1 as libc::c_int)
                                as usize] & 1 as libc::c_int)
                    } else {
                        ((*psPLC)
                            .prevGain_Q16[(4 as libc::c_int - 1 as libc::c_int) as usize]
                            >> 16 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                            >> 1 as libc::c_int
                    }) >> 10 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                >> 1 as libc::c_int
        }) > 0x7fff as libc::c_int
        {
            0x7fff as libc::c_int
        } else if (if 10 as libc::c_int == 1 as libc::c_int {
            ((sig_Q10[i as usize] >> 16 as libc::c_int)
                * (*psPLC).prevGain_Q16[(4 as libc::c_int - 1 as libc::c_int) as usize]
                    as libc::c_short as libc::c_int
                + ((sig_Q10[i as usize] & 0xffff as libc::c_int)
                    * (*psPLC)
                        .prevGain_Q16[(4 as libc::c_int - 1 as libc::c_int) as usize]
                        as libc::c_short as libc::c_int >> 16 as libc::c_int)
                + sig_Q10[i as usize]
                    * (if 16 as libc::c_int == 1 as libc::c_int {
                        ((*psPLC)
                            .prevGain_Q16[(4 as libc::c_int - 1 as libc::c_int) as usize]
                            >> 1 as libc::c_int)
                            + ((*psPLC)
                                .prevGain_Q16[(4 as libc::c_int - 1 as libc::c_int)
                                as usize] & 1 as libc::c_int)
                    } else {
                        ((*psPLC)
                            .prevGain_Q16[(4 as libc::c_int - 1 as libc::c_int) as usize]
                            >> 16 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                            >> 1 as libc::c_int
                    }) >> 1 as libc::c_int)
                + ((sig_Q10[i as usize] >> 16 as libc::c_int)
                    * (*psPLC)
                        .prevGain_Q16[(4 as libc::c_int - 1 as libc::c_int) as usize]
                        as libc::c_short as libc::c_int
                    + ((sig_Q10[i as usize] & 0xffff as libc::c_int)
                        * (*psPLC)
                            .prevGain_Q16[(4 as libc::c_int - 1 as libc::c_int) as usize]
                            as libc::c_short as libc::c_int >> 16 as libc::c_int)
                    + sig_Q10[i as usize]
                        * (if 16 as libc::c_int == 1 as libc::c_int {
                            ((*psPLC)
                                .prevGain_Q16[(4 as libc::c_int - 1 as libc::c_int)
                                as usize] >> 1 as libc::c_int)
                                + ((*psPLC)
                                    .prevGain_Q16[(4 as libc::c_int - 1 as libc::c_int)
                                    as usize] & 1 as libc::c_int)
                        } else {
                            ((*psPLC)
                                .prevGain_Q16[(4 as libc::c_int - 1 as libc::c_int)
                                as usize] >> 16 as libc::c_int - 1 as libc::c_int)
                                + 1 as libc::c_int >> 1 as libc::c_int
                        }) & 1 as libc::c_int)
        } else {
            ((sig_Q10[i as usize] >> 16 as libc::c_int)
                * (*psPLC).prevGain_Q16[(4 as libc::c_int - 1 as libc::c_int) as usize]
                    as libc::c_short as libc::c_int
                + ((sig_Q10[i as usize] & 0xffff as libc::c_int)
                    * (*psPLC)
                        .prevGain_Q16[(4 as libc::c_int - 1 as libc::c_int) as usize]
                        as libc::c_short as libc::c_int >> 16 as libc::c_int)
                + sig_Q10[i as usize]
                    * (if 16 as libc::c_int == 1 as libc::c_int {
                        ((*psPLC)
                            .prevGain_Q16[(4 as libc::c_int - 1 as libc::c_int) as usize]
                            >> 1 as libc::c_int)
                            + ((*psPLC)
                                .prevGain_Q16[(4 as libc::c_int - 1 as libc::c_int)
                                as usize] & 1 as libc::c_int)
                    } else {
                        ((*psPLC)
                            .prevGain_Q16[(4 as libc::c_int - 1 as libc::c_int) as usize]
                            >> 16 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                            >> 1 as libc::c_int
                    }) >> 10 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                >> 1 as libc::c_int
        }) < 0x8000 as libc::c_int as libc::c_short as libc::c_int
        {
            0x8000 as libc::c_int as libc::c_short as libc::c_int
        } else if 10 as libc::c_int == 1 as libc::c_int {
            ((sig_Q10[i as usize] >> 16 as libc::c_int)
                * (*psPLC).prevGain_Q16[(4 as libc::c_int - 1 as libc::c_int) as usize]
                    as libc::c_short as libc::c_int
                + ((sig_Q10[i as usize] & 0xffff as libc::c_int)
                    * (*psPLC)
                        .prevGain_Q16[(4 as libc::c_int - 1 as libc::c_int) as usize]
                        as libc::c_short as libc::c_int >> 16 as libc::c_int)
                + sig_Q10[i as usize]
                    * (if 16 as libc::c_int == 1 as libc::c_int {
                        ((*psPLC)
                            .prevGain_Q16[(4 as libc::c_int - 1 as libc::c_int) as usize]
                            >> 1 as libc::c_int)
                            + ((*psPLC)
                                .prevGain_Q16[(4 as libc::c_int - 1 as libc::c_int)
                                as usize] & 1 as libc::c_int)
                    } else {
                        ((*psPLC)
                            .prevGain_Q16[(4 as libc::c_int - 1 as libc::c_int) as usize]
                            >> 16 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                            >> 1 as libc::c_int
                    }) >> 1 as libc::c_int)
                + ((sig_Q10[i as usize] >> 16 as libc::c_int)
                    * (*psPLC)
                        .prevGain_Q16[(4 as libc::c_int - 1 as libc::c_int) as usize]
                        as libc::c_short as libc::c_int
                    + ((sig_Q10[i as usize] & 0xffff as libc::c_int)
                        * (*psPLC)
                            .prevGain_Q16[(4 as libc::c_int - 1 as libc::c_int) as usize]
                            as libc::c_short as libc::c_int >> 16 as libc::c_int)
                    + sig_Q10[i as usize]
                        * (if 16 as libc::c_int == 1 as libc::c_int {
                            ((*psPLC)
                                .prevGain_Q16[(4 as libc::c_int - 1 as libc::c_int)
                                as usize] >> 1 as libc::c_int)
                                + ((*psPLC)
                                    .prevGain_Q16[(4 as libc::c_int - 1 as libc::c_int)
                                    as usize] & 1 as libc::c_int)
                        } else {
                            ((*psPLC)
                                .prevGain_Q16[(4 as libc::c_int - 1 as libc::c_int)
                                as usize] >> 16 as libc::c_int - 1 as libc::c_int)
                                + 1 as libc::c_int >> 1 as libc::c_int
                        }) & 1 as libc::c_int)
        } else {
            ((sig_Q10[i as usize] >> 16 as libc::c_int)
                * (*psPLC).prevGain_Q16[(4 as libc::c_int - 1 as libc::c_int) as usize]
                    as libc::c_short as libc::c_int
                + ((sig_Q10[i as usize] & 0xffff as libc::c_int)
                    * (*psPLC)
                        .prevGain_Q16[(4 as libc::c_int - 1 as libc::c_int) as usize]
                        as libc::c_short as libc::c_int >> 16 as libc::c_int)
                + sig_Q10[i as usize]
                    * (if 16 as libc::c_int == 1 as libc::c_int {
                        ((*psPLC)
                            .prevGain_Q16[(4 as libc::c_int - 1 as libc::c_int) as usize]
                            >> 1 as libc::c_int)
                            + ((*psPLC)
                                .prevGain_Q16[(4 as libc::c_int - 1 as libc::c_int)
                                as usize] & 1 as libc::c_int)
                    } else {
                        ((*psPLC)
                            .prevGain_Q16[(4 as libc::c_int - 1 as libc::c_int) as usize]
                            >> 16 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                            >> 1 as libc::c_int
                    }) >> 10 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                >> 1 as libc::c_int
        }) as libc::c_short;
        i += 1;
    }
    (*psPLC).rand_seed = rand_seed;
    (*psPLC).randScale_Q14 = rand_scale_Q14;
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        (*psDecCtrl).pitchL[i as usize] = lag;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_PLC_glue_frames(
    mut psDec: *mut SKP_Silk_decoder_state,
    mut _psDecCtrl: *mut SKP_Silk_decoder_control,
    mut signal: *mut libc::c_short,
    mut length: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut energy_shift: libc::c_int = 0;
    let mut energy: libc::c_int = 0;
    let mut psPLC: *mut SKP_Silk_PLC_struct = 0 as *mut SKP_Silk_PLC_struct;
    psPLC = &mut (*psDec).sPLC;
    if (*psDec).lossCnt != 0 {
        SKP_Silk_sum_sqr_shift(
            &mut (*psPLC).conc_energy,
            &mut (*psPLC).conc_energy_shift,
            signal as *const libc::c_short,
            length,
        );
        (*psPLC).last_frame_lost = 1 as libc::c_int;
    } else {
        if (*psDec).sPLC.last_frame_lost != 0 {
            SKP_Silk_sum_sqr_shift(
                &mut energy,
                &mut energy_shift,
                signal as *const libc::c_short,
                length,
            );
            if energy_shift > (*psPLC).conc_energy_shift {
                (*psPLC)
                    .conc_energy = (*psPLC).conc_energy
                    >> energy_shift - (*psPLC).conc_energy_shift;
            } else if energy_shift < (*psPLC).conc_energy_shift {
                energy = energy >> (*psPLC).conc_energy_shift - energy_shift;
            }
            if energy > (*psPLC).conc_energy {
                let mut frac_Q24: libc::c_int = 0;
                let mut LZ: libc::c_int = 0;
                let mut gain_Q12: libc::c_int = 0;
                let mut slope_Q12: libc::c_int = 0;
                LZ = SKP_Silk_CLZ32((*psPLC).conc_energy);
                LZ = LZ - 1 as libc::c_int;
                (*psPLC).conc_energy = (*psPLC).conc_energy << LZ;
                energy = energy >> SKP_max_32(24 as libc::c_int - LZ, 0 as libc::c_int);
                frac_Q24 = (*psPLC).conc_energy
                    / (if energy > 1 as libc::c_int {
                        energy
                    } else {
                        1 as libc::c_int
                    });
                gain_Q12 = SKP_Silk_SQRT_APPROX(frac_Q24);
                slope_Q12 = (((1 as libc::c_int) << 12 as libc::c_int) - gain_Q12)
                    / length;
                i = 0 as libc::c_int;
                while i < length {
                    *signal
                        .offset(
                            i as isize,
                        ) = (gain_Q12 * *signal.offset(i as isize) as libc::c_int
                        >> 12 as libc::c_int) as libc::c_short;
                    gain_Q12 += slope_Q12;
                    gain_Q12 = if gain_Q12 < (1 as libc::c_int) << 12 as libc::c_int {
                        gain_Q12
                    } else {
                        (1 as libc::c_int) << 12 as libc::c_int
                    };
                    i += 1;
                }
            }
        }
        (*psPLC).last_frame_lost = 0 as libc::c_int;
    };
}
