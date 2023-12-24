#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn SKP_Silk_corrMatrix_FIX(
        x: *const libc::c_short,
        L: libc::c_int,
        order: libc::c_int,
        head_room: libc::c_int,
        XX: *mut libc::c_int,
        rshifts: *mut libc::c_int,
    );
    fn SKP_Silk_corrVector_FIX(
        x: *const libc::c_short,
        t: *const libc::c_short,
        L: libc::c_int,
        order: libc::c_int,
        Xt: *mut libc::c_int,
        rshifts: libc::c_int,
    );
    fn SKP_Silk_regularize_correlations_FIX(
        XX: *mut libc::c_int,
        xx: *mut libc::c_int,
        noise: libc::c_int,
        D: libc::c_int,
    );
    fn SKP_Silk_solve_LDL_FIX(
        A: *mut libc::c_int,
        M: libc::c_int,
        b: *const libc::c_int,
        x_Q16: *mut libc::c_int,
    );
    fn SKP_Silk_residual_energy16_covar_FIX(
        c: *const libc::c_short,
        wXX: *const libc::c_int,
        wXx: *const libc::c_int,
        wxx: libc::c_int,
        D: libc::c_int,
        cQ: libc::c_int,
    ) -> libc::c_int;
    fn SKP_Silk_lin2log(inLin: libc::c_int) -> libc::c_int;
    fn SKP_Silk_sum_sqr_shift(
        energy: *mut libc::c_int,
        shift: *mut libc::c_int,
        x: *const libc::c_short,
        len: libc::c_int,
    );
    fn SKP_Silk_scale_vector32_Q26_lshift_18(
        data1: *mut libc::c_int,
        gain_Q26: libc::c_int,
        dataSize: libc::c_int,
    );
}
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
unsafe extern "C" fn SKP_min_int(mut a: libc::c_int, mut b: libc::c_int) -> libc::c_int {
    return if a < b { a } else { b };
}
#[inline]
unsafe extern "C" fn SKP_min_32(mut a: libc::c_int, mut b: libc::c_int) -> libc::c_int {
    return if a < b { a } else { b };
}
#[inline]
unsafe extern "C" fn SKP_max_int(mut a: libc::c_int, mut b: libc::c_int) -> libc::c_int {
    return if a > b { a } else { b };
}
#[inline]
unsafe extern "C" fn SKP_max_16(
    mut a: libc::c_short,
    mut b: libc::c_short,
) -> libc::c_short {
    return (if a as libc::c_int > b as libc::c_int {
        a as libc::c_int
    } else {
        b as libc::c_int
    }) as libc::c_short;
}
#[inline]
unsafe extern "C" fn SKP_max_32(mut a: libc::c_int, mut b: libc::c_int) -> libc::c_int {
    return if a > b { a } else { b };
}
#[inline]
unsafe extern "C" fn SKP_DIV32_varQ(
    a32: libc::c_int,
    b32: libc::c_int,
    Qres: libc::c_int,
) -> libc::c_int {
    let mut a_headrm: libc::c_int = 0;
    let mut b_headrm: libc::c_int = 0;
    let mut lshift: libc::c_int = 0;
    let mut b32_inv: libc::c_int = 0;
    let mut a32_nrm: libc::c_int = 0;
    let mut b32_nrm: libc::c_int = 0;
    let mut result: libc::c_int = 0;
    a_headrm = SKP_Silk_CLZ32((if a32 > 0 as libc::c_int { a32 } else { -a32 }))
        - 1 as libc::c_int;
    a32_nrm = a32 << a_headrm;
    b_headrm = SKP_Silk_CLZ32((if b32 > 0 as libc::c_int { b32 } else { -b32 }))
        - 1 as libc::c_int;
    b32_nrm = b32 << b_headrm;
    b32_inv = (0x7fffffff as libc::c_int >> 2 as libc::c_int)
        / (b32_nrm >> 16 as libc::c_int);
    result = (a32_nrm >> 16 as libc::c_int) * b32_inv as libc::c_short as libc::c_int
        + ((a32_nrm & 0xffff as libc::c_int) * b32_inv as libc::c_short as libc::c_int
            >> 16 as libc::c_int);
    a32_nrm
        -= ((b32_nrm as int64_t * result as int64_t >> 32 as libc::c_int) as libc::c_int)
            << 3 as libc::c_int;
    result = result
        + ((a32_nrm >> 16 as libc::c_int) * b32_inv as libc::c_short as libc::c_int
            + ((a32_nrm & 0xffff as libc::c_int)
                * b32_inv as libc::c_short as libc::c_int >> 16 as libc::c_int));
    lshift = 29 as libc::c_int + a_headrm - b_headrm - Qres;
    if lshift <= 0 as libc::c_int {
        return (if 0x80000000 as libc::c_uint as libc::c_int >> -lshift
            > 0x7fffffff as libc::c_int >> -lshift
        {
            (if result > 0x80000000 as libc::c_uint as libc::c_int >> -lshift {
                0x80000000 as libc::c_uint as libc::c_int >> -lshift
            } else {
                (if result < 0x7fffffff as libc::c_int >> -lshift {
                    0x7fffffff as libc::c_int >> -lshift
                } else {
                    result
                })
            })
        } else {
            (if result > 0x7fffffff as libc::c_int >> -lshift {
                0x7fffffff as libc::c_int >> -lshift
            } else {
                (if result < 0x80000000 as libc::c_uint as libc::c_int >> -lshift {
                    0x80000000 as libc::c_uint as libc::c_int >> -lshift
                } else {
                    result
                })
            })
        }) << -lshift
    } else if lshift < 32 as libc::c_int {
        return result >> lshift
    } else {
        return 0 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_fit_LTP(
    mut LTP_coefs_Q16: *mut libc::c_int,
    mut LTP_coefs_Q14: *mut libc::c_short,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 5 as libc::c_int {
        *LTP_coefs_Q14
            .offset(
                i as isize,
            ) = (if (if 2 as libc::c_int == 1 as libc::c_int {
            (*LTP_coefs_Q16.offset(i as isize) >> 1 as libc::c_int)
                + (*LTP_coefs_Q16.offset(i as isize) & 1 as libc::c_int)
        } else {
            (*LTP_coefs_Q16.offset(i as isize) >> 2 as libc::c_int - 1 as libc::c_int)
                + 1 as libc::c_int >> 1 as libc::c_int
        }) > 0x7fff as libc::c_int
        {
            0x7fff as libc::c_int
        } else if (if 2 as libc::c_int == 1 as libc::c_int {
            (*LTP_coefs_Q16.offset(i as isize) >> 1 as libc::c_int)
                + (*LTP_coefs_Q16.offset(i as isize) & 1 as libc::c_int)
        } else {
            (*LTP_coefs_Q16.offset(i as isize) >> 2 as libc::c_int - 1 as libc::c_int)
                + 1 as libc::c_int >> 1 as libc::c_int
        }) < 0x8000 as libc::c_int as libc::c_short as libc::c_int
        {
            0x8000 as libc::c_int as libc::c_short as libc::c_int
        } else if 2 as libc::c_int == 1 as libc::c_int {
            (*LTP_coefs_Q16.offset(i as isize) >> 1 as libc::c_int)
                + (*LTP_coefs_Q16.offset(i as isize) & 1 as libc::c_int)
        } else {
            (*LTP_coefs_Q16.offset(i as isize) >> 2 as libc::c_int - 1 as libc::c_int)
                + 1 as libc::c_int >> 1 as libc::c_int
        }) as libc::c_short;
        i += 1;
        i;
    }
}
