#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]

use crate::{skp_s_mul_w_b, skp_s_mla_w_w, skp_r_shift_sat_32};
pub type __int64_t = libc::c_longlong;
pub type int64_t = __int64_t;

fn SKP_Silk_CLZ16(mut in16: i16) -> i32 {
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

fn SKP_Silk_CLZ32(mut in32: i32) -> i32 {
    if in32 as u32 & 0xffff0000 != 0 {
        SKP_Silk_CLZ16((in32 >> 16) as i16)
    } else {
        SKP_Silk_CLZ16(in32 as i16) + 16 as i32
    }
}

fn SKP_INVERSE32_varQ(
    b32: i32,
    q_res: i32,
) -> i32 {
    let b_head_rm = SKP_Silk_CLZ32(b32.abs()) - 1;
    let b32_nrm = b32 << b_head_rm;
    let b32_inv = (i32::MAX >> 2) / (b32_nrm >> 16);
    let mut result = b32_inv << 16;
    let err_Q32 = -skp_s_mul_w_b!(b32_nrm, b32_inv) >> 3;
    result = skp_s_mla_w_w!(result, err_Q32, b32_inv);
    let l_shift = 61 - b_head_rm - q_res;
    if l_shift <= 0 {
        skp_r_shift_sat_32!(result, -l_shift)
    } else if l_shift < 32 {
        result >> l_shift
    } else {
        0
    }
}
unsafe extern "C" fn LPC_inverse_pred_gain_QA(
    mut invGain_Q30: *mut libc::c_int,
    mut A_QA: *mut [libc::c_int; 16],
    order: libc::c_int,
) -> libc::c_int {
    let mut k: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut headrm: libc::c_int = 0;
    let mut rc_Q31: libc::c_int = 0;
    let mut rc_mult1_Q30: libc::c_int = 0;
    let mut rc_mult2_Q16: libc::c_int = 0;
    let mut tmp_QA: libc::c_int = 0;
    let mut Aold_QA: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut Anew_QA: *mut libc::c_int = 0 as *mut libc::c_int;
    Anew_QA = (*A_QA.offset((order & 1 as libc::c_int) as isize)).as_mut_ptr();
    *invGain_Q30 = (1 as libc::c_int) << 30 as libc::c_int;
    k = order - 1 as libc::c_int;
    while k > 0 as libc::c_int {
        if *Anew_QA.offset(k as isize)
            > (0.99975f64
                * ((1 as libc::c_int as int64_t) << 16 as libc::c_int) as libc::c_double
                + 0.5f64) as libc::c_int
            || *Anew_QA.offset(k as isize)
                < -((0.99975f64
                    * ((1 as libc::c_int as int64_t) << 16 as libc::c_int)
                        as libc::c_double + 0.5f64) as libc::c_int)
        {
            return 1 as libc::c_int;
        }
        rc_Q31 = -(*Anew_QA.offset(k as isize) << 31 as libc::c_int - 16 as libc::c_int);
        rc_mult1_Q30 = (0x7fffffff as libc::c_int >> 1 as libc::c_int)
            - ((rc_Q31 as int64_t * rc_Q31 as int64_t).wrapping_shr(32) as libc::c_int)
                as libc::c_int;
        rc_mult2_Q16 = SKP_INVERSE32_varQ(rc_mult1_Q30, 46 as libc::c_int);
        *invGain_Q30 = (((*invGain_Q30 as int64_t * rc_mult1_Q30 as int64_t).wrapping_shr(32) as libc::c_int) as libc::c_int) << 2 as libc::c_int;
        Aold_QA = Anew_QA;
        Anew_QA = (*A_QA.offset((k & 1 as libc::c_int) as isize)).as_mut_ptr();
        headrm = SKP_Silk_CLZ32(rc_mult2_Q16) - 1 as libc::c_int;
        rc_mult2_Q16 = rc_mult2_Q16 << headrm;
        n = 0 as libc::c_int;
        while n < k {
            tmp_QA = *Aold_QA.offset(n as isize)
                - ((((*Aold_QA.offset((k - n - 1 as libc::c_int) as isize) as int64_t
                    * rc_Q31 as int64_t).wrapping_shr(32) as libc::c_int) as libc::c_int)
                    << 1 as libc::c_int);
            *Anew_QA
                .offset(
                    n as isize,
                ) = (((tmp_QA as int64_t * rc_mult2_Q16 as int64_t).wrapping_shr(32) as libc::c_int)
                as libc::c_int) << 16 as libc::c_int - headrm;
            n += 1;
        }
        k -= 1;
    }
    if *Anew_QA.offset(0 as libc::c_int as isize)
        > (0.99975f64
            * ((1 as libc::c_int as int64_t) << 16 as libc::c_int) as libc::c_double
            + 0.5f64) as libc::c_int
        || *Anew_QA.offset(0 as libc::c_int as isize)
            < -((0.99975f64
                * ((1 as libc::c_int as int64_t) << 16 as libc::c_int) as libc::c_double
                + 0.5f64) as libc::c_int)
    {
        return 1 as libc::c_int;
    }
    rc_Q31 = -(*Anew_QA.offset(0 as libc::c_int as isize)
        << 31 as libc::c_int - 16 as libc::c_int);
    rc_mult1_Q30 = (0x7fffffff as libc::c_int >> 1 as libc::c_int)
        - ((rc_Q31 as int64_t * rc_Q31 as int64_t).wrapping_shr(32) as libc::c_int) as libc::c_int;
    *invGain_Q30 = (((*invGain_Q30 as int64_t * rc_mult1_Q30 as int64_t).wrapping_shr(32)
        as libc::c_int) as libc::c_int) << 2 as libc::c_int;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_LPC_inverse_pred_gain(
    invGain_Q30: &mut i32,
    A_Q12: &[i16],
    order: libc::c_int,
) -> libc::c_int {
    let mut k: libc::c_int = 0;
    let mut Atmp_QA: [[libc::c_int; 16]; 2] = [[0; 16]; 2];
    let mut Anew_QA: *mut libc::c_int = 0 as *mut libc::c_int;
    Anew_QA = (Atmp_QA[(order & 1 as libc::c_int) as usize]).as_mut_ptr();
    k = 0 as libc::c_int;
    while k < order {
        *Anew_QA
            .offset(
                k as isize,
            ) = (A_Q12[k as usize] as libc::c_int)
            << 16 as libc::c_int - 12 as libc::c_int;
        k += 1;
    }
    return LPC_inverse_pred_gain_QA(invGain_Q30, Atmp_QA.as_mut_ptr(), order);
}
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_LPC_inverse_pred_gain_Q24(
    mut invGain_Q30: *mut libc::c_int,
    mut A_Q24: *const libc::c_int,
    order: libc::c_int,
) -> libc::c_int {
    let mut k: libc::c_int = 0;
    let mut Atmp_QA: [[libc::c_int; 16]; 2] = [[0; 16]; 2];
    let mut Anew_QA: *mut libc::c_int = 0 as *mut libc::c_int;
    Anew_QA = (Atmp_QA[(order & 1 as libc::c_int) as usize]).as_mut_ptr();
    k = 0 as libc::c_int;
    while k < order {
        *Anew_QA
            .offset(
                k as isize,
            ) = if 24 as libc::c_int - 16 as libc::c_int == 1 as libc::c_int {
            (*A_Q24.offset(k as isize) >> 1 as libc::c_int)
                + (*A_Q24.offset(k as isize) & 1 as libc::c_int)
        } else {
            (*A_Q24.offset(k as isize)
                >> 24 as libc::c_int - 16 as libc::c_int - 1 as libc::c_int)
                + 1 as libc::c_int >> 1 as libc::c_int
        };
        k += 1;
    }
    return LPC_inverse_pred_gain_QA(invGain_Q30, Atmp_QA.as_mut_ptr(), order);
}
