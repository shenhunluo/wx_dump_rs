#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]

use crate::{SKP_Silk_dec_API::{SkpSilkDecoderStruct, SKP_Silk_decoder_control}, SKP_Silk_range_coder::SKP_Silk_range_dec_init, skp_silk_biquad::skp_silk_biquad, skp_silk_plc::{skp_silk_plc, skp_silk_plc_glue_frames}, SKP_Silk_decoder_set_fs::SKP_Silk_decoder_set_fs, skp_silk_cng::skp_silk_cng, skp_silk_decode_parameters::skp_silk_decode_parameters, skp_silk_decode_core::skp_silk_decode_core};
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
#[no_mangle]
pub unsafe fn SKP_Silk_decode_frame(
    mut psDec: &mut SkpSilkDecoderStruct,
    mut pOut: &mut [i16],
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
        ltp_coef_q14: [0; 20],
        LTP_scale_Q14: 0,
        PERIndex: 0,
        RateLevelIndex: 0,
        QuantOffsetType: 0,
        sig_type: 0,
        NLSFInterpCoef_Q2: 0,
    };
    let mut L: libc::c_int = 0;
    let mut fs_k_hz_old: libc::c_int = 0;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut Pulses: [libc::c_int; 480] = [0; 480];
    L = (*psDec).frame_length;
    sDecCtrl.LTP_scale_Q14 = 0 as libc::c_int;
    *decBytes = 0 as libc::c_int;
    if action == 0 as libc::c_int {
        fs_k_hz_old = (*psDec).fs_k_hz;
        if (*psDec).nFramesDecoded == 0 as libc::c_int {
            SKP_Silk_range_dec_init(&mut (*psDec).sRC, pCode, nBytes);
        }
        skp_silk_decode_parameters(
            psDec,
            &mut sDecCtrl,
            &mut Pulses,
            1 as libc::c_int,
        );
        if (*psDec).sRC.error != 0 {
            (*psDec).nBytesLeft = 0 as libc::c_int;
            action = 1 as libc::c_int;
            SKP_Silk_decoder_set_fs(psDec, fs_k_hz_old);
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
            skp_silk_decode_core(
                psDec,
                &mut sDecCtrl,
                pOut,
                &Pulses,
            );
            skp_silk_plc(psDec, &mut sDecCtrl, pOut, action);
            (*psDec).lossCnt = 0 as libc::c_int;
            (*psDec).prev_sig_type = sDecCtrl.sig_type;
            (*psDec).first_frame_after_reset = 0 as libc::c_int;
        }
    }
    if action == 1 as libc::c_int {
        skp_silk_plc(psDec, &mut sDecCtrl, pOut, action);
    }
    memcpy(
        ((*psDec).outBuf).as_mut_ptr() as *mut libc::c_void,
        pOut.as_mut_ptr() as *const libc::c_void,
        (L as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as libc::c_ulong),
    );
    skp_silk_plc_glue_frames(psDec, &mut sDecCtrl, pOut, L as usize);
    skp_silk_cng(psDec, &mut sDecCtrl, &mut pOut, L as usize);
    skp_silk_biquad(
        &pOut.to_vec(),
        psDec.HP_B.unwrap(),
        psDec.HP_A.unwrap(),
        &mut psDec.HPState,
        &mut pOut,
        L as usize,
    );
    *pN = L as libc::c_short;
    (*psDec).lagPrev = sDecCtrl.pitchL[(4 as libc::c_int - 1 as libc::c_int) as usize];
    return ret;
}
