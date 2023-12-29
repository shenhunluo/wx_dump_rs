use crate::{
    skp_silk_biquad::skp_silk_biquad,
    skp_silk_cng::skp_silk_cng,
    skp_silk_decode_core::skp_silk_decode_core,
    skp_silk_decode_parameters::skp_silk_decode_parameters,
    skp_silk_plc::{skp_silk_plc, skp_silk_plc_glue_frames},
    skp_utils::NB_SUBFR,
    SKP_Silk_dec_API::{SKP_Silk_decoder_control, SkpSilkDecoderStruct},
    skp_silk_decoder_set_fs::skp_silk_decoder_set_fs,
    SKP_Silk_range_coder::SKP_Silk_range_dec_init,
};

#[no_mangle]
pub fn skp_silk_decode_frame(
    ps_dec: &mut SkpSilkDecoderStruct,
    mut p_out: &mut [i16],
    p_n: &mut usize,
    p_code: &[u8],
    n_bytes: i32,
    mut action: i32,
    dec_bytes: &mut i32,
) -> i32 {
    let mut s_dec_ctrl: SKP_Silk_decoder_control = SKP_Silk_decoder_control {
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
    let mut ret = 0;
    let mut len = ps_dec.frame_length as usize;
    s_dec_ctrl.LTP_scale_Q14 = 0;
    *dec_bytes = 0;
    if action == 0 {
        let fs_k_hz_old = (*ps_dec).fs_k_hz;
        if (*ps_dec).nFramesDecoded == 0 {
            SKP_Silk_range_dec_init(&mut (*ps_dec).sRC, p_code, n_bytes);
        }
        let mut pulses = [0; 480];
        skp_silk_decode_parameters(ps_dec, &mut s_dec_ctrl, &mut pulses, 1);
        if (*ps_dec).sRC.error != 0 {
            (*ps_dec).nBytesLeft = 0;
            action = 1;
            skp_silk_decoder_set_fs(ps_dec, fs_k_hz_old);
            *dec_bytes = ps_dec.sRC.bufferLength;
            if ps_dec.sRC.error == -8 {
                ret = -11;
            } else {
                ret = -12;
            }
        } else {
            *dec_bytes = (*ps_dec).sRC.bufferLength - (*ps_dec).nBytesLeft;
            (*ps_dec).nFramesDecoded += 1;
            (*ps_dec).nFramesDecoded;
            len = ps_dec.frame_length as usize;
            skp_silk_decode_core(ps_dec, &mut s_dec_ctrl, p_out, &pulses);
            skp_silk_plc(ps_dec, &mut s_dec_ctrl, p_out, action);
            (*ps_dec).lossCnt = 0;
            (*ps_dec).prev_sig_type = s_dec_ctrl.sig_type;
            (*ps_dec).first_frame_after_reset = 0;
        }
    }
    if action == 1 {
        skp_silk_plc(ps_dec, &mut s_dec_ctrl, p_out, action);
    }
    for i in 0..len {
        ps_dec.outBuf[i] = p_out[i];
    }
    skp_silk_plc_glue_frames(ps_dec, &mut s_dec_ctrl, p_out, len);
    skp_silk_cng(ps_dec, &mut s_dec_ctrl, &mut p_out, len);
    skp_silk_biquad(
        &p_out.to_vec(),
        ps_dec.HP_B.unwrap(),
        ps_dec.HP_A.unwrap(),
        &mut ps_dec.HPState,
        &mut p_out,
        len,
    );
    *p_n = len;
    (*ps_dec).lagPrev = s_dec_ctrl.pitchL[NB_SUBFR - 1];
    return ret;
}
