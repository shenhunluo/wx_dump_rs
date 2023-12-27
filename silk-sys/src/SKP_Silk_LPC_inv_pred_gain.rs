#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
pub type __int64_t = libc::c_longlong;
pub type int64_t = __int64_t;
#[inline]
unsafe extern "C" fn SKP_Silk_CLZ16(mut in16: libc::c_short) -> libc::c_int {
    let mut out32: libc::c_int = 0 as libc::c_int;
    if in16 as libc::c_int == 0 as libc::c_int {
        return 16 as libc::c_int;
    }
    if in16 as libc::c_int & 0xff00 as libc::c_int != 0 {
        if in16 as libc::c_int & 0xf000 as libc::c_int != 0 {
            in16 = (in16 as libc::c_int >> 12 as libc::c_int) as libc::c_short;
        } else {
            out32 += 4 as libc::c_int;
            in16 = (in16 as libc::c_int >> 8 as libc::c_int) as libc::c_short;
        }
    } else if in16 as libc::c_int & 0xfff0 as libc::c_int != 0 {
        out32 += 8 as libc::c_int;
        in16 = (in16 as libc::c_int >> 4 as libc::c_int) as libc::c_short;
    } else {
        out32 += 12 as libc::c_int;
    }
    if in16 as libc::c_int & 0xc as libc::c_int != 0 {
        if in16 as libc::c_int & 0x8 as libc::c_int != 0 {
            return out32 + 0 as libc::c_int
        } else {
            return out32 + 1 as libc::c_int
        }
    } else if in16 as libc::c_int & 0xe as libc::c_int != 0 {
        return out32 + 2 as libc::c_int
    } else {
        return out32 + 3 as libc::c_int
    };
}
#[inline]
unsafe extern "C" fn SKP_Silk_CLZ32(mut in32: libc::c_int) -> libc::c_int {
    if in32 as libc::c_uint & 0xffff0000 as libc::c_uint != 0 {
        return SKP_Silk_CLZ16((in32 >> 16 as libc::c_int) as libc::c_short)
    } else {
        return SKP_Silk_CLZ16(in32 as libc::c_short) + 16 as libc::c_int
    };
}
#[inline]
unsafe extern "C" fn SKP_INVERSE32_varQ(
    b32: libc::c_int,
    Qres: libc::c_int,
) -> libc::c_int {
    let mut b_headrm: libc::c_int = 0;
    let mut lshift: libc::c_int = 0;
    let mut b32_inv: libc::c_int = 0;
    let mut b32_nrm: libc::c_int = 0;
    let mut err_Q32: libc::c_int = 0;
    let mut result: libc::c_int = 0;
    b_headrm = SKP_Silk_CLZ32(if b32 > 0 as libc::c_int { b32 } else { -b32 })
        - 1 as libc::c_int;
    b32_nrm = b32 << b_headrm;
    b32_inv = (0x7fffffff as libc::c_int >> 2 as libc::c_int)
        / (b32_nrm >> 16 as libc::c_int);
    result = b32_inv << 16 as libc::c_int;
    err_Q32 = -((b32_nrm >> 16 as libc::c_int) * b32_inv as libc::c_short as libc::c_int
        + ((b32_nrm & 0xffff as libc::c_int) * b32_inv as libc::c_short as libc::c_int
            >> 16 as libc::c_int)) << 3 as libc::c_int;
    result = result
        + ((err_Q32 >> 16 as libc::c_int) * b32_inv as libc::c_short as libc::c_int
            + ((err_Q32 & 0xffff as libc::c_int)
                * b32_inv as libc::c_short as libc::c_int >> 16 as libc::c_int))
        + err_Q32
            * (if 16 as libc::c_int == 1 as libc::c_int {
                (b32_inv >> 1 as libc::c_int) + (b32_inv & 1 as libc::c_int)
            } else {
                (b32_inv >> 16 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                    >> 1 as libc::c_int
            });
    lshift = 61 as libc::c_int - b_headrm - Qres;
    if lshift <= 0 as libc::c_int {
        return (if 0x80000000 as libc::c_uint as libc::c_int >> -lshift
            > 0x7fffffff as libc::c_int >> -lshift
        {
            if result > 0x80000000 as libc::c_uint as libc::c_int >> -lshift {
                0x80000000 as libc::c_uint as libc::c_int >> -lshift
            } else {
                if result < 0x7fffffff as libc::c_int >> -lshift {
                    0x7fffffff as libc::c_int >> -lshift
                } else {
                    result
                }
            }
        } else {
            if result > 0x7fffffff as libc::c_int >> -lshift {
                0x7fffffff as libc::c_int >> -lshift
            } else {
                if result < 0x80000000 as libc::c_uint as libc::c_int >> -lshift {
                    0x80000000 as libc::c_uint as libc::c_int >> -lshift
                } else {
                    result
                }
            }
        }) << -lshift
    } else if lshift < 32 as libc::c_int {
        return result >> lshift
    } else {
        return 0 as libc::c_int
    };
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
    mut invGain_Q30: *mut libc::c_int,
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
