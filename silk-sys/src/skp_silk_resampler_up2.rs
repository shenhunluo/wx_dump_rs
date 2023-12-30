use crate::{
    skp_l_shift, skp_r_shift_round, skp_s_mla_w_b, skp_s_mul_w_b, skp_sat_16,
    skp_silk_resampler_rom::{SKP_SILK_RESAMPLER_UP2_LQ_0, SKP_SILK_RESAMPLER_UP2_LQ_1},
};

pub fn skp_silk_resampler_up2(s: &mut [i32], out: &mut [i16], in_0: &[i16], len: usize) {
    for k in 0..len {
        let in32 = skp_l_shift!(in_0[k] as i32, 10);
        let mut y = in32 - s[0];
        let mut x = skp_s_mul_w_b!(y, SKP_SILK_RESAMPLER_UP2_LQ_0);
        let mut out32 = s[0] + x;
        s[0] = in32 + x;

        out[2 * k] = skp_sat_16!(skp_r_shift_round!(out32, 10), i32) as i16;

        y = in32 - s[1];
        x = skp_s_mla_w_b!(y, y, SKP_SILK_RESAMPLER_UP2_LQ_1);
        out32 = s[1] + x;
        s[1] = in32 + x;

        out[2 * k + 1] = skp_sat_16!(skp_r_shift_round!(out32, 10), i32) as i16;
    }
}
