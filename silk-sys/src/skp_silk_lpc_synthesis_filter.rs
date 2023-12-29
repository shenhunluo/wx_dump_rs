use crate::{
    skp_add_sat32, skp_l_shift_sat_32, skp_r_shift_round, skp_s_mla_w_b, skp_s_mla_w_t,
    skp_s_mul_b_b, skp_s_mul_w_b, skp_sat_16,
};
#[no_mangle]
pub fn skp_silk_lpc_synthesis_filter(
    in_0: &[i16],
    a_q12: &[i16],
    gain_q26: i32,
    s: &mut [i32],
    out: &mut [i16],
    len: usize,
    order: usize,
) {
    let order_half: usize = order >> 1;
    let mut a_align_q12: [libc::c_int; 8] = [0; 8];
    for k in 0..order_half {
        let idx = skp_s_mul_b_b!(2, k);
        a_align_q12[k] =
            a_q12[idx as usize] as i32 & 0xffff | (a_q12[idx as usize + 1] as i32) << 16;
    }
    for k in 0..len {
        let mut sa = s[order - 1];
        let mut out32_q10 = 0;
        for j in 0..order_half - 1 {
            let idx = skp_s_mul_b_b!(2, j) + 1;
            let a_tmp = a_align_q12[j as usize];
            let sb = s[order - 1 - idx as usize];
            s[order - 1 - idx as usize] = sa;

            out32_q10 = skp_s_mla_w_b!(out32_q10, sa, a_tmp);
            out32_q10 = skp_s_mla_w_t!(out32_q10, sb, a_tmp);

            sa = s[order - 2 - idx as usize];
            s[order - 2 - idx as usize] = sb;
        }
        let a_tmp = a_align_q12[order_half - 1];
        let sb = s[0];
        s[0] = sa;
        out32_q10 = skp_s_mla_w_b!(out32_q10, sa, a_tmp);
        out32_q10 = skp_s_mla_w_t!(out32_q10, sb, a_tmp);
        out32_q10 = skp_add_sat32!(out32_q10, skp_s_mul_w_b!(gain_q26, in_0[k]));
        let out32 = skp_r_shift_round!(out32_q10, 10);
        out[k] = skp_sat_16!(out32, i32) as i16;
        s[order - 1] = skp_l_shift_sat_32!(out32_q10, 4);
    }
}
