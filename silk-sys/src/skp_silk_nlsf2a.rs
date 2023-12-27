use crate::{
    skp_l_shift, skp_r_shift_round, skp_s_mul_l, skp_sat_16,
    skp_silk_bwexpander_32::skp_silk_bwexpander_32,
    skp_silk_lsf_cos_table::SKP_SILK_LSF_COS_TAB_FIX_Q12,
};

fn skp_silk_nlsf2a_find_poly(out: &mut [i32], c_lsf: &[i32], dd: usize) {
    out[0] = skp_l_shift!(1, 20);
    out[1] = -c_lsf[0];
    for k in 1..dd {
        let f_tmp = c_lsf[2 * k];
        out[k + 1] = skp_l_shift!(out[k - 1], 1)
            - skp_r_shift_round!(skp_s_mul_l!(f_tmp, out[k]), 20) as i32;
        for n in (2..=k).rev() {
            out[n] += out[n - 2] - skp_r_shift_round!(skp_s_mul_l!(f_tmp, out[n - 1]), 20) as i32;
        }
        out[1] -= f_tmp;
    }
}

pub fn skp_silk_nlsf2a(a: &mut [i16], nlsf: &[i32], d: usize) {
    let mut cos_lsf_q20 = [0; 16];
    let mut p = [0; 9];
    let mut q = [0; 9];
    for k in 0..d {
        let f_int = nlsf[k] >> 15 - 7;
        let f_frac = nlsf[k] - (f_int << 15 - 7);
        let cos_val = SKP_SILK_LSF_COS_TAB_FIX_Q12[f_int as usize];
        let delta = SKP_SILK_LSF_COS_TAB_FIX_Q12[f_int as usize + 1] - cos_val;
        cos_lsf_q20[k] = (cos_val << 8) + delta * f_frac;
    }
    let dd = d >> 1;
    skp_silk_nlsf2a_find_poly(&mut p, &cos_lsf_q20[0..], dd);
    skp_silk_nlsf2a_find_poly(&mut q, &cos_lsf_q20[1..], dd);

    let mut a_int32 = [0; 16];
    for k in 0..dd {
        let p_tmp = p[k + 1] + p[k];
        let q_tmp = q[k + 1] - q[k];
        a_int32[k] = -skp_r_shift_round!(p_tmp + q_tmp, 9);
        a_int32[d - k - 1] = skp_r_shift_round!(q_tmp - p_tmp, 9);
    }
    let mut i = 0;
    while i < 10 {
        let mut max_abs = 0;
        let mut idx = 0;
        for k in 0..d {
            let abs_val = a_int32[k].abs();
            if abs_val > max_abs {
                max_abs = abs_val;
                idx = k as i32;
            }
        }
        if !(max_abs > 0x7fff) {
            break;
        }
        max_abs = max_abs.min(98369);
        let sc_q16 = 65470 - (65470 >> 2) * (max_abs - 0x7fff) / (max_abs * (idx + 1) >> 2);
        skp_silk_bwexpander_32(&mut a_int32, d, sc_q16);
        i += 1;
    }

    if i == 10 {
        for k in 0..d {
            a_int32[k] = skp_sat_16!(a_int32[k], i32);
        }
    }
    for k in 0..d {
        a[k] = a_int32[k] as i16;
    }
}
