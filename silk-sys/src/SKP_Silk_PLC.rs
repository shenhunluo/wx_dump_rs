use crate::{
    i16_to_i32, skp_l_shift, skp_r_shift, skp_r_shift_round, skp_rand, skp_s_mla_w_b,
    skp_s_mla_w_t, skp_s_mul_b_b, skp_s_mul_w_b, skp_s_mul_w_w, skp_sat_16,
    skp_silk_bwexpander::skp_silk_bwexpander,
    skp_silk_lpc_inv_pred_gain::skp_silk_lpc_inverse_pred_gain,
    skp_silk_sum_sqr_shift::skp_silk_sum_sqr_shift,
    skp_utils::{skp_silk_clz32, skp_silk_sqrt_approx, NB_SUBFR, LTP_ORDER, V_PITCH_GAIN_START_MIN_Q14, MAX_LPC_ORDER},
    SKP_Silk_dec_API::{SKP_Silk_decoder_control, SkpSilkDecoderStruct},
};

#[derive(Copy, Clone)]
pub struct SkpSilkPlcStruct {
    pub pitch_l_q8: i32,
    pub ltp_coef_q14: [i16; 5],
    pub prev_lpc_q12: [i16; 16],
    pub last_frame_lost: i32,
    pub rand_seed: i32,
    pub rand_scale_q14: i16,
    pub conc_energy: i32,
    pub conc_energy_shift: i32,
    pub prev_ltp_scale_q14: i16,
    pub prev_gain_q16: [i32; 4],
    pub fs_k_hz: i32,
}

impl Default for SkpSilkPlcStruct {
    fn default() -> Self {
        Self {
            pitch_l_q8: Default::default(),
            ltp_coef_q14: Default::default(),
            prev_lpc_q12: Default::default(),
            last_frame_lost: Default::default(),
            rand_seed: Default::default(),
            rand_scale_q14: Default::default(),
            conc_energy: Default::default(),
            conc_energy_shift: Default::default(),
            prev_ltp_scale_q14: Default::default(),
            prev_gain_q16: Default::default(),
            fs_k_hz: Default::default(),
        }
    }
}

static HARM_ATT_Q15: [i16; 2] = [32440, 31130];
static PLC_RAND_ATTENUATE_V_Q15: [i16; 2] = [31130, 26214];
static PLC_RAND_ATTENUATE_UV_Q15: [i16; 2] = [32440, 29491];

pub fn skp_silk_plc_reset(ps_dec: &mut SkpSilkDecoderStruct) {
    ps_dec.sPLC.pitch_l_q8 = ps_dec.frame_length >> 1;
}

pub fn skp_silk_plc(
    ps_dec: &mut SkpSilkDecoderStruct,
    ps_dec_ctrl: &mut SKP_Silk_decoder_control,
    signal: &mut [i16],
    lost: i32,
) {
    if ps_dec.fs_k_hz != ps_dec.sPLC.fs_k_hz {
        skp_silk_plc_reset(ps_dec);
        ps_dec.sPLC.fs_k_hz = ps_dec.fs_k_hz;
    }
    if lost != 0 {
        skp_silk_plc_conceal(ps_dec, ps_dec_ctrl, signal);
        (*ps_dec).lossCnt += 1;
        (*ps_dec).lossCnt;
    } else {
        skp_silk_plc_update(ps_dec, ps_dec_ctrl);
    };
}

pub fn skp_silk_plc_update(
    ps_dec: &mut SkpSilkDecoderStruct,
    ps_dec_ctrl: &mut SKP_Silk_decoder_control,
) {
    let ps_plc = &mut ps_dec.sPLC;
    ps_dec.prev_sig_type = ps_dec_ctrl.sig_type;
    let mut ltp_gain_q14 = 0;
    if (*ps_dec_ctrl).sig_type == 0 {
        let mut j = 0;
        while j * (ps_dec.subfr_length as usize) < (ps_dec_ctrl.pitchL[NB_SUBFR - 1] as usize) {
            let mut temp_ltp_gain_q14 = 0;
            for i in 0..LTP_ORDER {
                temp_ltp_gain_q14 +=
                    ps_dec_ctrl.ltp_coef_q14[(NB_SUBFR - 1 - j) * LTP_ORDER + i] as i32;
            }
            if temp_ltp_gain_q14 > ltp_gain_q14 {
                ltp_gain_q14 = temp_ltp_gain_q14;
                for i in 0..LTP_ORDER {
                    ps_plc.ltp_coef_q14[i] = ps_dec_ctrl.ltp_coef_q14
                        [skp_s_mul_b_b!(NB_SUBFR - 1 - j as usize, LTP_ORDER) as usize + i]
                }
                ps_plc.pitch_l_q8 = ps_dec_ctrl.pitchL[NB_SUBFR - 1 - j] << 8;
            }
            j += 1;
        }
        for i in 0..LTP_ORDER {
            ps_plc.ltp_coef_q14[i] = 0;
        }
        ps_plc.ltp_coef_q14[LTP_ORDER / 2] = ltp_gain_q14 as i16;
        if ltp_gain_q14 < V_PITCH_GAIN_START_MIN_Q14 {
            let tmp = V_PITCH_GAIN_START_MIN_Q14 << 10;
            let scale_q10 = tmp / ltp_gain_q14.max(1);
            for i in 0..LTP_ORDER {
                ps_plc.ltp_coef_q14[i] =
                    skp_r_shift!(skp_s_mul_b_b!(ps_plc.ltp_coef_q14[i], scale_q10), 10) as i16;
            }
        } else if ltp_gain_q14 > V_PITCH_GAIN_START_MIN_Q14 {
            let tmp_0 = V_PITCH_GAIN_START_MIN_Q14 << 14;
            let scale_q10 = tmp_0 / ltp_gain_q14.max(1);
            for i in 0..LTP_ORDER {
                ps_plc.ltp_coef_q14[i] =
                    skp_r_shift!(skp_s_mul_b_b!(ps_plc.ltp_coef_q14[i], scale_q10), 14) as i16;
            }
        }
    } else {
        ps_plc.pitch_l_q8 = skp_l_shift!(skp_s_mul_b_b!(ps_dec.fs_k_hz, 18), 8);
        for i in 0..LTP_ORDER {
            ps_plc.ltp_coef_q14[i] = 0;
        }
    }
    for i in 0..ps_dec.LPC_order as usize {
        ps_plc.prev_lpc_q12[i] = ps_dec_ctrl.PredCoef_Q12[1][i];
    }
    ps_plc.prev_ltp_scale_q14 = ps_dec_ctrl.LTP_scale_Q14 as i16;
    for i in 0..NB_SUBFR {
        ps_plc.prev_gain_q16[i] = ps_dec_ctrl.Gains_Q16[i];
    }
}

pub fn skp_silk_plc_conceal(
    ps_dec: &mut SkpSilkDecoderStruct,
    ps_dec_ctrl: &mut SKP_Silk_decoder_control,
    signal: &mut [i16],
) {
    let mut exc_buf = [0; 480];
    let mut sig_q10 = [0; 480];
    let ps_plc = &mut ps_dec.sPLC;
    for i in 0..ps_dec.frame_length as usize {
        ps_dec.sLTP_Q16[i] = ps_dec.sLTP_Q16[ps_dec.frame_length as usize + i];
    }
    skp_silk_bwexpander(&mut ps_plc.prev_lpc_q12, ps_dec.LPC_order as usize, 64880);
    let mut exc_buf_ptr = &mut exc_buf[..];
    for k in (4 >> 1)..4 {
        for i in 0..ps_dec.subfr_length as usize {
            exc_buf_ptr[i] = skp_r_shift!(
                skp_s_mul_w_w!(
                    ps_dec.exc_Q10[i + k * ps_dec.subfr_length as usize],
                    ps_plc.prev_gain_q16[k]
                ),
                10
            ) as i16;
        }
        exc_buf_ptr = &mut exc_buf_ptr[ps_dec.subfr_length as usize..];
    }
    let mut shift1 = 0;
    let mut energy1 = 0;
    skp_silk_sum_sqr_shift(
        &mut energy1,
        &mut shift1,
        &exc_buf,
        ps_dec.subfr_length as usize,
    );
    let mut shift2 = 0;
    let mut energy2 = 0;
    skp_silk_sum_sqr_shift(
        &mut energy2,
        &mut shift2,
        &exc_buf[ps_dec.subfr_length as usize..],
        ps_dec.subfr_length as usize,
    );
    let rand_ptr = if energy1 >> shift2 < energy2 >> shift1 {
        &mut ps_dec.exc_Q10[i32::max(0, 3 * ps_dec.subfr_length - 128) as usize..]
    } else {
        &mut ps_dec.exc_Q10[i32::max(0, ps_dec.subfr_length - 128) as usize..]
    };
    let b_q14 = &mut ps_plc.ltp_coef_q14;
    let mut rand_scale_q14 = ps_plc.rand_scale_q14;
    let harm_gain_q15 = HARM_ATT_Q15[i32::min(2 - 1, ps_dec.lossCnt) as usize];
    let mut rand_gain_q15 = if (*ps_dec).prev_sig_type == 0 {
        PLC_RAND_ATTENUATE_V_Q15[i32::min(2 - 1, ps_dec.lossCnt) as usize]
    } else {
        PLC_RAND_ATTENUATE_UV_Q15[i32::min(2 - 1, ps_dec.lossCnt) as usize]
    } as i32;
    if ps_dec.lossCnt == 0 {
        rand_scale_q14 = (1 << 14) as i16;
        if (*ps_dec).prev_sig_type == 0 {
            for i in 0..5 {
                rand_scale_q14 = rand_scale_q14 - b_q14[i];
            }
            rand_scale_q14 = i16::max(3277, rand_scale_q14);
            rand_scale_q14 = skp_r_shift!(
                skp_s_mul_b_b!(rand_scale_q14, ps_plc.prev_ltp_scale_q14),
                14
            ) as i16;
        }
        if (*ps_dec).prev_sig_type == 1 {
            let mut inv_gain_q30 = 0;
            skp_silk_lpc_inverse_pred_gain(
                &mut inv_gain_q30,
                &mut ((*ps_plc).prev_lpc_q12),
                (*ps_dec).LPC_order as usize,
            );
            let mut down_scale_q30 = i32::min(1 << 30 >> 3, inv_gain_q30);
            down_scale_q30 = i32::max(1 << 30 >> 8, down_scale_q30);
            down_scale_q30 = down_scale_q30 << 3;

            rand_gain_q15 = skp_r_shift!(skp_s_mul_w_b!(down_scale_q30, rand_gain_q15), 14);
        }
    }
    let mut rand_seed = (*ps_plc).rand_seed;
    let mut lag = skp_r_shift_round!(ps_plc.pitch_l_q8, 8);
    let mut s_ltp_buf_idx = ps_dec.frame_length;
    let mut sig_q10_ptr = &mut sig_q10[..];
    for _ in 0..4 {
        let mut pred_lag_ptr = (s_ltp_buf_idx - lag + 5 / 2) as usize;
        for i in 0..ps_dec.subfr_length as usize {
            rand_seed = skp_rand!(rand_seed);
            let idx = rand_seed >> 25 & 128 - 1;
            let mut ltp_pred_q14 = skp_s_mul_w_b!(ps_dec.sLTP_Q16[pred_lag_ptr - 0], b_q14[0]);
            ltp_pred_q14 =
                skp_s_mla_w_b!(ltp_pred_q14, ps_dec.sLTP_Q16[pred_lag_ptr - 1], b_q14[1]);
            ltp_pred_q14 =
                skp_s_mla_w_b!(ltp_pred_q14, ps_dec.sLTP_Q16[pred_lag_ptr - 2], b_q14[2]);
            ltp_pred_q14 =
                skp_s_mla_w_b!(ltp_pred_q14, ps_dec.sLTP_Q16[pred_lag_ptr - 3], b_q14[3]);
            ltp_pred_q14 =
                skp_s_mla_w_b!(ltp_pred_q14, ps_dec.sLTP_Q16[pred_lag_ptr - 4], b_q14[4]);
            pred_lag_ptr += if pred_lag_ptr == ps_dec.sLTP_Q16.len() - 1 {
                0
            } else {
                1
            };
            let lpc_exc_q10 =
                skp_l_shift!(skp_s_mul_w_b!(rand_ptr[idx as usize], rand_scale_q14), 2);
            let lpc_exc_q10 = lpc_exc_q10 + skp_r_shift_round!(ltp_pred_q14, 4);
            (*ps_dec).sLTP_Q16[s_ltp_buf_idx as usize] = lpc_exc_q10 << 6;
            s_ltp_buf_idx += 1;
            sig_q10_ptr[i] = lpc_exc_q10;
        }
        sig_q10_ptr = &mut sig_q10_ptr[ps_dec.subfr_length as usize..];
        for j in 0..5 {
            b_q14[j] = skp_r_shift!(skp_s_mul_b_b!(harm_gain_q15, b_q14[j]), 15) as i16;
        }
        rand_scale_q14 =
            skp_r_shift!(skp_s_mul_b_b!(rand_scale_q14, rand_gain_q15 as i16), 15) as i16;
        ps_plc.pitch_l_q8 += skp_s_mul_w_b!(ps_plc.pitch_l_q8, 655);
        ps_plc.pitch_l_q8 = i32::min(
            ps_plc.pitch_l_q8,
            skp_l_shift!(skp_s_mul_b_b!(18, ps_dec.fs_k_hz), 8),
        );
        lag = skp_r_shift_round!(ps_plc.pitch_l_q8, 8);
    }
    sig_q10_ptr = &mut sig_q10[..];
    let mut a_q12_tmp = [0; 16];
    for i in 0..ps_dec.LPC_order as usize {
        a_q12_tmp[i] = ps_plc.prev_lpc_q12[i];
    }
    for _ in 0..4 {
        for i in 0..ps_dec.subfr_length as usize {
            let a_tmp = i16_to_i32!(a_q12_tmp[0], a_q12_tmp[1]);
            let mut lpc_pred_q10 = skp_s_mul_w_b!(ps_dec.sLPC_Q14[MAX_LPC_ORDER + i - 1], a_tmp);
            lpc_pred_q10 =
                skp_s_mla_w_t!(lpc_pred_q10, ps_dec.sLPC_Q14[MAX_LPC_ORDER + i - 2], a_tmp);
            let a_tmp = i16_to_i32!(a_q12_tmp[2], a_q12_tmp[3]);
            lpc_pred_q10 =
                skp_s_mla_w_b!(lpc_pred_q10, ps_dec.sLPC_Q14[MAX_LPC_ORDER + i - 3], a_tmp);
            lpc_pred_q10 =
                skp_s_mla_w_t!(lpc_pred_q10, ps_dec.sLPC_Q14[MAX_LPC_ORDER + i - 4], a_tmp);
            let a_tmp = i16_to_i32!(a_q12_tmp[4], a_q12_tmp[5]);
            lpc_pred_q10 =
                skp_s_mla_w_b!(lpc_pred_q10, ps_dec.sLPC_Q14[MAX_LPC_ORDER + i - 5], a_tmp);
            lpc_pred_q10 =
                skp_s_mla_w_t!(lpc_pred_q10, ps_dec.sLPC_Q14[MAX_LPC_ORDER + i - 6], a_tmp);
            let a_tmp = i16_to_i32!(a_q12_tmp[6], a_q12_tmp[7]);
            lpc_pred_q10 =
                skp_s_mla_w_b!(lpc_pred_q10, ps_dec.sLPC_Q14[MAX_LPC_ORDER + i - 7], a_tmp);
            lpc_pred_q10 =
                skp_s_mla_w_t!(lpc_pred_q10, ps_dec.sLPC_Q14[MAX_LPC_ORDER + i - 8], a_tmp);
            let a_tmp = i16_to_i32!(a_q12_tmp[8], a_q12_tmp[9]);
            lpc_pred_q10 =
                skp_s_mla_w_b!(lpc_pred_q10, ps_dec.sLPC_Q14[MAX_LPC_ORDER + i - 9], a_tmp);
            lpc_pred_q10 =
                skp_s_mla_w_t!(lpc_pred_q10, ps_dec.sLPC_Q14[MAX_LPC_ORDER + i - 10], a_tmp);
            for j in (10..ps_dec.LPC_order as usize).step_by(2) {
                let a_tmp = i16_to_i32!(a_q12_tmp[j], a_q12_tmp[j + 1]);
                lpc_pred_q10 = skp_s_mla_w_b!(
                    lpc_pred_q10,
                    ps_dec.sLPC_Q14[MAX_LPC_ORDER + i - 1 - j],
                    a_tmp
                );
                lpc_pred_q10 = skp_s_mla_w_t!(
                    lpc_pred_q10,
                    ps_dec.sLPC_Q14[MAX_LPC_ORDER + i - 2 - j],
                    a_tmp
                );
            }
            sig_q10_ptr[i] = sig_q10_ptr[i] + lpc_pred_q10;
            ps_dec.sLPC_Q14[16 + i] = sig_q10_ptr[i] << 4;
        }
        sig_q10_ptr = &mut sig_q10_ptr[ps_dec.subfr_length as usize..];
        for i in 0..16 {
            ps_dec.sLPC_Q14[i] = ps_dec.sLPC_Q14[ps_dec.subfr_length as usize + i];
        }
    }
    for i in 0..ps_dec.frame_length as usize {
        signal[i] = skp_sat_16!(
            skp_r_shift_round!(skp_s_mul_w_w!(sig_q10[i], ps_plc.prev_gain_q16[4 - 1]), 10),
            i32
        ) as i16;
    }
    (*ps_plc).rand_seed = rand_seed;
    (*ps_plc).rand_scale_q14 = rand_scale_q14;
    for i in 0..4 {
        (*ps_dec_ctrl).pitchL[i] = lag;
    }
}

pub fn skp_silk_plc_glue_frames(
    ps_dec: &mut SkpSilkDecoderStruct,
    _ps_dec_ctrl: &mut SKP_Silk_decoder_control,
    signal: &mut [i16],
    length: usize,
) {
    let ps_plc = &mut ps_dec.sPLC;
    if (*ps_dec).lossCnt != 0 {
        skp_silk_sum_sqr_shift(
            &mut (*ps_plc).conc_energy,
            &mut (*ps_plc).conc_energy_shift,
            signal,
            length as usize,
        );
        (*ps_plc).last_frame_lost = 1;
    } else {
        let mut energy_shift = 0;
        let mut energy = 0;
        if ps_plc.last_frame_lost != 0 {
            skp_silk_sum_sqr_shift(&mut energy, &mut energy_shift, signal, length as usize);
            if energy_shift > (*ps_plc).conc_energy_shift {
                (*ps_plc).conc_energy =
                    (*ps_plc).conc_energy >> energy_shift - (*ps_plc).conc_energy_shift;
            } else if energy_shift < (*ps_plc).conc_energy_shift {
                energy = energy >> (*ps_plc).conc_energy_shift - energy_shift;
            }
            if energy > (*ps_plc).conc_energy {
                let lz = skp_silk_clz32((*ps_plc).conc_energy);
                let lz = lz - 1;
                (*ps_plc).conc_energy = (*ps_plc).conc_energy << lz;
                energy = energy >> i32::max(24 - lz, 0);
                let frac_q24 = ps_plc.conc_energy / energy.max(1);
                let mut gain_q12 = skp_silk_sqrt_approx(frac_q24);
                let slope_q12 = ((1 << 12) - gain_q12) / length as i32;
                for i in 0..length as usize {
                    signal[i] = (gain_q12 * signal[i] as i32 >> 12) as i16;
                    gain_q12 += slope_q12;
                    gain_q12 = gain_q12.min(1 << 12);
                }
            }
        }
        (*ps_plc).last_frame_lost = 0;
    };
}
