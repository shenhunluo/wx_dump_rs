use crate::{
    skp_s_mul_b_b,
    skp_silk_tables_other::SKP_SILK_LSB_CDF,
    skp_silk_tables_pulses_per_block::{
        SKP_SILK_PULSES_PER_BLOCK_CDF, SKP_SILK_PULSES_PER_BLOCK_CDF_OFFSET,
        SKP_SILK_RATE_LEVELS_CDF, SKP_SILK_RATE_LEVELS_CDF_OFFSET,
    },
    SKP_Silk_code_signs::SKP_Silk_decode_signs,
    SKP_Silk_dec_API::SKP_Silk_decoder_control,
    SKP_Silk_range_coder::{SKP_Silk_range_coder_state, SKP_Silk_range_decoder},
    SKP_Silk_shell_coder::SKP_Silk_shell_decoder,
};
#[no_mangle]
pub fn skp_silk_decode_pulses(
    ps_r_c: &mut SKP_Silk_range_coder_state,
    ps_dec_ctrl: &mut SKP_Silk_decoder_control,
    q: &mut [i32],
    frame_length: i32,
) {
    let mut sum_pulses = [0; 30];
    let mut n_l_shifts = [0; 30];
    ps_dec_ctrl.RateLevelIndex = SKP_Silk_range_decoder(
        ps_r_c,
        &SKP_SILK_RATE_LEVELS_CDF[ps_dec_ctrl.sig_type as usize],
        SKP_SILK_RATE_LEVELS_CDF_OFFSET,
    );
    let iter = frame_length as usize / 16;
    let cdf_ptr = &SKP_SILK_PULSES_PER_BLOCK_CDF[ps_dec_ctrl.RateLevelIndex as usize];
    for i in 0..iter {
        n_l_shifts[i] = 0;
        sum_pulses[i] = SKP_Silk_range_decoder(ps_r_c, cdf_ptr, SKP_SILK_PULSES_PER_BLOCK_CDF_OFFSET);
        while sum_pulses[i] == 18 + 1 {
            n_l_shifts[i] += 1;
            sum_pulses[i] = SKP_Silk_range_decoder(
                ps_r_c,
                &SKP_SILK_PULSES_PER_BLOCK_CDF[10 - 1],
                SKP_SILK_PULSES_PER_BLOCK_CDF_OFFSET,
            );
        }
    }

    for i in 0..iter {
        if sum_pulses[i] > 0 {
            SKP_Silk_shell_decoder(
                &mut q[skp_s_mul_b_b!(i, 16) as usize..],
                ps_r_c,
                sum_pulses[i],
            );
        } else {
            for i in &mut q[skp_s_mul_b_b!(i, 16) as usize..(skp_s_mul_b_b!(i, 16) as usize + 16)] {
                *i = 0;
            }
        }
    }

    for i in 0..iter {
        if n_l_shifts[i] > 0 {
            let pulses_ptr = &mut q[skp_s_mul_b_b!(i, 16) as usize..];
            for k in 0..16 {
                let mut abs_q = pulses_ptr[k];
                for _ in 0..n_l_shifts[i] {
                    abs_q = abs_q << 1;
                    abs_q += SKP_Silk_range_decoder(ps_r_c, &SKP_SILK_LSB_CDF, 1);
                }
                pulses_ptr[k] = abs_q;
            }
        }
    }
    SKP_Silk_decode_signs(
        ps_r_c,
        q,
        frame_length,
        ps_dec_ctrl.sig_type,
        ps_dec_ctrl.QuantOffsetType,
        ps_dec_ctrl.RateLevelIndex,
    );
}
