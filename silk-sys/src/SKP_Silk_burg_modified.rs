#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn SKP_Silk_inner_prod16_aligned_64(
        inVec1: *const libc::c_short,
        inVec2: *const libc::c_short,
        len: libc::c_int,
    ) -> int64_t;
    fn SKP_Silk_inner_prod_aligned(
        inVec1: *const libc::c_short,
        inVec2: *const libc::c_short,
        len: libc::c_int,
    ) -> libc::c_int;
    fn SKP_Silk_sum_sqr_shift(
        energy: *mut libc::c_int,
        shift: *mut libc::c_int,
        x: *const libc::c_short,
        len: libc::c_int,
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
pub unsafe extern "C" fn SKP_Silk_burg_modified(
    mut res_nrg: *mut libc::c_int,
    mut res_nrg_Q: *mut libc::c_int,
    mut A_Q16: *mut libc::c_int,
    mut x: *const libc::c_short,
    subfr_length: libc::c_int,
    nb_subfr: libc::c_int,
    WhiteNoiseFrac_Q32: libc::c_int,
    D: libc::c_int,
) {
    let mut k: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut s: libc::c_int = 0;
    let mut lz: libc::c_int = 0;
    let mut rshifts: libc::c_int = 0;
    let mut rshifts_extra: libc::c_int = 0;
    let mut C0: libc::c_int = 0;
    let mut num: libc::c_int = 0;
    let mut nrg: libc::c_int = 0;
    let mut rc_Q31: libc::c_int = 0;
    let mut Atmp_QA: libc::c_int = 0;
    let mut Atmp1: libc::c_int = 0;
    let mut tmp1: libc::c_int = 0;
    let mut tmp2: libc::c_int = 0;
    let mut x1: libc::c_int = 0;
    let mut x2: libc::c_int = 0;
    let mut x_ptr: *const libc::c_short = 0 as *const libc::c_short;
    let mut C_first_row: [libc::c_int; 16] = [0; 16];
    let mut C_last_row: [libc::c_int; 16] = [0; 16];
    let mut Af_QA: [libc::c_int; 16] = [0; 16];
    let mut CAf: [libc::c_int; 17] = [0; 17];
    let mut CAb: [libc::c_int; 17] = [0; 17];
    SKP_Silk_sum_sqr_shift(&mut C0, &mut rshifts, x, nb_subfr * subfr_length);
    if rshifts > 32 as libc::c_int - 25 as libc::c_int {
        C0 = C0 << rshifts - (32 as libc::c_int - 25 as libc::c_int);
        rshifts = 32 as libc::c_int - 25 as libc::c_int;
    } else {
        lz = SKP_Silk_CLZ32(C0) - 1 as libc::c_int;
        rshifts_extra = 2 as libc::c_int - lz;
        if rshifts_extra > 0 as libc::c_int {
            rshifts_extra = if rshifts_extra
                < 32 as libc::c_int - 25 as libc::c_int - rshifts
            {
                rshifts_extra
            } else {
                32 as libc::c_int - 25 as libc::c_int - rshifts
            };
            C0 = C0 >> rshifts_extra;
        } else {
            rshifts_extra = if rshifts_extra > -(16 as libc::c_int) - rshifts {
                rshifts_extra
            } else {
                -(16 as libc::c_int) - rshifts
            };
            C0 = C0 << -rshifts_extra;
        }
        rshifts += rshifts_extra;
    }
    memset(
        C_first_row.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        (16 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    if rshifts > 0 as libc::c_int {
        s = 0 as libc::c_int;
        while s < nb_subfr {
            x_ptr = x.offset((s * subfr_length) as isize);
            n = 1 as libc::c_int;
            while n < D + 1 as libc::c_int {
                C_first_row[(n - 1 as libc::c_int) as usize]
                    += (SKP_Silk_inner_prod16_aligned_64(
                        x_ptr,
                        x_ptr.offset(n as isize),
                        subfr_length - n,
                    ) >> rshifts) as libc::c_int;
                n += 1;
                n;
            }
            s += 1;
            s;
        }
    } else {
        s = 0 as libc::c_int;
        while s < nb_subfr {
            x_ptr = x.offset((s * subfr_length) as isize);
            n = 1 as libc::c_int;
            while n < D + 1 as libc::c_int {
                C_first_row[(n - 1 as libc::c_int) as usize]
                    += SKP_Silk_inner_prod_aligned(
                        x_ptr,
                        x_ptr.offset(n as isize),
                        subfr_length - n,
                    ) << -rshifts;
                n += 1;
                n;
            }
            s += 1;
            s;
        }
    }
    memcpy(
        C_last_row.as_mut_ptr() as *mut libc::c_void,
        C_first_row.as_mut_ptr() as *const libc::c_void,
        (16 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    CAf[0 as libc::c_int
        as usize] = C0
        + (WhiteNoiseFrac_Q32 as int64_t * C0 as int64_t >> 32 as libc::c_int)
            as libc::c_int + 1 as libc::c_int;
    CAb[0 as libc::c_int as usize] = CAf[0 as libc::c_int as usize];
    n = 0 as libc::c_int;
    while n < D {
        if rshifts > -(2 as libc::c_int) {
            s = 0 as libc::c_int;
            while s < nb_subfr {
                x_ptr = x.offset((s * subfr_length) as isize);
                x1 = -((*x_ptr.offset(n as isize) as libc::c_int)
                    << 16 as libc::c_int - rshifts);
                x2 = -((*x_ptr.offset((subfr_length - n - 1 as libc::c_int) as isize)
                    as libc::c_int) << 16 as libc::c_int - rshifts);
                tmp1 = (*x_ptr.offset(n as isize) as libc::c_int)
                    << 25 as libc::c_int - 16 as libc::c_int;
                tmp2 = (*x_ptr.offset((subfr_length - n - 1 as libc::c_int) as isize)
                    as libc::c_int) << 25 as libc::c_int - 16 as libc::c_int;
                k = 0 as libc::c_int;
                while k < n {
                    C_first_row[k
                        as usize] = C_first_row[k as usize]
                        + ((x1 >> 16 as libc::c_int)
                            * *x_ptr.offset((n - k - 1 as libc::c_int) as isize)
                                as libc::c_int
                            + ((x1 & 0xffff as libc::c_int)
                                * *x_ptr.offset((n - k - 1 as libc::c_int) as isize)
                                    as libc::c_int >> 16 as libc::c_int));
                    C_last_row[k
                        as usize] = C_last_row[k as usize]
                        + ((x2 >> 16 as libc::c_int)
                            * *x_ptr.offset((subfr_length - n + k) as isize)
                                as libc::c_int
                            + ((x2 & 0xffff as libc::c_int)
                                * *x_ptr.offset((subfr_length - n + k) as isize)
                                    as libc::c_int >> 16 as libc::c_int));
                    Atmp_QA = Af_QA[k as usize];
                    tmp1 = tmp1
                        + ((Atmp_QA >> 16 as libc::c_int)
                            * *x_ptr.offset((n - k - 1 as libc::c_int) as isize)
                                as libc::c_int
                            + ((Atmp_QA & 0xffff as libc::c_int)
                                * *x_ptr.offset((n - k - 1 as libc::c_int) as isize)
                                    as libc::c_int >> 16 as libc::c_int));
                    tmp2 = tmp2
                        + ((Atmp_QA >> 16 as libc::c_int)
                            * *x_ptr.offset((subfr_length - n + k) as isize)
                                as libc::c_int
                            + ((Atmp_QA & 0xffff as libc::c_int)
                                * *x_ptr.offset((subfr_length - n + k) as isize)
                                    as libc::c_int >> 16 as libc::c_int));
                    k += 1;
                    k;
                }
                tmp1 = -tmp1 << 32 as libc::c_int - 25 as libc::c_int - rshifts;
                tmp2 = -tmp2 << 32 as libc::c_int - 25 as libc::c_int - rshifts;
                k = 0 as libc::c_int;
                while k <= n {
                    CAf[k
                        as usize] = CAf[k as usize]
                        + ((tmp1 >> 16 as libc::c_int)
                            * *x_ptr.offset((n - k) as isize) as libc::c_int
                            + ((tmp1 & 0xffff as libc::c_int)
                                * *x_ptr.offset((n - k) as isize) as libc::c_int
                                >> 16 as libc::c_int));
                    CAb[k
                        as usize] = CAb[k as usize]
                        + ((tmp2 >> 16 as libc::c_int)
                            * *x_ptr
                                .offset((subfr_length - n + k - 1 as libc::c_int) as isize)
                                as libc::c_int
                            + ((tmp2 & 0xffff as libc::c_int)
                                * *x_ptr
                                    .offset((subfr_length - n + k - 1 as libc::c_int) as isize)
                                    as libc::c_int >> 16 as libc::c_int));
                    k += 1;
                    k;
                }
                s += 1;
                s;
            }
        } else {
            s = 0 as libc::c_int;
            while s < nb_subfr {
                x_ptr = x.offset((s * subfr_length) as isize);
                x1 = -((*x_ptr.offset(n as isize) as libc::c_int) << -rshifts);
                x2 = -((*x_ptr.offset((subfr_length - n - 1 as libc::c_int) as isize)
                    as libc::c_int) << -rshifts);
                tmp1 = (*x_ptr.offset(n as isize) as libc::c_int) << 17 as libc::c_int;
                tmp2 = (*x_ptr.offset((subfr_length - n - 1 as libc::c_int) as isize)
                    as libc::c_int) << 17 as libc::c_int;
                k = 0 as libc::c_int;
                while k < n {
                    C_first_row[k
                        as usize] = C_first_row[k as usize]
                        + x1
                            * *x_ptr.offset((n - k - 1 as libc::c_int) as isize)
                                as libc::c_int;
                    C_last_row[k
                        as usize] = C_last_row[k as usize]
                        + x2
                            * *x_ptr.offset((subfr_length - n + k) as isize)
                                as libc::c_int;
                    Atmp1 = if 25 as libc::c_int - 17 as libc::c_int == 1 as libc::c_int
                    {
                        (Af_QA[k as usize] >> 1 as libc::c_int)
                            + (Af_QA[k as usize] & 1 as libc::c_int)
                    } else {
                        (Af_QA[k as usize]
                            >> 25 as libc::c_int - 17 as libc::c_int - 1 as libc::c_int)
                            + 1 as libc::c_int >> 1 as libc::c_int
                    };
                    tmp1 = tmp1
                        + *x_ptr.offset((n - k - 1 as libc::c_int) as isize)
                            as libc::c_int * Atmp1;
                    tmp2 = tmp2
                        + *x_ptr.offset((subfr_length - n + k) as isize) as libc::c_int
                            * Atmp1;
                    k += 1;
                    k;
                }
                tmp1 = -tmp1;
                tmp2 = -tmp2;
                k = 0 as libc::c_int;
                while k <= n {
                    CAf[k
                        as usize] = CAf[k as usize]
                        + ((tmp1 >> 16 as libc::c_int)
                            * ((*x_ptr.offset((n - k) as isize) as libc::c_int)
                                << -rshifts - 1 as libc::c_int) as libc::c_short
                                as libc::c_int
                            + ((tmp1 & 0xffff as libc::c_int)
                                * ((*x_ptr.offset((n - k) as isize) as libc::c_int)
                                    << -rshifts - 1 as libc::c_int) as libc::c_short
                                    as libc::c_int >> 16 as libc::c_int))
                        + tmp1
                            * (if 16 as libc::c_int == 1 as libc::c_int {
                                ((*x_ptr.offset((n - k) as isize) as libc::c_int)
                                    << -rshifts - 1 as libc::c_int >> 1 as libc::c_int)
                                    + ((*x_ptr.offset((n - k) as isize) as libc::c_int)
                                        << -rshifts - 1 as libc::c_int & 1 as libc::c_int)
                            } else {
                                ((*x_ptr.offset((n - k) as isize) as libc::c_int)
                                    << -rshifts - 1 as libc::c_int
                                    >> 16 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                                    >> 1 as libc::c_int
                            });
                    CAb[k
                        as usize] = CAb[k as usize]
                        + ((tmp2 >> 16 as libc::c_int)
                            * ((*x_ptr
                                .offset((subfr_length - n + k - 1 as libc::c_int) as isize)
                                as libc::c_int) << -rshifts - 1 as libc::c_int)
                                as libc::c_short as libc::c_int
                            + ((tmp2 & 0xffff as libc::c_int)
                                * ((*x_ptr
                                    .offset((subfr_length - n + k - 1 as libc::c_int) as isize)
                                    as libc::c_int) << -rshifts - 1 as libc::c_int)
                                    as libc::c_short as libc::c_int >> 16 as libc::c_int))
                        + tmp2
                            * (if 16 as libc::c_int == 1 as libc::c_int {
                                ((*x_ptr
                                    .offset((subfr_length - n + k - 1 as libc::c_int) as isize)
                                    as libc::c_int) << -rshifts - 1 as libc::c_int
                                    >> 1 as libc::c_int)
                                    + ((*x_ptr
                                        .offset((subfr_length - n + k - 1 as libc::c_int) as isize)
                                        as libc::c_int) << -rshifts - 1 as libc::c_int
                                        & 1 as libc::c_int)
                            } else {
                                ((*x_ptr
                                    .offset((subfr_length - n + k - 1 as libc::c_int) as isize)
                                    as libc::c_int) << -rshifts - 1 as libc::c_int
                                    >> 16 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                                    >> 1 as libc::c_int
                            });
                    k += 1;
                    k;
                }
                s += 1;
                s;
            }
        }
        tmp1 = C_first_row[n as usize];
        tmp2 = C_last_row[n as usize];
        num = 0 as libc::c_int;
        nrg = CAb[0 as libc::c_int as usize] + CAf[0 as libc::c_int as usize];
        k = 0 as libc::c_int;
        while k < n {
            Atmp_QA = Af_QA[k as usize];
            lz = SKP_Silk_CLZ32(
                (if Atmp_QA > 0 as libc::c_int { Atmp_QA } else { -Atmp_QA }),
            ) - 1 as libc::c_int;
            lz = if (32 as libc::c_int - 25 as libc::c_int) < lz {
                32 as libc::c_int - 25 as libc::c_int
            } else {
                lz
            };
            Atmp1 = Atmp_QA << lz;
            tmp1 = tmp1
                + (((C_last_row[(n - k - 1 as libc::c_int) as usize] as int64_t
                    * Atmp1 as int64_t >> 32 as libc::c_int) as libc::c_int)
                    << 32 as libc::c_int - 25 as libc::c_int - lz);
            tmp2 = tmp2
                + (((C_first_row[(n - k - 1 as libc::c_int) as usize] as int64_t
                    * Atmp1 as int64_t >> 32 as libc::c_int) as libc::c_int)
                    << 32 as libc::c_int - 25 as libc::c_int - lz);
            num = num
                + (((CAb[(n - k) as usize] as int64_t * Atmp1 as int64_t
                    >> 32 as libc::c_int) as libc::c_int)
                    << 32 as libc::c_int - 25 as libc::c_int - lz);
            nrg = nrg
                + ((((CAb[(k + 1 as libc::c_int) as usize]
                    + CAf[(k + 1 as libc::c_int) as usize]) as int64_t * Atmp1 as int64_t
                    >> 32 as libc::c_int) as libc::c_int)
                    << 32 as libc::c_int - 25 as libc::c_int - lz);
            k += 1;
            k;
        }
        CAf[(n + 1 as libc::c_int) as usize] = tmp1;
        CAb[(n + 1 as libc::c_int) as usize] = tmp2;
        num = num + tmp2;
        num = -num << 1 as libc::c_int;
        if (if num > 0 as libc::c_int { num } else { -num }) < nrg {
            rc_Q31 = SKP_DIV32_varQ(num, nrg, 31 as libc::c_int);
            k = 0 as libc::c_int;
            while k < n + 1 as libc::c_int >> 1 as libc::c_int {
                tmp1 = Af_QA[k as usize];
                tmp2 = Af_QA[(n - k - 1 as libc::c_int) as usize];
                Af_QA[k
                    as usize] = tmp1
                    + (((tmp2 as int64_t * rc_Q31 as int64_t >> 32 as libc::c_int)
                        as libc::c_int) << 1 as libc::c_int);
                Af_QA[(n - k - 1 as libc::c_int)
                    as usize] = tmp2
                    + (((tmp1 as int64_t * rc_Q31 as int64_t >> 32 as libc::c_int)
                        as libc::c_int) << 1 as libc::c_int);
                k += 1;
                k;
            }
            Af_QA[n as usize] = rc_Q31 >> 31 as libc::c_int - 25 as libc::c_int;
            k = 0 as libc::c_int;
            while k <= n + 1 as libc::c_int {
                tmp1 = CAf[k as usize];
                tmp2 = CAb[(n - k + 1 as libc::c_int) as usize];
                CAf[k
                    as usize] = tmp1
                    + (((tmp2 as int64_t * rc_Q31 as int64_t >> 32 as libc::c_int)
                        as libc::c_int) << 1 as libc::c_int);
                CAb[(n - k + 1 as libc::c_int)
                    as usize] = tmp2
                    + (((tmp1 as int64_t * rc_Q31 as int64_t >> 32 as libc::c_int)
                        as libc::c_int) << 1 as libc::c_int);
                k += 1;
                k;
            }
            n += 1;
            n;
        } else {
            memset(
                &mut *Af_QA.as_mut_ptr().offset(n as isize) as *mut libc::c_int
                    as *mut libc::c_void,
                0 as libc::c_int,
                ((D - n) as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
            );
            break;
        }
    }
    nrg = CAf[0 as libc::c_int as usize];
    tmp1 = (1 as libc::c_int) << 16 as libc::c_int;
    k = 0 as libc::c_int;
    while k < D {
        Atmp1 = if 25 as libc::c_int - 16 as libc::c_int == 1 as libc::c_int {
            (Af_QA[k as usize] >> 1 as libc::c_int)
                + (Af_QA[k as usize] & 1 as libc::c_int)
        } else {
            (Af_QA[k as usize]
                >> 25 as libc::c_int - 16 as libc::c_int - 1 as libc::c_int)
                + 1 as libc::c_int >> 1 as libc::c_int
        };
        nrg = nrg
            + ((CAf[(k + 1 as libc::c_int) as usize] >> 16 as libc::c_int)
                * Atmp1 as libc::c_short as libc::c_int
                + ((CAf[(k + 1 as libc::c_int) as usize] & 0xffff as libc::c_int)
                    * Atmp1 as libc::c_short as libc::c_int >> 16 as libc::c_int))
            + CAf[(k + 1 as libc::c_int) as usize]
                * (if 16 as libc::c_int == 1 as libc::c_int {
                    (Atmp1 >> 1 as libc::c_int) + (Atmp1 & 1 as libc::c_int)
                } else {
                    (Atmp1 >> 16 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                        >> 1 as libc::c_int
                });
        tmp1 = tmp1
            + ((Atmp1 >> 16 as libc::c_int) * Atmp1 as libc::c_short as libc::c_int
                + ((Atmp1 & 0xffff as libc::c_int)
                    * Atmp1 as libc::c_short as libc::c_int >> 16 as libc::c_int))
            + Atmp1
                * (if 16 as libc::c_int == 1 as libc::c_int {
                    (Atmp1 >> 1 as libc::c_int) + (Atmp1 & 1 as libc::c_int)
                } else {
                    (Atmp1 >> 16 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                        >> 1 as libc::c_int
                });
        *A_Q16.offset(k as isize) = -Atmp1;
        k += 1;
        k;
    }
    *res_nrg = nrg
        + (((WhiteNoiseFrac_Q32 as int64_t * C0 as int64_t >> 32 as libc::c_int)
            as libc::c_int >> 16 as libc::c_int) * -tmp1 as libc::c_short as libc::c_int
            + (((WhiteNoiseFrac_Q32 as int64_t * C0 as int64_t >> 32 as libc::c_int)
                as libc::c_int & 0xffff as libc::c_int)
                * -tmp1 as libc::c_short as libc::c_int >> 16 as libc::c_int))
        + (WhiteNoiseFrac_Q32 as int64_t * C0 as int64_t >> 32 as libc::c_int)
            as libc::c_int
            * (if 16 as libc::c_int == 1 as libc::c_int {
                (-tmp1 >> 1 as libc::c_int) + (-tmp1 & 1 as libc::c_int)
            } else {
                (-tmp1 >> 16 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                    >> 1 as libc::c_int
            });
    *res_nrg_Q = -rshifts;
}
