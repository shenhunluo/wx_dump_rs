use crate::{skp_l_shift, skp_r_shift_round, skp_s_mla_b_b, skp_s_mul_w_b, skp_sat_16};
pub fn skp_silk_biquad(
    in_0: &[i16],
    b: &[i16],
    a: &[i16],
    s: &mut [i32],
    out: &mut [i16],
    len: usize,
) {
    let mut s0 = s[0];
    let mut s1 = s[1];
    let a0_neg = -a[0] as i32;
    let a1_neg = -a[1] as i32;
    for k in 0..len {
        let in16 = in_0[k] as i32;
        let out32 = skp_s_mla_b_b!(s0, in16, b[0]);

        s0 = skp_s_mla_b_b!(s1, in16, b[1]);
        s0 += skp_l_shift!(skp_s_mul_w_b!(out32, a0_neg), 3);

        s1 = skp_l_shift!(skp_s_mul_w_b!(out32, a1_neg), 3);
        s1 = skp_s_mla_b_b!(s1, in16, b[2]);
        let tmp32 = skp_r_shift_round!(out32, 13) + 1;
        out[k] = skp_sat_16!(tmp32, i32) as i16;
    }
    s[0] = s0;
    s[1] = s1;
}
