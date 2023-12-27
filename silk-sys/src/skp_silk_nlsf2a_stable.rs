use crate::{
    skp_s_mul_b_b, skp_silk_bwexpander::skp_silk_bwexpander,
    skp_silk_lpc_inv_pred_gain::skp_silk_lpc_inverse_pred_gain, skp_silk_nlsf2a::skp_silk_nlsf2a,
};

pub fn skp_silk_nlsf2a_stable(p_ar_q12: &mut [i16], p_nlsf: &[i32], lpc_order: usize) {
    skp_silk_nlsf2a(p_ar_q12, p_nlsf, lpc_order);
    let mut i = 0;
    while i < 20 {
        if !(skp_silk_lpc_inverse_pred_gain(&mut 0, p_ar_q12, lpc_order) == 1) {
            break;
        }
        skp_silk_bwexpander(p_ar_q12, lpc_order, 65536 - skp_s_mul_b_b!(10 + i, i));
        i += 1;
    }
    if i == 20 {
        for i in 0..lpc_order {
            p_ar_q12[i] = 0;
        }
    }
}
