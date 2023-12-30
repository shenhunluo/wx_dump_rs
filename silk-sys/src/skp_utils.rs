use crate::{
    skp_l_shift, skp_l_shift_sat_32, skp_s_m_mul, skp_s_mla_w_b, skp_s_mla_w_w, skp_s_mul_w_b, skp_s_mul_b_b,
};

pub const MAX_LPC_ORDER: usize = 16;
pub const NB_SUBFR: usize = 4;
pub const LTP_ORDER: usize = 5;
pub const V_PITCH_GAIN_START_MIN_Q14: i32 = 11469;
pub const CNG_NLSF_SMTH_Q16: i32 = 16348;
pub const CNG_GAIN_SMTH_Q16: i32 = 4634;
pub const RESAMPLER_DOWN_ORDER_FIR: i32 = 12;

fn skp_silk_clz16(mut in16: i16) -> i32 {
    let mut out32 = 0;
    if in16 == 0 {
        return 16;
    }
    if in16 as i32 & 0xff00 != 0 {
        if in16 as i32 & 0xf000 != 0 {
            in16 = in16 >> 12;
        } else {
            out32 += 4;
            in16 = in16 >> 8;
        }
    } else if in16 as i32 & 0xfff0 != 0 {
        out32 += 8;
        in16 = in16 >> 4;
    } else {
        out32 += 12;
    }
    if in16 & 0xc != 0 {
        if in16 & 0x8 != 0 {
            out32 + 0
        } else {
            out32 + 1
        }
    } else if in16 & 0xe != 0 {
        out32 + 2
    } else {
        out32 + 3
    }
}

pub fn skp_silk_clz32(in32: i32) -> i32 {
    if in32 as u32 & 0xffff0000 != 0 {
        skp_silk_clz16((in32 >> 16) as i16)
    } else {
        skp_silk_clz16(in32 as i16) + 16 as i32
    }
}

pub fn skp_inverse32_var_q(b32: i32, q_res: i32) -> i32 {
    let b_head_rm = skp_silk_clz32(b32.abs()) - 1;
    let b32_nrm = b32 << b_head_rm;
    let b32_inv = (i32::MAX >> 2) / (b32_nrm >> 16);
    let mut result = b32_inv << 16;
    let err_q32 = -skp_s_mul_w_b!(b32_nrm, b32_inv) >> 3;
    result = skp_s_mla_w_w!(result, err_q32, b32_inv);
    let l_shift = 61 - b_head_rm - q_res;
    if l_shift <= 0 {
        skp_l_shift_sat_32!(result, -l_shift)
    } else if l_shift < 32 {
        result >> l_shift
    } else {
        0
    }
}

pub fn skp_div32_var_q(a32: i32, b32: i32, q_res: i32) -> i32 {
    let a_head_rm = skp_silk_clz32(a32.abs()) - 1;
    let mut a32_nrm = a32 << a_head_rm;
    let b_head_rm = skp_silk_clz32(b32.abs()) - 1;
    let b32_nrm = b32 << b_head_rm;
    let b32_inv = (i32::MAX >> 2) / (b32_nrm >> 16);
    let mut result = skp_s_mul_w_b!(a32_nrm, b32_inv);
    a32_nrm -= skp_l_shift!(skp_s_m_mul!(b32_nrm, result), 3);
    result = skp_s_mla_w_b!(result, a32_nrm, b32_inv);

    let l_shift = 29 + a_head_rm - b_head_rm - q_res;
    if l_shift <= 0 {
        skp_l_shift_sat_32!(result, -l_shift)
    } else if l_shift < 32 {
        result >> l_shift
    } else {
        0
    }
}


pub fn skp_ror32(
    a32: i32,
    rot: i32,
) -> i32 {
    let mut x = a32;
    let mut r = rot as u32;
    let mut m = -rot as u32;
    if rot <= 0 {
        (x << m | x >> (32 as u32).wrapping_sub(m)) as i32
    } else {
        (x << (32 as u32).wrapping_sub(r) | x >> r) as i32
    }
}

pub fn skp_silk_clz_frac(
    in_0: i32,
    lz: &mut i32,
    frac_q7: &mut i32
) {
    let l_zeros = skp_silk_clz32(in_0);
    *lz = l_zeros;
    *frac_q7 = skp_ror32(in_0, 24 - l_zeros) & 0x7f;
}

pub fn skp_silk_sqrt_approx(x: i32) -> i32 {
    let mut lz = 0;
    let mut frac_q7 = 0;
    if x <= 0 {
        0
    } else {
        skp_silk_clz_frac(x, &mut lz, &mut frac_q7);
        let mut y = if lz & 1 != 0 {
            32768
        } else {
            46214
        };
        y >>= lz >> 1;
        skp_s_mla_w_b!(y, y, skp_s_mul_b_b!(213, frac_q7))
    }
}
