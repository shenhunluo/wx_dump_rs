use crate::{skp_add_r_shift, skp_l_shift, skp_mla, skp_r_shift, skp_s_mla_w_b};

pub fn skp_silk_log2lin(in_log_q7: i32) -> i32 {
    if in_log_q7 < 0 {
        return 0;
    } else if in_log_q7 >= 31 << 7 {
        return 0x7fffffff;
    }
    let mut out = skp_l_shift!(1, skp_r_shift!(in_log_q7, 7));
    let frac_q7 = in_log_q7 & 0x7f;
    if in_log_q7 < 2048 {
        out = skp_add_r_shift!(
            out,
            out * skp_s_mla_w_b!(frac_q7, frac_q7 * (128 - frac_q7), -174),
            7
        );
    } else {
        out = skp_mla!(
            out,
            skp_r_shift!(out, 7),
            skp_s_mla_w_b!(frac_q7, frac_q7 * (128 - frac_q7), -174)
        );
    }
    return out;
}
