#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]

use crate::{SKP_Silk_dec_API::{SKP_Silk_decoder_state, SKP_Silk_decoder_control}, SKP_Silk_range_coder::SKP_Silk_range_dec_init, SKP_Silk_biquad::SKP_Silk_biquad, SKP_Silk_PLC::{SKP_Silk_PLC, SKP_Silk_PLC_glue_frames}, SKP_Silk_decoder_set_fs::SKP_Silk_decoder_set_fs, SKP_Silk_CNG::SKP_Silk_CNG, SKP_Silk_decode_parameters::SKP_Silk_decode_parameters, SKP_Silk_decode_core::SKP_Silk_decode_core};
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
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
#[no_mangle]
pub unsafe fn SKP_Silk_decode_frame(
    mut psDec: &mut SKP_Silk_decoder_state,
    mut pOut: *mut libc::c_short,
    mut pN: *mut libc::c_short,
    mut pCode: &[u8],
    nBytes: libc::c_int,
    mut action: libc::c_int,
    mut decBytes: *mut libc::c_int,
) -> libc::c_int {
    let mut sDecCtrl: SKP_Silk_decoder_control = SKP_Silk_decoder_control {
        pitchL: [0; 4],
        Gains_Q16: [0; 4],
        Seed: 0,
        PredCoef_Q12: [[0; 16]; 2],
        LTPCoef_Q14: [0; 20],
        LTP_scale_Q14: 0,
        PERIndex: 0,
        RateLevelIndex: 0,
        QuantOffsetType: 0,
        sig_type: 0,
        NLSFInterpCoef_Q2: 0,
    };
    let mut L: libc::c_int = 0;
    let mut fs_Khz_old: libc::c_int = 0;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut Pulses: [libc::c_int; 480] = [0; 480];
    L = (*psDec).frame_length;
    sDecCtrl.LTP_scale_Q14 = 0 as libc::c_int;
    *decBytes = 0 as libc::c_int;
    if action == 0 as libc::c_int {
        fs_Khz_old = (*psDec).fs_kHz;
        if (*psDec).nFramesDecoded == 0 as libc::c_int {
            SKP_Silk_range_dec_init(&mut (*psDec).sRC, pCode, nBytes);
        }
        SKP_Silk_decode_parameters(
            psDec,
            &mut sDecCtrl,
            &mut Pulses,
            1 as libc::c_int,
        );
        if (*psDec).sRC.error != 0 {
            (*psDec).nBytesLeft = 0 as libc::c_int;
            action = 1 as libc::c_int;
            SKP_Silk_decoder_set_fs(psDec, fs_Khz_old);
            *decBytes = (*psDec).sRC.bufferLength;
            if (*psDec).sRC.error == -(8 as libc::c_int) {
                ret = -(11 as libc::c_int);
            } else {
                ret = -(12 as libc::c_int);
            }
        } else {
            *decBytes = (*psDec).sRC.bufferLength - (*psDec).nBytesLeft;
            (*psDec).nFramesDecoded += 1;
            (*psDec).nFramesDecoded;
            L = (*psDec).frame_length;
            SKP_Silk_decode_core(
                psDec,
                &mut sDecCtrl,
                pOut,
                Pulses.as_mut_ptr() as *const libc::c_int,
            );
            SKP_Silk_PLC(psDec, &mut sDecCtrl, pOut, L, action);
            (*psDec).lossCnt = 0 as libc::c_int;
            (*psDec).prev_sigtype = sDecCtrl.sig_type;
            (*psDec).first_frame_after_reset = 0 as libc::c_int;
        }
    }
    if action == 1 as libc::c_int {
        SKP_Silk_PLC(psDec, &mut sDecCtrl, pOut, L, action);
    }
    memcpy(
        ((*psDec).outBuf).as_mut_ptr() as *mut libc::c_void,
        pOut as *const libc::c_void,
        (L as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as libc::c_ulong),
    );
    SKP_Silk_PLC_glue_frames(psDec, &mut sDecCtrl, pOut, L);
    SKP_Silk_CNG(psDec, &mut sDecCtrl, pOut, L);
    SKP_Silk_biquad(
        pOut as *const libc::c_short,
        psDec.HP_B.unwrap(),
        psDec.HP_A.unwrap(),
        ((*psDec).HPState).as_mut_ptr(),
        pOut,
        L,
    );
    *pN = L as libc::c_short;
    (*psDec).lagPrev = sDecCtrl.pitchL[(4 as libc::c_int - 1 as libc::c_int) as usize];
    return ret;
}
