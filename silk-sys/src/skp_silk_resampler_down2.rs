use crate::{
    skp_l_shift, skp_r_shift_round, skp_s_mla_w_b, skp_s_mul_w_b, skp_sat_16,
    skp_silk_resampler_rom::{SKP_SILK_RESAMPLER_DOWN2_0, SKP_SILK_RESAMPLER_DOWN2_1},
};

pub fn skp_silk_resampler_down2(s: &mut [i32], out: &mut [i16], in_0: &[i16], in_len: usize) {
    let len2 = in_len >> 1;
    for k in 0..len2 {
        let mut in32 = skp_l_shift!(in_0[2 * k] as i32, 10);
        let y = in32 - s[0];
        let x = skp_s_mla_w_b!(y, y, SKP_SILK_RESAMPLER_DOWN2_1);
        let mut out32 = s[0] + x;
        s[0] = in32 + x;
        in32 = skp_l_shift!(in_0[2 * k + 1] as i32, 10);
        let y = in32 - s[1];
        let x = skp_s_mul_w_b!(y, SKP_SILK_RESAMPLER_DOWN2_0);
        out32 = out32 + s[1];
        out32 = out32 + x;
        s[1] = in32 + x;

        out[k] = skp_sat_16!(skp_r_shift_round!(out32, 11), i32) as i16;
    }
}
