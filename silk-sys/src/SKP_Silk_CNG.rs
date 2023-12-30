use crate::{
    skp_r_shift_round, skp_rand, skp_s_mul_w_b, skp_s_mul_w_w, skp_sat_16,
    skp_silk_lpc_synthesis_filter::skp_silk_lpc_synthesis_filter,
    skp_silk_lpc_synthesis_order16::skp_silk_lpc_synthesis_order16,
    skp_silk_nlsf2a_stable::skp_silk_nlsf2a_stable,
    skp_utils::{CNG_GAIN_SMTH_Q16, CNG_NLSF_SMTH_Q16, NB_SUBFR},
    skp_silk_dec_api::{SkpSilkDecoderControl, SkpSilkDecoderStruct},
};
#[derive(Copy, Clone)]
pub struct SkpSilkCngStruct {
    pub cng_exc_buf_q10: [i32; 480],
    pub cng_smth_nlsf_q15: [i32; 16],
    pub cng_synth_state: [i32; 16],
    pub cng_smth_gain_q16: i32,
    pub rand_seed: i32,
    pub fs_k_hz: i32,
}

impl Default for SkpSilkCngStruct {
    fn default() -> Self {
        Self {
            cng_exc_buf_q10: [0; 480],
            cng_smth_nlsf_q15: Default::default(),
            cng_synth_state: Default::default(),
            cng_smth_gain_q16: Default::default(),
            rand_seed: Default::default(),
            fs_k_hz: Default::default(),
        }
    }
}

fn skp_silk_cng_exc(
    residual: &mut [i16],
    exc_buf_q10: &[i32],
    gain_q16: i32,
    length: usize,
    rand_seed: &mut i32,
) {
    let mut exc_mask = 255;
    while exc_mask > length as i32 {
        exc_mask = exc_mask >> 1;
    }
    let mut seed = *rand_seed;
    for i in 0..length {
        seed = skp_rand!(seed);
        let idx = (seed >> 24 & exc_mask) as usize;
        residual[i] = skp_sat_16!(
            skp_r_shift_round!(skp_s_mul_w_w!(exc_buf_q10[idx], gain_q16), 10),
            i32
        ) as i16;
    }
    *rand_seed = seed;
}

pub fn skp_silk_cng_reset(ps_dec: &mut SkpSilkDecoderStruct) {
    let nlsf_step_q15 = 0x7fff / (ps_dec.lpc_order + 1);
    let mut nlsf_acc_q15 = 0;
    for i in 0..ps_dec.lpc_order as usize {
        nlsf_acc_q15 += nlsf_step_q15;
        ps_dec.s_cng.cng_smth_nlsf_q15[i] = nlsf_acc_q15;
    }
    ps_dec.s_cng.cng_smth_gain_q16 = 0;
    ps_dec.s_cng.rand_seed = 3176576;
}

pub fn skp_silk_cng(
    ps_dec: &mut SkpSilkDecoderStruct,
    ps_dec_ctrl: &mut SkpSilkDecoderControl,
    signal: &mut [i16],
    length: usize,
) {
    let mut lpc_buf = [0; 16];
    let mut cng_sig = [0; 480];
    if (*ps_dec).fs_k_hz != ps_dec.s_cng.fs_k_hz {
        skp_silk_cng_reset(ps_dec);
        ps_dec.s_cng.fs_k_hz = ps_dec.fs_k_hz;
    }
    let p_s_cng = &mut ps_dec.s_cng;
    if (*ps_dec).loss_cnt == 0 && (*ps_dec).vad_flag == 0 {
        for i in 0..ps_dec.lpc_order as usize {
            p_s_cng.cng_smth_nlsf_q15[i] += skp_s_mul_w_b!(
                ps_dec.prev_nlsf_q15[i] - p_s_cng.cng_smth_nlsf_q15[i],
                CNG_NLSF_SMTH_Q16
            );
        }
        let mut max_gain_q16 = 0;
        let mut subfr = 0;
        for i in 0..4 {
            if ps_dec_ctrl.gains_q16[i] > max_gain_q16 {
                max_gain_q16 = ps_dec_ctrl.gains_q16[i];
                subfr = i as i32;
            }
        }
        let data_clone =
            p_s_cng.cng_exc_buf_q10[..ps_dec.subfr_length as usize * (NB_SUBFR - 1)].to_vec();
        for i in 0..ps_dec.subfr_length as usize * (NB_SUBFR - 1) {
            p_s_cng.cng_exc_buf_q10[ps_dec.subfr_length as usize + i] = data_clone[i];
        }
        for i in 0..ps_dec.subfr_length as usize {
            p_s_cng.cng_exc_buf_q10[i] = ps_dec.exc_q10[(subfr * ps_dec.subfr_length) as usize];
        }
        for i in 0..NB_SUBFR {
            p_s_cng.cng_smth_gain_q16 += skp_s_mul_w_b!(
                ps_dec_ctrl.gains_q16[i] - p_s_cng.cng_smth_gain_q16,
                CNG_GAIN_SMTH_Q16
            );
        }
    }
    if (*ps_dec).loss_cnt != 0 {
        skp_silk_cng_exc(
            &mut cng_sig,
            &p_s_cng.cng_exc_buf_q10,
            p_s_cng.cng_smth_gain_q16,
            length,
            &mut p_s_cng.rand_seed,
        );
        skp_silk_nlsf2a_stable(
            &mut lpc_buf,
            &p_s_cng.cng_smth_nlsf_q15,
            ps_dec.lpc_order as usize,
        );
        let gain_q26 = 1 << 26;
        if (*ps_dec).lpc_order == 16 {
            skp_silk_lpc_synthesis_order16(
                &cng_sig.clone(),
                &lpc_buf,
                gain_q26,
                &mut p_s_cng.cng_synth_state,
                &mut cng_sig,
                length,
            );
        } else {
            skp_silk_lpc_synthesis_filter(
                &cng_sig.clone(),
                &lpc_buf,
                gain_q26,
                &mut p_s_cng.cng_synth_state,
                &mut cng_sig,
                length,
                ps_dec.lpc_order as usize,
            );
        }
        for i in 0..length {
            let tmp_32 = (signal[i] + cng_sig[i]) as i32;
            signal[i] = skp_sat_16!(tmp_32, i32) as i16;
        }
    } else {
        for i in 0..ps_dec.lpc_order as usize {
            p_s_cng.cng_synth_state[i] = 0;
        }
    };
}
