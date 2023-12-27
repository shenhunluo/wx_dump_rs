#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]

use crate::{skp_s_mul_w_b, skp_s_mla_w_w, skp_r_shift_sat_32, skp_s_m_mul, skp_l_shift};
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
fn LPC_inverse_pred_gain_QA(
    mut inv_gain_q30: &mut i32,
    mut a_qa: &mut [[i32; 16]],
    order: usize,
) -> i32 {
    let mut a_new_qa_index = order & 1;
    *inv_gain_q30 = 1 << 30;
    let mut rc_q31 = 0;
    let mut rc_mul_t1_q30 = 0;
    for k in (1..order).rev() {
        if  a_qa[a_new_qa_index][k] > (0.99975f64 * (1i64 << 16) as f64 + 0.5f64) as i32 ||
            a_qa[a_new_qa_index][k] < -((0.99975f64 * (1i64 << 16) as f64 + 0.5f64) as i32)
        {
            return 1;
        }
        rc_q31 = -(a_qa[a_new_qa_index][k] << 31 - 16);
        rc_mul_t1_q30 = (i32::MAX >> 1) - skp_s_m_mul!(rc_q31,rc_q31);
        let mut rc_mul_t2_q16 = SKP_INVERSE32_varQ(rc_mul_t1_q30, 46);
        *inv_gain_q30 = skp_l_shift!( skp_s_m_mul!( *inv_gain_q30, rc_mul_t1_q30 ), 2 );
        let a_old_qa_index = a_new_qa_index;
        a_new_qa_index = k & 1;
        let head_rm = SKP_Silk_CLZ32(rc_mul_t2_q16) - 1;
        rc_mul_t2_q16 = rc_mul_t2_q16 << head_rm;
        for n in 0..k {
            let tmp_qa = a_qa[a_old_qa_index][n] - skp_l_shift!( skp_s_m_mul!( a_qa[a_old_qa_index][k - n - 1], rc_q31 ), 1 );
            a_qa[a_new_qa_index][n] = skp_l_shift!( skp_s_m_mul!( tmp_qa, rc_mul_t2_q16 ), 16 - head_rm );
        }
    }
    if  a_qa[a_new_qa_index][0] > (0.99975f64 * (1i64 << 16) as f64 + 0.5f64) as i32 || 
        a_qa[a_new_qa_index][0] < -((0.99975f64 * (1i64 << 16) as f64 + 0.5f64) as i32)
    {
        return 1;
    }
    rc_q31 = -(a_qa[a_new_qa_index][0] << 31 - 16);
    rc_mul_t1_q30 = ( i32::MAX >> 1 ) - skp_s_m_mul!( rc_q31, rc_q31 );
    *inv_gain_q30 = skp_l_shift!( skp_s_m_mul!( *inv_gain_q30, rc_mul_t1_q30 ), 2 );
    return 0;
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
    let Anew_QA = &mut Atmp_QA[(order & 1 as libc::c_int) as usize];
    k = 0 as libc::c_int;
    while k < order {
        Anew_QA[k as usize] = (A_Q12[k as usize] as libc::c_int)
            << 16 as libc::c_int - 12 as libc::c_int;
        k += 1;
    }
    return LPC_inverse_pred_gain_QA(invGain_Q30, &mut Atmp_QA, order as usize);
}
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_LPC_inverse_pred_gain_Q24(
    mut invGain_Q30: &mut libc::c_int,
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
    return LPC_inverse_pred_gain_QA(invGain_Q30, &mut Atmp_QA, order as usize);
}
