#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
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
pub unsafe extern "C" fn SKP_Silk_schur64(
    mut rc_Q16: *mut libc::c_int,
    mut c: *const libc::c_int,
    mut order: libc::c_int,
) -> libc::c_int {
    let mut k: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut C: [[libc::c_int; 2]; 17] = [[0; 2]; 17];
    let mut Ctmp1_Q30: libc::c_int = 0;
    let mut Ctmp2_Q30: libc::c_int = 0;
    let mut rc_tmp_Q31: libc::c_int = 0;
    if *c.offset(0 as libc::c_int as isize) <= 0 as libc::c_int {
        memset(
            rc_Q16 as *mut libc::c_void,
            0 as libc::c_int,
            (order as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
        );
        return 0 as libc::c_int;
    }
    k = 0 as libc::c_int;
    while k < order + 1 as libc::c_int {
        C[k as usize][1 as libc::c_int as usize] = *c.offset(k as isize);
        C[k
            as usize][0 as libc::c_int
            as usize] = C[k as usize][1 as libc::c_int as usize];
        k += 1;
        k;
    }
    k = 0 as libc::c_int;
    while k < order {
        rc_tmp_Q31 = SKP_DIV32_varQ(
            -C[(k + 1 as libc::c_int) as usize][0 as libc::c_int as usize],
            C[0 as libc::c_int as usize][1 as libc::c_int as usize],
            31 as libc::c_int,
        );
        *rc_Q16
            .offset(
                k as isize,
            ) = if 15 as libc::c_int == 1 as libc::c_int {
            (rc_tmp_Q31 >> 1 as libc::c_int) + (rc_tmp_Q31 & 1 as libc::c_int)
        } else {
            (rc_tmp_Q31 >> 15 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                >> 1 as libc::c_int
        };
        n = 0 as libc::c_int;
        while n < order - k {
            Ctmp1_Q30 = C[(n + k + 1 as libc::c_int)
                as usize][0 as libc::c_int as usize];
            Ctmp2_Q30 = C[n as usize][1 as libc::c_int as usize];
            C[(n + k + 1 as libc::c_int)
                as usize][0 as libc::c_int
                as usize] = Ctmp1_Q30
                + ((Ctmp2_Q30 << 1 as libc::c_int) as int64_t * rc_tmp_Q31 as int64_t
                    >> 32 as libc::c_int) as libc::c_int;
            C[n
                as usize][1 as libc::c_int
                as usize] = Ctmp2_Q30
                + ((Ctmp1_Q30 << 1 as libc::c_int) as int64_t * rc_tmp_Q31 as int64_t
                    >> 32 as libc::c_int) as libc::c_int;
            n += 1;
            n;
        }
        k += 1;
        k;
    }
    return C[0 as libc::c_int as usize][1 as libc::c_int as usize];
}
