use crate::{
    skp_silk_biquad::skp_silk_biquad,
    skp_silk_cng::skp_silk_cng,
    skp_silk_decode_core::skp_silk_decode_core,
    skp_silk_decode_parameters::skp_silk_decode_parameters,
    skp_silk_plc::{skp_silk_plc, skp_silk_plc_glue_frames},
    skp_utils::NB_SUBFR,
    skp_silk_dec_api::{SkpSilkDecoderControl, SkpSilkDecoderStruct},
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
    let mut s_dec_ctrl = SkpSilkDecoderControl::default();
    let mut ret = 0;
    let mut len = ps_dec.frame_length as usize;
    s_dec_ctrl.ltp_scale_q14 = 0;
    *dec_bytes = 0;
    if action == 0 {
        let fs_k_hz_old = (*ps_dec).fs_k_hz;
        if (*ps_dec).n_frames_decoded == 0 {
            SKP_Silk_range_dec_init(&mut (*ps_dec).s_r_c, p_code, n_bytes);
        }
        let mut pulses = [0; 480];
        skp_silk_decode_parameters(ps_dec, &mut s_dec_ctrl, &mut pulses, 1);
        if (*ps_dec).s_r_c.error != 0 {
            (*ps_dec).n_bytes_left = 0;
            action = 1;
            skp_silk_decoder_set_fs(ps_dec, fs_k_hz_old);
            *dec_bytes = ps_dec.s_r_c.bufferLength;
            if ps_dec.s_r_c.error == -8 {
                ret = -11;
            } else {
                ret = -12;
            }
        } else {
            *dec_bytes = (*ps_dec).s_r_c.bufferLength - (*ps_dec).n_bytes_left;
            (*ps_dec).n_frames_decoded += 1;
            (*ps_dec).n_frames_decoded;
            len = ps_dec.frame_length as usize;
            skp_silk_decode_core(ps_dec, &mut s_dec_ctrl, p_out, &pulses);
            skp_silk_plc(ps_dec, &mut s_dec_ctrl, p_out, action);
            (*ps_dec).loss_cnt = 0;
            (*ps_dec).prev_sig_type = s_dec_ctrl.sig_type;
            (*ps_dec).first_frame_after_reset = 0;
        }
    }
    if action == 1 {
        skp_silk_plc(ps_dec, &mut s_dec_ctrl, p_out, action);
    }
    for i in 0..len {
        ps_dec.out_buf[i] = p_out[i];
    }
    skp_silk_plc_glue_frames(ps_dec, &mut s_dec_ctrl, p_out, len);
    skp_silk_cng(ps_dec, &mut s_dec_ctrl, &mut p_out, len);
    skp_silk_biquad(
        &p_out.to_vec(),
        ps_dec.hp_b.unwrap(),
        ps_dec.hp_a.unwrap(),
        &mut ps_dec.hp_state,
        &mut p_out,
        len,
    );
    *p_n = len;
    (*ps_dec).lag_prev = s_dec_ctrl.pitch_l[NB_SUBFR - 1];
    return ret;
}
