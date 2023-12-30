use crate::{skp_add_l_shift, skp_l_shift, skp_s_mla_w_b};

pub fn skp_silk_resampler_private_ar2(
    s: &mut [i32],
    out_q8: &mut [i32],
    in_0: &[i16],
    a_q14: &[i16],
    len: usize,
) {
    for k in 0..len {
        let out32 = skp_add_l_shift!(s[0], in_0[k] as i32, 8);
        out_q8[k] = out32;
        let out32 = skp_l_shift!(out32, 2);
        s[0] = skp_s_mla_w_b!(s[1], out32, a_q14[0]);
        s[1] = skp_s_mla_w_b!(out32, a_q14[1]);
    }
}
