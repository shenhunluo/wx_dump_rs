use crate::{skp_add_l_shift, skp_l_shift, skp_r_shift, skp_s_mla_w_b, skp_sat_16};

pub fn skp_silk_resampler_private_arma4(
    s: &mut [i32],
    out: &mut [i16],
    in_0: &[i16],
    coef: &[i16],
    len: usize,
) {
    for k in 0..len {
        let in_q8 = skp_l_shift!(in_0[k] as i32, 8);
        let out1_q8 = skp_add_l_shift!(in_q8, s[0], 2);
        let out2_q8 = skp_add_l_shift!(out1_q8, s[2], 2);

        let x = skp_s_mla_w_b!(s[1], in_q8, coef[0]);
        s[0] = skp_s_mla_w_b!(x, out1_q8, coef[2]);

        let x = skp_s_mla_w_b!(s[3], out1_q8, coef[1]);
        s[2] = skp_s_mla_w_b!(x, out2_q8, coef[4]);

        s[1] = skp_s_mla_w_b!(skp_r_shift!(in_q8, 2), out1_q8, coef[3]);
        s[3] = skp_s_mla_w_b!(skp_r_shift!(out1_q8, 2), out2_q8, coef[5]);

        out[k] = skp_sat_16!(skp_r_shift!(skp_s_mla_w_b!(128, out2_q8, coef[6]), 8), i32) as i16;
    }
}
