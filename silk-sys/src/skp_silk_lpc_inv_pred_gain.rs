use crate::{
    skp_l_shift, skp_r_shift_round, skp_r_shift_sat_32, skp_s_m_mul, skp_s_mla_w_w, skp_s_mul_w_b, skp_utils::skp_silk_clz32,
};

fn skp_inverse32_var_q(b32: i32, q_res: i32) -> i32 {
    let b_head_rm = skp_silk_clz32(b32.abs()) - 1;
    let b32_nrm = b32 << b_head_rm;
    let b32_inv = (i32::MAX >> 2) / (b32_nrm >> 16);
    let mut result = b32_inv << 16;
    let err_q32 = -skp_s_mul_w_b!(b32_nrm, b32_inv) >> 3;
    result = skp_s_mla_w_w!(result, err_q32, b32_inv);
    let l_shift = 61 - b_head_rm - q_res;
    if l_shift <= 0 {
        skp_r_shift_sat_32!(result, -l_shift)
    } else if l_shift < 32 {
        result >> l_shift
    } else {
        0
    }
}
fn lpc_inverse_pred_gain_qa(inv_gain_q30: &mut i32, a_qa: &mut [[i32; 16]], order: usize) -> i32 {
    let mut a_new_qa_index = order & 1;
    *inv_gain_q30 = 1 << 30;
    let mut rc_q31;
    let mut rc_mul_t1_q30;
    for k in (1..order).rev() {
        if a_qa[a_new_qa_index][k] > (0.99975f64 * (1i64 << 16) as f64 + 0.5f64) as i32
            || a_qa[a_new_qa_index][k] < -((0.99975f64 * (1i64 << 16) as f64 + 0.5f64) as i32)
        {
            return 1;
        }
        rc_q31 = -(a_qa[a_new_qa_index][k] << 31 - 16);
        rc_mul_t1_q30 = (i32::MAX >> 1) - skp_s_m_mul!(rc_q31, rc_q31);
        let mut rc_mul_t2_q16 = skp_inverse32_var_q(rc_mul_t1_q30, 46);
        *inv_gain_q30 = skp_l_shift!(skp_s_m_mul!(*inv_gain_q30, rc_mul_t1_q30), 2);
        let a_old_qa_index = a_new_qa_index;
        a_new_qa_index = k & 1;
        let head_rm = skp_silk_clz32(rc_mul_t2_q16) - 1;
        rc_mul_t2_q16 = rc_mul_t2_q16 << head_rm;
        for n in 0..k {
            let tmp_qa = a_qa[a_old_qa_index][n]
                - skp_l_shift!(skp_s_m_mul!(a_qa[a_old_qa_index][k - n - 1], rc_q31), 1);
            a_qa[a_new_qa_index][n] =
                skp_l_shift!(skp_s_m_mul!(tmp_qa, rc_mul_t2_q16), 16 - head_rm);
        }
    }
    if a_qa[a_new_qa_index][0] > (0.99975f64 * (1i64 << 16) as f64 + 0.5f64) as i32
        || a_qa[a_new_qa_index][0] < -((0.99975f64 * (1i64 << 16) as f64 + 0.5f64) as i32)
    {
        return 1;
    }
    rc_q31 = -(a_qa[a_new_qa_index][0] << 31 - 16);
    rc_mul_t1_q30 = (i32::MAX >> 1) - skp_s_m_mul!(rc_q31, rc_q31);
    *inv_gain_q30 = skp_l_shift!(skp_s_m_mul!(*inv_gain_q30, rc_mul_t1_q30), 2);
    return 0;
}

pub fn skp_silk_lpc_inverse_pred_gain(inv_gain_q30: &mut i32, a_q12: &[i16], order: usize) -> i32 {
    let mut a_tmp_qa = [[0; 16]; 2];
    let a_new_qa = &mut a_tmp_qa[order & 1];
    for k in 0..order {
        a_new_qa[k] = (a_q12[k] as i32) << 16 - 12;
    }
    return lpc_inverse_pred_gain_qa(inv_gain_q30, &mut a_tmp_qa, order);
}

pub fn skp_silk_lpc_inverse_pred_gain_q24(
    inv_gain_q30: &mut i32,
    a_q24: &[i32],
    order: usize,
) -> i32 {
    let mut a_tmp_qa = [[0; 16]; 2];
    let a_new_qa_index = order & 1;
    for k in 0..order {
        a_tmp_qa[a_new_qa_index][k] = skp_r_shift_round!(a_q24[k], 24 - 16);
    }
    return lpc_inverse_pred_gain_qa(inv_gain_q30, &mut a_tmp_qa, order);
}
