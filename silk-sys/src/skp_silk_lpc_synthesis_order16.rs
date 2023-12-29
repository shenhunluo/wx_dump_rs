use crate::{skp_s_mul_w_b, skp_s_mla_w_t_ovflw, skp_s_mla_w_b, skp_add_sat32, skp_r_shift_round, skp_sat_16, skp_l_shift_sat_32};

pub fn skp_silk_lpc_synthesis_order16(
    in_0: &[i16],
    a_q12: &[i16],
    gain_q26: i32,
    s: &mut [i32],
    out: &mut [i16],
    len: usize,
) {
    let mut a_align_q12 = [0; 8];
    for k in 0..8 {
        a_align_q12[k] = a_q12[2 * k] as i32 & 0xffff | (a_q12[2 * k + 1] as i32) << 16;
    }
    for k in 0..len {
        let mut sa = s[ 15 ];
        let mut a_tmp = a_align_q12[ 0 ];
        let mut sb = s[ 14 ];
        s[ 14 ] = sa;
        let mut out32_q10 = skp_s_mul_w_b!(                  sa, a_tmp );
        out32_q10 = skp_s_mla_w_t_ovflw!( out32_q10, sb, a_tmp );
        sa = s[ 13 ];
        s[ 13 ] = sb;
        a_tmp = a_align_q12[ 1 ];
        sb = s[ 12 ];
        s[ 12 ] = sa;
        out32_q10 = skp_s_mla_w_b!( out32_q10, sa, a_tmp );
        out32_q10 = skp_s_mla_w_t_ovflw!( out32_q10, sb, a_tmp );
        sa = s[ 11 ];
        s[ 11 ] = sb;
        a_tmp = a_align_q12[ 2 ];
        sb = s[ 10 ];
        s[ 10 ] = sa;
        out32_q10 = skp_s_mla_w_b!( out32_q10, sa, a_tmp );
        out32_q10 = skp_s_mla_w_t_ovflw!( out32_q10, sb, a_tmp );
        sa = s[ 9 ];
        s[ 9 ] = sb;
        a_tmp = a_align_q12[ 3 ];
        sb = s[ 8 ];
        s[ 8 ] = sa;
        out32_q10 = skp_s_mla_w_b!( out32_q10, sa, a_tmp );
        out32_q10 = skp_s_mla_w_t_ovflw!( out32_q10, sb, a_tmp );
        sa = s[ 7 ];
        s[ 7 ] = sb;
        a_tmp = a_align_q12[ 4 ];
        sb = s[ 6 ];
        s[ 6 ] = sa;
        out32_q10 = skp_s_mla_w_b!( out32_q10, sa, a_tmp );
        out32_q10 = skp_s_mla_w_t_ovflw!( out32_q10, sb, a_tmp );
        sa = s[ 5 ];
        s[ 5 ] = sb;
        a_tmp = a_align_q12[ 5 ];
        sb = s[ 4 ];
        s[ 4 ] = sa;
        out32_q10 = skp_s_mla_w_b!( out32_q10, sa, a_tmp );
        out32_q10 = skp_s_mla_w_t_ovflw!( out32_q10, sb, a_tmp );
        sa = s[ 3 ];
        s[ 3 ] = sb;
        a_tmp = a_align_q12[ 6 ];
        sb = s[ 2 ];
        s[ 2 ] = sa;
        out32_q10 = skp_s_mla_w_b!( out32_q10, sa, a_tmp );
        out32_q10 = skp_s_mla_w_t_ovflw!( out32_q10, sb, a_tmp );
        sa = s[ 1 ];
        s[ 1 ] = sb;
        a_tmp = a_align_q12[ 7 ];
        sb = s[ 0 ];
        s[ 0 ] = sa;
        out32_q10 = skp_s_mla_w_b!( out32_q10, sa, a_tmp );
        out32_q10 = skp_s_mla_w_t_ovflw!( out32_q10, sb, a_tmp );
        out32_q10 = skp_add_sat32!( out32_q10, skp_s_mul_w_b!( gain_q26, in_0[k] ) );
        let out32 = skp_r_shift_round!( out32_q10, 10 );
        out[k] = skp_sat_16!(out32, i32) as i16;
        s[ 15 ] = skp_l_shift_sat_32!( out32_q10, 4 );
    }
}
