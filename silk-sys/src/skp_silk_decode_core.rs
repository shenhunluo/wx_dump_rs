use crate::{
    i16_to_i32, skp_l_shift, skp_r_shift, skp_r_shift_round, skp_rand, skp_s_mla_w_b,
    skp_s_mla_w_t, skp_s_mul_w_b, skp_s_mul_w_w, skp_sat_16,
    skp_silk_tables_other::SKP_SILK_QUANTIZATION_OFFSETS_Q10,
    skp_utils::{skp_div32_var_q, skp_inverse32_var_q, MAX_LPC_ORDER},
};
use crate::{
    SKP_Silk_MA::skp_silk_ma_prediction,
    skp_silk_dec_api::{SkpSilkDecoderControl, SkpSilkDecoderStruct},
};

pub fn skp_silk_decode_core(
    ps_dec: &mut SkpSilkDecoderStruct,
    ps_dec_ctrl: &mut SkpSilkDecoderControl,
    xq: &mut [i16],
    q: &[i32],
) {
    let mut lag = 0;
    let mut s_ltp = [0; 480];
    let mut vec_q10 = [0; 120];
    let offset_q10 = SKP_SILK_QUANTIZATION_OFFSETS_Q10[ps_dec_ctrl.sig_type as usize]
        [ps_dec_ctrl.quant_offset_type as usize] as i32;
    let nlsf_interpolation_flag = if ps_dec_ctrl.nlsf_interp_coef_q2 < 1 << 2 {
        1
    } else {
        0
    };
    let mut rand_seed = ps_dec_ctrl.seed;
    for i in 0..ps_dec.frame_length as usize {
        rand_seed = skp_rand!(rand_seed);
        let dither = skp_r_shift!(rand_seed, 31);
        ps_dec.exc_q10[i] = skp_l_shift!(q[i], 10) + offset_q10;
        ps_dec.exc_q10[i] = (ps_dec.exc_q10[i] ^ dither) - dither;
        rand_seed += q[i];
    }
    let mut p_exc_q10 = &mut ps_dec.exc_q10[..];
    let mut p_res_q10 = &mut ps_dec.res_q10[..];
    let mut pxq = ps_dec.frame_length as usize;
    let mut s_ltp_buf_idx = ps_dec.frame_length as usize;
    for k in 0..4 {
        let a_q12 = &ps_dec_ctrl.pred_coef_q12[k >> 1];
        let mut a_q12_temp = a_q12.clone();
        let b_q14 = &mut ps_dec_ctrl.ltp_coef_q14[k * 5..];
        let gain_q16 = ps_dec_ctrl.gains_q16[k as usize];
        let mut sig_type = ps_dec_ctrl.sig_type;
        let inv_gain_q16 = skp_inverse32_var_q(gain_q16.max(1), 32);
        let inv_gain_q16 = inv_gain_q16.min(i16::MAX as i32);
        let mut gain_adj_q16 = 1 << 16;
        if inv_gain_q16 != ps_dec.prev_inv_gain_q16 {
            gain_adj_q16 = skp_div32_var_q(inv_gain_q16, ps_dec.prev_inv_gain_q16, 16);
        }
        if ps_dec.loss_cnt != 0
            && ps_dec.prev_sig_type == 0
            && ps_dec_ctrl.sig_type == 1
            && k < 4 >> 1
        {
            for i in 0..5 {
                b_q14[i] = 0;
            }
            b_q14[5 / 2] = 1 << 12;
            sig_type = 0;
            ps_dec_ctrl.pitch_l[k as usize] = ps_dec.lag_prev;
        }
        if sig_type == 0 {
            lag = ps_dec_ctrl.pitch_l[k as usize];
            if k as i32 & 3 - skp_l_shift!(nlsf_interpolation_flag, 1) == 0 {
                let start_idx = (ps_dec.frame_length - lag - ps_dec.lpc_order - 5 / 2) as usize;
                let mut filt_state = [0; 16];
                skp_silk_ma_prediction(
                    &ps_dec.out_buf[(start_idx + k * (ps_dec.frame_length as usize >> 2))..],
                    a_q12,
                    &mut filt_state,
                    &mut s_ltp[start_idx as usize..],
                    (ps_dec.frame_length as usize - start_idx) as usize,
                    ps_dec.lpc_order as usize,
                );
                let mut inv_gain_q32 = skp_l_shift!(inv_gain_q16, 16);
                if k == 0 {
                    inv_gain_q32 =
                        skp_l_shift!(skp_s_mla_w_b!(inv_gain_q32, ps_dec_ctrl.ltp_scale_q14), 2);
                }
                for i in 0..lag as usize + 5 / 2 {
                    ps_dec.s_ltp_q16[s_ltp_buf_idx - i - 1] = skp_s_mul_w_b!(
                        inv_gain_q32,
                        s_ltp[(ps_dec.frame_length as usize - i - 1) as usize]
                    );
                }
            } else if gain_adj_q16 != 1 << 16 {
                for i in 0..lag as usize + 5 / 2 {
                    ps_dec.s_ltp_q16[(s_ltp_buf_idx - i - 1) as usize] = skp_s_mul_w_w!(
                        gain_adj_q16,
                        ps_dec.s_ltp_q16[(s_ltp_buf_idx - i - 1) as usize]
                    );
                }
            }
        }
        for i in 0..16 {
            ps_dec.s_lpc_q14[i] = skp_s_mul_w_w!(gain_adj_q16, ps_dec.s_lpc_q14[i]);
        }
        ps_dec.prev_inv_gain_q16 = inv_gain_q16;
        if sig_type == 0 {
            let mut pred_lag_ptr = s_ltp_buf_idx - (lag as usize) + 5 / 2;
            for i in 0..ps_dec.subfr_length as usize {
                let mut ltp_pred_q14 = skp_s_mul_w_b!(ps_dec.s_ltp_q16[pred_lag_ptr - 0], b_q14[0]);
                ltp_pred_q14 =
                    skp_s_mla_w_b!(ltp_pred_q14, ps_dec.s_ltp_q16[pred_lag_ptr - 1], b_q14[1]);
                ltp_pred_q14 =
                    skp_s_mla_w_b!(ltp_pred_q14, ps_dec.s_ltp_q16[pred_lag_ptr - 2], b_q14[2]);
                ltp_pred_q14 =
                    skp_s_mla_w_b!(ltp_pred_q14, ps_dec.s_ltp_q16[pred_lag_ptr - 3], b_q14[3]);
                ltp_pred_q14 =
                    skp_s_mla_w_b!(ltp_pred_q14, ps_dec.s_ltp_q16[pred_lag_ptr - 4], b_q14[4]);
                pred_lag_ptr += 1;
                p_res_q10[i] = p_exc_q10[i] + skp_r_shift_round!(ltp_pred_q14, 4);
                ps_dec.s_ltp_q16[s_ltp_buf_idx as usize] = p_res_q10[i as usize] << 6;
                s_ltp_buf_idx += 1;
            }
        } else {
            for i in 0..ps_dec.subfr_length as usize {
                p_res_q10[i] = p_exc_q10[i];
            }
        }
        skp_silk_decode_short_term_prediction(
            &mut vec_q10,
            &p_res_q10,
            &mut ps_dec.s_lpc_q14,
            &mut a_q12_temp,
            ps_dec.lpc_order,
            ps_dec.subfr_length as usize,
        );
        for i in 0..ps_dec.subfr_length as usize {
            ps_dec.out_buf[pxq + i] = skp_sat_16!(
                skp_r_shift_round!(skp_s_mul_w_w!(vec_q10[i], gain_q16), 10),
                i32
            ) as i16;
        }
        for i in 0..16 {
            ps_dec.s_lpc_q14[i] = ps_dec.s_lpc_q14[ps_dec.subfr_length as usize + i];
        }
        p_exc_q10 = &mut p_exc_q10[ps_dec.subfr_length as usize..];
        p_res_q10 = &mut p_res_q10[ps_dec.subfr_length as usize..];
        pxq += ps_dec.subfr_length as usize;
    }
    for i in 0..ps_dec.frame_length as usize {
        xq[i] = ps_dec.out_buf[ps_dec.frame_length as usize + i];
    }
}

pub fn skp_silk_decode_short_term_prediction(
    vec_q10: &mut [i32],
    p_res_q10: &[i32],
    s_lpc_q14: &mut [i32],
    a_q12_tmp: &mut [i16],
    lpc_order: i32,
    subfr_length: usize,
) {
    if lpc_order == 16 {
        for i in 0..subfr_length {
            /* unrolled */
            let a_tmp = i16_to_i32!(a_q12_tmp[0], a_q12_tmp[1]);
            let mut lpc_pred_q10 = skp_s_mul_w_b!(s_lpc_q14[MAX_LPC_ORDER + i - 1], a_tmp);
            lpc_pred_q10 = skp_s_mla_w_t!(lpc_pred_q10, s_lpc_q14[MAX_LPC_ORDER + i - 2], a_tmp);
            let a_tmp = i16_to_i32!(a_q12_tmp[2], a_q12_tmp[3]);
            lpc_pred_q10 = skp_s_mla_w_b!(lpc_pred_q10, s_lpc_q14[MAX_LPC_ORDER + i - 3], a_tmp);
            lpc_pred_q10 = skp_s_mla_w_t!(lpc_pred_q10, s_lpc_q14[MAX_LPC_ORDER + i - 4], a_tmp);
            let a_tmp = i16_to_i32!(a_q12_tmp[4], a_q12_tmp[5]);
            lpc_pred_q10 = skp_s_mla_w_b!(lpc_pred_q10, s_lpc_q14[MAX_LPC_ORDER + i - 5], a_tmp);
            lpc_pred_q10 = skp_s_mla_w_t!(lpc_pred_q10, s_lpc_q14[MAX_LPC_ORDER + i - 6], a_tmp);
            let a_tmp = i16_to_i32!(a_q12_tmp[6], a_q12_tmp[7]);
            lpc_pred_q10 = skp_s_mla_w_b!(lpc_pred_q10, s_lpc_q14[MAX_LPC_ORDER + i - 7], a_tmp);
            lpc_pred_q10 = skp_s_mla_w_t!(lpc_pred_q10, s_lpc_q14[MAX_LPC_ORDER + i - 8], a_tmp);
            let a_tmp = i16_to_i32!(a_q12_tmp[8], a_q12_tmp[9]);
            lpc_pred_q10 = skp_s_mla_w_b!(lpc_pred_q10, s_lpc_q14[MAX_LPC_ORDER + i - 9], a_tmp);
            lpc_pred_q10 = skp_s_mla_w_t!(lpc_pred_q10, s_lpc_q14[MAX_LPC_ORDER + i - 10], a_tmp);
            let a_tmp = i16_to_i32!(a_q12_tmp[10], a_q12_tmp[11]);
            lpc_pred_q10 = skp_s_mla_w_b!(lpc_pred_q10, s_lpc_q14[MAX_LPC_ORDER + i - 11], a_tmp);
            lpc_pred_q10 = skp_s_mla_w_t!(lpc_pred_q10, s_lpc_q14[MAX_LPC_ORDER + i - 12], a_tmp);
            let a_tmp = i16_to_i32!(a_q12_tmp[12], a_q12_tmp[13]);
            lpc_pred_q10 = skp_s_mla_w_b!(lpc_pred_q10, s_lpc_q14[MAX_LPC_ORDER + i - 13], a_tmp);
            lpc_pred_q10 = skp_s_mla_w_t!(lpc_pred_q10, s_lpc_q14[MAX_LPC_ORDER + i - 14], a_tmp);
            let a_tmp = i16_to_i32!(a_q12_tmp[14], a_q12_tmp[15]);
            lpc_pred_q10 = skp_s_mla_w_b!(lpc_pred_q10, s_lpc_q14[MAX_LPC_ORDER + i - 15], a_tmp);
            lpc_pred_q10 = skp_s_mla_w_t!(lpc_pred_q10, s_lpc_q14[MAX_LPC_ORDER + i - 16], a_tmp);

            /* Add prediction to LPC residual */
            vec_q10[i] = p_res_q10[i] + lpc_pred_q10;

            /* Update states */
            s_lpc_q14[MAX_LPC_ORDER + i] = skp_l_shift!(vec_q10[i], 4);
        }
    } else {
        for i in 0..subfr_length {
            /* unrolled */
            let a_tmp = i16_to_i32!(a_q12_tmp[0], a_q12_tmp[1]);
            let mut lpc_pred_q10 = skp_s_mul_w_b!(s_lpc_q14[MAX_LPC_ORDER + i - 1], a_tmp);
            lpc_pred_q10 = skp_s_mla_w_t!(lpc_pred_q10, s_lpc_q14[MAX_LPC_ORDER + i - 2], a_tmp);
            let a_tmp = i16_to_i32!(a_q12_tmp[2], a_q12_tmp[3]);
            lpc_pred_q10 = skp_s_mla_w_b!(lpc_pred_q10, s_lpc_q14[MAX_LPC_ORDER + i - 3], a_tmp);
            lpc_pred_q10 = skp_s_mla_w_t!(lpc_pred_q10, s_lpc_q14[MAX_LPC_ORDER + i - 4], a_tmp);
            let a_tmp = i16_to_i32!(a_q12_tmp[4], a_q12_tmp[5]);
            lpc_pred_q10 = skp_s_mla_w_b!(lpc_pred_q10, s_lpc_q14[MAX_LPC_ORDER + i - 5], a_tmp);
            lpc_pred_q10 = skp_s_mla_w_t!(lpc_pred_q10, s_lpc_q14[MAX_LPC_ORDER + i - 6], a_tmp);
            let a_tmp = i16_to_i32!(a_q12_tmp[6], a_q12_tmp[7]);
            lpc_pred_q10 = skp_s_mla_w_b!(lpc_pred_q10, s_lpc_q14[MAX_LPC_ORDER + i - 7], a_tmp);
            lpc_pred_q10 = skp_s_mla_w_t!(lpc_pred_q10, s_lpc_q14[MAX_LPC_ORDER + i - 8], a_tmp);
            let a_tmp = i16_to_i32!(a_q12_tmp[8], a_q12_tmp[9]);
            lpc_pred_q10 = skp_s_mla_w_b!(lpc_pred_q10, s_lpc_q14[MAX_LPC_ORDER + i - 9], a_tmp);
            lpc_pred_q10 = skp_s_mla_w_t!(lpc_pred_q10, s_lpc_q14[MAX_LPC_ORDER + i - 10], a_tmp);
            vec_q10[i] = p_res_q10[i] + lpc_pred_q10;
            s_lpc_q14[MAX_LPC_ORDER + i] = skp_l_shift!(vec_q10[i], 4);
        }
    };
}
