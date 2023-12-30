use crate::{
    skp_r_shift_round, skp_s_mla_w_b, skp_s_mul_w_b, skp_sat_16,
    skp_silk_resampler_rom::{SKP_SILK_RESAMPLER_DOWN2_0, SKP_SILK_RESAMPLER_DOWN2_1},
};

pub fn skp_silk_resampler_private_down4(
    s: &mut [i32],
    out: &mut [i16],
    in_0: &[i16],
    in_len: usize,
) {
    let len4 = in_len >> 2;
    for k in 0..len4 {
        let in32 = (in_0[4 * k] as i32 + in_0[4 * k + 1] as i32) << 9;
        let y = in32 - s[0];
        let x = skp_s_mla_w_b!(y, y, SKP_SILK_RESAMPLER_DOWN2_1);
        let mut out32 = s[0] + x;
        s[0] = in32 + x;
        let in32 = (in_0[4 * k + 2] as i32 + in_0[4 * k + 3] as i32) << 9;
        let y = in32 - s[1];
        let x = skp_s_mul_w_b!(y, SKP_SILK_RESAMPLER_DOWN2_0);
        out32 = out32 + s[1];
        out32 = out32 + x;
        s[1] = in32 + x;
        out[k] = skp_sat_16!(skp_r_shift_round!(out32, 11), i32) as i16;
    }
}
