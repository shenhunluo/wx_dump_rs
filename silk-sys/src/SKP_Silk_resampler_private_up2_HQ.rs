use crate::{
    skp_l_shift, skp_r_shift, skp_s_mla_w_b, skp_s_mul_w_b, skp_sat_16,
    skp_silk_resampler_rom::{
        SKP_SILK_RESAMPLER_UP2_HQ_0, SKP_SILK_RESAMPLER_UP2_HQ_1, SKP_SILK_RESAMPLER_UP2_HQ_NOTCH,
    },
    skp_silk_resampler::SkpSilkResamplerStateStruct,
};

pub fn skp_silk_resampler_private_up2_hq(s: &mut [i32], out: &mut [i16], in_0: &[i16], len: usize) {
    for k in 0..len {
        let in32 = skp_l_shift!(in_0[k] as i32, 10);
        let mut y = in32 - s[0];
        let mut x = skp_s_mul_w_b!(y, SKP_SILK_RESAMPLER_UP2_HQ_0[0]);
        let mut out32_1 = s[0] + x;
        s[0] = in32 + x;
        y = out32_1 - s[1];
        x = skp_s_mla_w_b!(y, y, SKP_SILK_RESAMPLER_UP2_HQ_0[1]);
        let mut out32_2 = s[1] + x;
        s[1] = out32_1 + x;
        out32_2 = skp_s_mla_w_b!(out32_2, s[5], SKP_SILK_RESAMPLER_UP2_HQ_NOTCH[2]);
        out32_2 = skp_s_mla_w_b!(out32_2, s[4], SKP_SILK_RESAMPLER_UP2_HQ_NOTCH[1]);
        out32_1 = skp_s_mla_w_b!(out32_2, s[4], SKP_SILK_RESAMPLER_UP2_HQ_NOTCH[0]);
        s[5] = out32_2 - s[5];
        out[2 * k] = skp_sat_16!(
            skp_r_shift!(
                skp_s_mla_w_b!(256, out32_1, SKP_SILK_RESAMPLER_UP2_HQ_NOTCH[3]),
                9
            ),
            i32
        ) as i16;

        y = in32 - s[2];
        x = skp_s_mul_w_b!(y, SKP_SILK_RESAMPLER_UP2_HQ_1[0]);
        out32_1 = s[2] + x;
        s[2] = in32 + x;

        y = out32_1 - s[3];
        x = skp_s_mla_w_b!(y, y, SKP_SILK_RESAMPLER_UP2_HQ_1[1]);
        out32_2 = s[3] + x;
        s[3] = out32_1 + x;

        out32_2 = skp_s_mla_w_b!(out32_2, s[4], SKP_SILK_RESAMPLER_UP2_HQ_NOTCH[2]);
        out32_2 = skp_s_mla_w_b!(out32_2, s[5], SKP_SILK_RESAMPLER_UP2_HQ_NOTCH[1]);
        out32_1 = skp_s_mla_w_b!(out32_2, s[5], SKP_SILK_RESAMPLER_UP2_HQ_NOTCH[0]);
        s[4] = out32_2 - s[4];

        out[2 * k + 1] = skp_sat_16!(
            skp_r_shift!(
                skp_s_mla_w_b!(256, out32_1, SKP_SILK_RESAMPLER_UP2_HQ_NOTCH[3]),
                9
            ),
            i32
        ) as i16;
    }
}

pub fn skp_silk_resampler_private_up2_hq_wrapper(
    ss: &mut SkpSilkResamplerStateStruct,
    out: &mut [i16],
    in_0: &[i16],
    len: usize,
) {
    skp_silk_resampler_private_up2_hq(&mut ss.s_iir, out, in_0, len);
}
