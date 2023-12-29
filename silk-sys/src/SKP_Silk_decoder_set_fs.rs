use crate::SKP_Silk_dec_API::SkpSilkDecoderStruct;
use crate::skp_s_mul_b_b;
use crate::skp_silk_tables_nlsf_cb0_10::SKP_SILK_NLSF_CB0_10;
use crate::skp_silk_tables_nlsf_cb0_16::SKP_SILK_NLSF_CB0_16;
use crate::skp_silk_tables_nlsf_cb1_10::SKP_SILK_NLSF_CB1_10;
use crate::skp_silk_tables_nlsf_cb1_16::SKP_SILK_NLSF_CB1_16;
use crate::skp_silk_tables_other::{SKP_SILK_DEC_A_HP_24, SKP_SILK_DEC_B_HP_24, SKP_SILK_DEC_B_HP_16, SKP_SILK_DEC_A_HP_16, SKP_SILK_DEC_A_HP_12, SKP_SILK_DEC_B_HP_12, SKP_SILK_DEC_A_HP_8, SKP_SILK_DEC_B_HP_8};


pub fn skp_silk_decoder_set_fs(
    p_s_dec: &mut SkpSilkDecoderStruct,
    fs_k_hz: i32,
) {
    if p_s_dec.fs_k_hz != fs_k_hz {
        p_s_dec.fs_k_hz = fs_k_hz;
        p_s_dec.frame_length = skp_s_mul_b_b!(20, fs_k_hz);
        p_s_dec.subfr_length = skp_s_mul_b_b!(20 / 4, fs_k_hz);
        if p_s_dec.fs_k_hz == 8 {
            p_s_dec.LPC_order = 10;
            p_s_dec.psNLSF_CB[0] = Some(&SKP_SILK_NLSF_CB0_10);
            p_s_dec.psNLSF_CB[1] = Some(&SKP_SILK_NLSF_CB1_10);
        } else {
            p_s_dec.LPC_order = 16;
            p_s_dec.psNLSF_CB[0] = Some(&SKP_SILK_NLSF_CB0_16);
            p_s_dec.psNLSF_CB[1] = Some(&SKP_SILK_NLSF_CB1_16);
        }
        for i in &mut p_s_dec.sLPC_Q14[0..16] {
            *i = 0;
        }
        for i in &mut p_s_dec.outBuf[0..20*24] {
            *i = 0;
        }
        for i in &mut p_s_dec.prevNLSF_Q15[0..16] {
            *i = 0;
        }
        p_s_dec.lagPrev = 100;
        p_s_dec.LastGainIndex = 1;
        p_s_dec.prev_sig_type = 0;
        p_s_dec.first_frame_after_reset = 1;
        if fs_k_hz == 24 {
            p_s_dec.HP_A = Some(&SKP_SILK_DEC_A_HP_24);
            p_s_dec.HP_B = Some(&SKP_SILK_DEC_B_HP_24);
        } else if fs_k_hz == 16 {
            p_s_dec.HP_A = Some(&SKP_SILK_DEC_A_HP_16);
            p_s_dec.HP_B = Some(&SKP_SILK_DEC_B_HP_16);
        } else if fs_k_hz == 12 {
            p_s_dec.HP_A = Some(&SKP_SILK_DEC_A_HP_12);
            p_s_dec.HP_B = Some(&SKP_SILK_DEC_B_HP_12);
        } else if fs_k_hz == 8 {
            p_s_dec.HP_A = Some(&SKP_SILK_DEC_A_HP_8);
            p_s_dec.HP_B = Some(&SKP_SILK_DEC_B_HP_8);
        }
    }
}
