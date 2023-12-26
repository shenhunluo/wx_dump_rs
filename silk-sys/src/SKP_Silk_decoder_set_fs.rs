#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]

use crate::SKP_Silk_dec_API::SKP_Silk_decoder_state;
use crate::skp_silk_tables_nlsf_cb0_10::SKP_SILK_NLSF_CB0_10;
use crate::skp_silk_tables_nlsf_cb0_16::SKP_SILK_NLSF_CB0_16;
use crate::skp_silk_tables_nlsf_cb1_10::SKP_SILK_NLSF_CB1_10;
use crate::skp_silk_tables_nlsf_cb1_16::SKP_SILK_NLSF_CB1_16;
use crate::skp_silk_tables_other::{SKP_SILK_DEC_A_HP_24, SKP_SILK_DEC_B_HP_24, SKP_SILK_DEC_B_HP_16, SKP_SILK_DEC_A_HP_16, SKP_SILK_DEC_A_HP_12, SKP_SILK_DEC_B_HP_12, SKP_SILK_DEC_A_HP_8, SKP_SILK_DEC_B_HP_8};
extern "C" {
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
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_decoder_set_fs(
    mut psDec: *mut SKP_Silk_decoder_state,
    mut fs_kHz: libc::c_int,
) {
    if (*psDec).fs_kHz != fs_kHz {
        (*psDec).fs_kHz = fs_kHz;
        (*psDec)
            .frame_length = 20 as libc::c_int as libc::c_short as libc::c_int
            * fs_kHz as libc::c_short as libc::c_int;
        (*psDec)
            .subfr_length = (20 as libc::c_int / 4 as libc::c_int) as libc::c_short
            as libc::c_int * fs_kHz as libc::c_short as libc::c_int;
        if (*psDec).fs_kHz == 8 as libc::c_int {
            (*psDec).LPC_order = 10 as libc::c_int;
            (*psDec).psNLSF_CB[0 as libc::c_int as usize] = Some(&SKP_SILK_NLSF_CB0_10);
            (*psDec).psNLSF_CB[1 as libc::c_int as usize] = Some(&SKP_SILK_NLSF_CB1_10);
        } else {
            (*psDec).LPC_order = 16 as libc::c_int;
            (*psDec).psNLSF_CB[0 as libc::c_int as usize] = Some(&SKP_SILK_NLSF_CB0_16);
            (*psDec).psNLSF_CB[1 as libc::c_int as usize] = Some(&SKP_SILK_NLSF_CB1_16);
        }
        memset(
            ((*psDec).sLPC_Q14).as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            (16 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
        );
        memset(
            ((*psDec).outBuf).as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            ((20 as libc::c_int * 24 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_short>() as libc::c_ulong),
        );
        memset(
            ((*psDec).prevNLSF_Q15).as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            (16 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
        );
        (*psDec).lagPrev = 100 as libc::c_int;
        (*psDec).LastGainIndex = 1 as libc::c_int;
        (*psDec).prev_sigtype = 0 as libc::c_int;
        (*psDec).first_frame_after_reset = 1 as libc::c_int;
        if fs_kHz == 24 as libc::c_int {
            (*psDec).HP_A = SKP_SILK_DEC_A_HP_24.as_ptr();
            (*psDec).HP_B = SKP_SILK_DEC_B_HP_24.as_ptr();
        } else if fs_kHz == 16 as libc::c_int {
            (*psDec).HP_A = SKP_SILK_DEC_A_HP_16.as_ptr();
            (*psDec).HP_B = SKP_SILK_DEC_B_HP_16.as_ptr();
        } else if fs_kHz == 12 as libc::c_int {
            (*psDec).HP_A = SKP_SILK_DEC_A_HP_12.as_ptr();
            (*psDec).HP_B = SKP_SILK_DEC_B_HP_12.as_ptr();
        } else if fs_kHz == 8 as libc::c_int {
            (*psDec).HP_A = SKP_SILK_DEC_A_HP_8.as_ptr();
            (*psDec).HP_B = SKP_SILK_DEC_B_HP_8.as_ptr();
        }
    }
}
