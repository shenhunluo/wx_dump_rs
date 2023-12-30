use crate::{
    i16_to_i32, i32_to_i16, skp_r_shift_round, skp_s_mla_b_b, skp_s_mul_w_b, skp_sat_16,
    skp_silk_resampler_private_arma4::skp_silk_resampler_private_arma4,
    skp_silk_resampler_rom::SKP_SILK_RESAMPLER_FRAC_FIR_144,
    SKP_Silk_resampler::SKP_Silk_resampler_state_struct,
};

fn skp_silk_resampler_private_iir_fir_interpol<'a>(
    mut out: &'a mut [i16],
    buf: &mut [i16],
    max_index_q16: i32,
    index_increment_q16: i32,
) -> &'a mut [i16] {
    for index_q16 in (0..max_index_q16).step_by(index_increment_q16 as usize) {
        let table_index = skp_s_mul_w_b!(index_q16 & 0xFFFF, 144) as usize;
        let buf_ptr = &buf[(index_q16 >> 16) as usize..];

        let mut res_q15 = skp_s_mul_w_b!(
            buf_ptr[0] as i32,
            SKP_SILK_RESAMPLER_FRAC_FIR_144[table_index][0] as i32
        );
        res_q15 = skp_s_mla_b_b!(
            res_q15,
            buf_ptr[1],
            SKP_SILK_RESAMPLER_FRAC_FIR_144[table_index][1]
        );
        res_q15 = skp_s_mla_b_b!(
            res_q15,
            buf_ptr[2],
            SKP_SILK_RESAMPLER_FRAC_FIR_144[table_index][2]
        );
        res_q15 = skp_s_mla_b_b!(
            res_q15,
            buf_ptr[3],
            SKP_SILK_RESAMPLER_FRAC_FIR_144[143 - table_index][2]
        );
        res_q15 = skp_s_mla_b_b!(
            res_q15,
            buf_ptr[4],
            SKP_SILK_RESAMPLER_FRAC_FIR_144[143 - table_index][1]
        );
        res_q15 = skp_s_mla_b_b!(
            res_q15,
            buf_ptr[5],
            SKP_SILK_RESAMPLER_FRAC_FIR_144[143 - table_index][0]
        );
        out[0] = skp_sat_16!(skp_r_shift_round!(res_q15, 15), i32) as i16;
        out = &mut out[1..];
    }
    return out;
}

pub fn skp_silk_resampler_private_iir_fir(
    ss: &mut SKP_Silk_resampler_state_struct,
    mut out: &mut [i16],
    mut in_0: &[i16],
    mut in_len: usize,
) {
    let mut buf = [0; 966];
    for i in 0..6 {
        (buf[2 * i], buf[2 * i + 1]) = i32_to_i16!(ss.sFIR[i]);
    }
    let index_increment_q16 = ss.invRatio_Q16;
    let mut n_samples_in;
    loop {
        n_samples_in = if in_len < ss.batchSize as usize {
            in_len
        } else {
            ss.batchSize as usize
        };
        if ss.input2x == 1 {
            (ss.up2_function).expect("non-null function pointer")(
                &mut ss.sIIR,
                &mut buf[6..],
                in_0,
                n_samples_in,
            );
        } else {
            skp_silk_resampler_private_arma4(
                &mut ss.sIIR,
                &mut buf[6..],
                &in_0,
                &ss.Coefs.unwrap(),
                n_samples_in,
            );
        }
        let max_index_q16 = (n_samples_in as i32) << 16 + ss.input2x;
        out = skp_silk_resampler_private_iir_fir_interpol(
            out,
            &mut buf,
            max_index_q16,
            index_increment_q16,
        );
        in_0 = &in_0[n_samples_in..];
        if in_len <= n_samples_in {
            break;
        }
        in_len -= n_samples_in;
        for i in 0..12 {
            buf[i] = buf[(n_samples_in << ss.input2x) + i];
        }
    }
    for i in 0..6 {
        ss.sFIR[i] = i16_to_i32!(
            buf[(n_samples_in << ss.input2x) + 2 * i],
            buf[(n_samples_in << ss.input2x) + 2 * i + 1]
        );
    }
}
