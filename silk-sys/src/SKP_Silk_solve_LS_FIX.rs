#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
pub type __int64_t = libc::c_longlong;
pub type int64_t = __int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct inv_D_t {
    pub Q36_part: libc::c_int,
    pub Q48_part: libc::c_int,
}
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
unsafe extern "C" fn SKP_max_32(mut a: libc::c_int, mut b: libc::c_int) -> libc::c_int {
    return if a > b { a } else { b };
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
    b_headrm = SKP_Silk_CLZ32((if b32 > 0 as libc::c_int { b32 } else { -b32 }))
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
pub unsafe extern "C" fn SKP_Silk_solve_LDL_FIX(
    mut A: *mut libc::c_int,
    mut M: libc::c_int,
    mut b: *const libc::c_int,
    mut x_Q16: *mut libc::c_int,
) {
    let mut L_Q16: [libc::c_int; 256] = [0; 256];
    let mut Y: [libc::c_int; 16] = [0; 16];
    let mut inv_D: [inv_D_t; 16] = [inv_D_t {
        Q36_part: 0,
        Q48_part: 0,
    }; 16];
    SKP_Silk_LDL_factorize_FIX(A, M, L_Q16.as_mut_ptr(), inv_D.as_mut_ptr());
    SKP_Silk_LS_SolveFirst_FIX(L_Q16.as_mut_ptr(), M, b, Y.as_mut_ptr());
    SKP_Silk_LS_divide_Q16_FIX(Y.as_mut_ptr(), inv_D.as_mut_ptr(), M);
    SKP_Silk_LS_SolveLast_FIX(L_Q16.as_mut_ptr(), M, Y.as_mut_ptr(), x_Q16);
}
#[inline]
unsafe extern "C" fn SKP_Silk_LDL_factorize_FIX(
    mut A: *mut libc::c_int,
    mut M: libc::c_int,
    mut L_Q16: *mut libc::c_int,
    mut inv_D: *mut inv_D_t,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut status: libc::c_int = 0;
    let mut loop_count: libc::c_int = 0;
    let mut ptr1: *const libc::c_int = 0 as *const libc::c_int;
    let mut ptr2: *const libc::c_int = 0 as *const libc::c_int;
    let mut diag_min_value: libc::c_int = 0;
    let mut tmp_32: libc::c_int = 0;
    let mut err: libc::c_int = 0;
    let mut v_Q0: [libc::c_int; 16] = [0; 16];
    let mut D_Q0: [libc::c_int; 16] = [0; 16];
    let mut one_div_diag_Q36: libc::c_int = 0;
    let mut one_div_diag_Q40: libc::c_int = 0;
    let mut one_div_diag_Q48: libc::c_int = 0;
    status = 1 as libc::c_int;
    diag_min_value = SKP_max_32(
        ((if (*A.offset(0 as libc::c_int as isize)
            + *A
                .offset(
                    (M as libc::c_short as libc::c_int
                        * M as libc::c_short as libc::c_int - 1 as libc::c_int) as isize,
                )) as libc::c_uint & 0x80000000 as libc::c_uint
            == 0 as libc::c_int as libc::c_uint
        {
            (if (*A.offset(0 as libc::c_int as isize)
                & *A
                    .offset(
                        (M as libc::c_short as libc::c_int
                            * M as libc::c_short as libc::c_int - 1 as libc::c_int)
                            as isize,
                    )) as libc::c_uint & 0x80000000 as libc::c_uint
                != 0 as libc::c_int as libc::c_uint
            {
                0x80000000 as libc::c_uint as libc::c_int
            } else {
                *A.offset(0 as libc::c_int as isize)
                    + *A
                        .offset(
                            (M as libc::c_short as libc::c_int
                                * M as libc::c_short as libc::c_int - 1 as libc::c_int)
                                as isize,
                        )
            })
        } else {
            (if (*A.offset(0 as libc::c_int as isize)
                | *A
                    .offset(
                        (M as libc::c_short as libc::c_int
                            * M as libc::c_short as libc::c_int - 1 as libc::c_int)
                            as isize,
                    )) as libc::c_uint & 0x80000000 as libc::c_uint
                == 0 as libc::c_int as libc::c_uint
            {
                0x7fffffff as libc::c_int
            } else {
                *A.offset(0 as libc::c_int as isize)
                    + *A
                        .offset(
                            (M as libc::c_short as libc::c_int
                                * M as libc::c_short as libc::c_int - 1 as libc::c_int)
                                as isize,
                        )
            })
        }) as int64_t
            * ((1e-5f32
                * ((1 as libc::c_int as int64_t) << 31 as libc::c_int) as libc::c_float)
                as libc::c_double + 0.5f64) as libc::c_int as int64_t
            >> 32 as libc::c_int) as libc::c_int,
        (1 as libc::c_int) << 9 as libc::c_int,
    );
    loop_count = 0 as libc::c_int;
    while loop_count < M && status == 1 as libc::c_int {
        status = 0 as libc::c_int;
        j = 0 as libc::c_int;
        while j < M {
            ptr1 = L_Q16.offset((j * M + 0 as libc::c_int) as isize);
            tmp_32 = 0 as libc::c_int;
            i = 0 as libc::c_int;
            while i < j {
                v_Q0[i
                    as usize] = (D_Q0[i as usize] >> 16 as libc::c_int)
                    * *ptr1.offset(i as isize) as libc::c_short as libc::c_int
                    + ((D_Q0[i as usize] & 0xffff as libc::c_int)
                        * *ptr1.offset(i as isize) as libc::c_short as libc::c_int
                        >> 16 as libc::c_int)
                    + D_Q0[i as usize]
                        * (if 16 as libc::c_int == 1 as libc::c_int {
                            (*ptr1.offset(i as isize) >> 1 as libc::c_int)
                                + (*ptr1.offset(i as isize) & 1 as libc::c_int)
                        } else {
                            (*ptr1.offset(i as isize)
                                >> 16 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                                >> 1 as libc::c_int
                        });
                tmp_32 = tmp_32
                    + ((v_Q0[i as usize] >> 16 as libc::c_int)
                        * *ptr1.offset(i as isize) as libc::c_short as libc::c_int
                        + ((v_Q0[i as usize] & 0xffff as libc::c_int)
                            * *ptr1.offset(i as isize) as libc::c_short as libc::c_int
                            >> 16 as libc::c_int))
                    + v_Q0[i as usize]
                        * (if 16 as libc::c_int == 1 as libc::c_int {
                            (*ptr1.offset(i as isize) >> 1 as libc::c_int)
                                + (*ptr1.offset(i as isize) & 1 as libc::c_int)
                        } else {
                            (*ptr1.offset(i as isize)
                                >> 16 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                                >> 1 as libc::c_int
                        });
                i += 1;
                i;
            }
            tmp_32 = *A.offset((j * M + j) as isize) - tmp_32;
            if tmp_32 < diag_min_value {
                tmp_32 = (loop_count + 1 as libc::c_int) as libc::c_short as libc::c_int
                    * diag_min_value as libc::c_short as libc::c_int - tmp_32;
                i = 0 as libc::c_int;
                while i < M {
                    *A
                        .offset(
                            (i * M + i) as isize,
                        ) = *A.offset((i * M + i) as isize) + tmp_32;
                    i += 1;
                    i;
                }
                status = 1 as libc::c_int;
                break;
            } else {
                D_Q0[j as usize] = tmp_32;
                one_div_diag_Q36 = SKP_INVERSE32_varQ(tmp_32, 36 as libc::c_int);
                one_div_diag_Q40 = one_div_diag_Q36 << 4 as libc::c_int;
                err = ((1 as libc::c_int) << 24 as libc::c_int)
                    - ((tmp_32 >> 16 as libc::c_int)
                        * one_div_diag_Q40 as libc::c_short as libc::c_int
                        + ((tmp_32 & 0xffff as libc::c_int)
                            * one_div_diag_Q40 as libc::c_short as libc::c_int
                            >> 16 as libc::c_int)
                        + tmp_32
                            * (if 16 as libc::c_int == 1 as libc::c_int {
                                (one_div_diag_Q40 >> 1 as libc::c_int)
                                    + (one_div_diag_Q40 & 1 as libc::c_int)
                            } else {
                                (one_div_diag_Q40 >> 16 as libc::c_int - 1 as libc::c_int)
                                    + 1 as libc::c_int >> 1 as libc::c_int
                            }));
                one_div_diag_Q48 = (err >> 16 as libc::c_int)
                    * one_div_diag_Q40 as libc::c_short as libc::c_int
                    + ((err & 0xffff as libc::c_int)
                        * one_div_diag_Q40 as libc::c_short as libc::c_int
                        >> 16 as libc::c_int)
                    + err
                        * (if 16 as libc::c_int == 1 as libc::c_int {
                            (one_div_diag_Q40 >> 1 as libc::c_int)
                                + (one_div_diag_Q40 & 1 as libc::c_int)
                        } else {
                            (one_div_diag_Q40 >> 16 as libc::c_int - 1 as libc::c_int)
                                + 1 as libc::c_int >> 1 as libc::c_int
                        });
                (*inv_D.offset(j as isize)).Q36_part = one_div_diag_Q36;
                (*inv_D.offset(j as isize)).Q48_part = one_div_diag_Q48;
                *L_Q16.offset((j * M + j) as isize) = 65536 as libc::c_int;
                ptr1 = A.offset((j * M + 0 as libc::c_int) as isize);
                ptr2 = L_Q16
                    .offset(((j + 1 as libc::c_int) * M + 0 as libc::c_int) as isize);
                i = j + 1 as libc::c_int;
                while i < M {
                    tmp_32 = 0 as libc::c_int;
                    k = 0 as libc::c_int;
                    while k < j {
                        tmp_32 = tmp_32
                            + ((v_Q0[k as usize] >> 16 as libc::c_int)
                                * *ptr2.offset(k as isize) as libc::c_short as libc::c_int
                                + ((v_Q0[k as usize] & 0xffff as libc::c_int)
                                    * *ptr2.offset(k as isize) as libc::c_short as libc::c_int
                                    >> 16 as libc::c_int))
                            + v_Q0[k as usize]
                                * (if 16 as libc::c_int == 1 as libc::c_int {
                                    (*ptr2.offset(k as isize) >> 1 as libc::c_int)
                                        + (*ptr2.offset(k as isize) & 1 as libc::c_int)
                                } else {
                                    (*ptr2.offset(k as isize)
                                        >> 16 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                                        >> 1 as libc::c_int
                                });
                        k += 1;
                        k;
                    }
                    tmp_32 = *ptr1.offset(i as isize) - tmp_32;
                    *L_Q16
                        .offset(
                            (i * M + j) as isize,
                        ) = (tmp_32 as int64_t * one_div_diag_Q48 as int64_t
                        >> 32 as libc::c_int) as libc::c_int
                        + ((tmp_32 >> 16 as libc::c_int)
                            * one_div_diag_Q36 as libc::c_short as libc::c_int
                            + ((tmp_32 & 0xffff as libc::c_int)
                                * one_div_diag_Q36 as libc::c_short as libc::c_int
                                >> 16 as libc::c_int)
                            + tmp_32
                                * (if 16 as libc::c_int == 1 as libc::c_int {
                                    (one_div_diag_Q36 >> 1 as libc::c_int)
                                        + (one_div_diag_Q36 & 1 as libc::c_int)
                                } else {
                                    (one_div_diag_Q36 >> 16 as libc::c_int - 1 as libc::c_int)
                                        + 1 as libc::c_int >> 1 as libc::c_int
                                }) >> 4 as libc::c_int);
                    ptr2 = ptr2.offset(M as isize);
                    i += 1;
                    i;
                }
                j += 1;
                j;
            }
        }
        loop_count += 1;
        loop_count;
    }
}
#[inline]
unsafe extern "C" fn SKP_Silk_LS_divide_Q16_FIX(
    mut T: *mut libc::c_int,
    mut inv_D: *mut inv_D_t,
    mut M: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut tmp_32: libc::c_int = 0;
    let mut one_div_diag_Q36: libc::c_int = 0;
    let mut one_div_diag_Q48: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < M {
        one_div_diag_Q36 = (*inv_D.offset(i as isize)).Q36_part;
        one_div_diag_Q48 = (*inv_D.offset(i as isize)).Q48_part;
        tmp_32 = *T.offset(i as isize);
        *T
            .offset(
                i as isize,
            ) = (tmp_32 as int64_t * one_div_diag_Q48 as int64_t >> 32 as libc::c_int)
            as libc::c_int
            + ((tmp_32 >> 16 as libc::c_int)
                * one_div_diag_Q36 as libc::c_short as libc::c_int
                + ((tmp_32 & 0xffff as libc::c_int)
                    * one_div_diag_Q36 as libc::c_short as libc::c_int
                    >> 16 as libc::c_int)
                + tmp_32
                    * (if 16 as libc::c_int == 1 as libc::c_int {
                        (one_div_diag_Q36 >> 1 as libc::c_int)
                            + (one_div_diag_Q36 & 1 as libc::c_int)
                    } else {
                        (one_div_diag_Q36 >> 16 as libc::c_int - 1 as libc::c_int)
                            + 1 as libc::c_int >> 1 as libc::c_int
                    }) >> 4 as libc::c_int);
        i += 1;
        i;
    }
}
#[inline]
unsafe extern "C" fn SKP_Silk_LS_SolveFirst_FIX(
    mut L_Q16: *const libc::c_int,
    mut M: libc::c_int,
    mut b: *const libc::c_int,
    mut x_Q16: *mut libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut ptr32: *const libc::c_int = 0 as *const libc::c_int;
    let mut tmp_32: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < M {
        ptr32 = L_Q16.offset((i * M + 0 as libc::c_int) as isize);
        tmp_32 = 0 as libc::c_int;
        j = 0 as libc::c_int;
        while j < i {
            tmp_32 = tmp_32
                + ((*ptr32.offset(j as isize) >> 16 as libc::c_int)
                    * *x_Q16.offset(j as isize) as libc::c_short as libc::c_int
                    + ((*ptr32.offset(j as isize) & 0xffff as libc::c_int)
                        * *x_Q16.offset(j as isize) as libc::c_short as libc::c_int
                        >> 16 as libc::c_int))
                + *ptr32.offset(j as isize)
                    * (if 16 as libc::c_int == 1 as libc::c_int {
                        (*x_Q16.offset(j as isize) >> 1 as libc::c_int)
                            + (*x_Q16.offset(j as isize) & 1 as libc::c_int)
                    } else {
                        (*x_Q16.offset(j as isize)
                            >> 16 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                            >> 1 as libc::c_int
                    });
            j += 1;
            j;
        }
        *x_Q16.offset(i as isize) = *b.offset(i as isize) - tmp_32;
        i += 1;
        i;
    }
}
#[inline]
unsafe extern "C" fn SKP_Silk_LS_SolveLast_FIX(
    mut L_Q16: *const libc::c_int,
    M: libc::c_int,
    mut b: *const libc::c_int,
    mut x_Q16: *mut libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut ptr32: *const libc::c_int = 0 as *const libc::c_int;
    let mut tmp_32: libc::c_int = 0;
    i = M - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        ptr32 = L_Q16.offset((0 as libc::c_int * M + i) as isize);
        tmp_32 = 0 as libc::c_int;
        j = M - 1 as libc::c_int;
        while j > i {
            tmp_32 = tmp_32
                + ((*ptr32
                    .offset(
                        (j as libc::c_short as libc::c_int
                            * M as libc::c_short as libc::c_int) as isize,
                    ) >> 16 as libc::c_int)
                    * *x_Q16.offset(j as isize) as libc::c_short as libc::c_int
                    + ((*ptr32
                        .offset(
                            (j as libc::c_short as libc::c_int
                                * M as libc::c_short as libc::c_int) as isize,
                        ) & 0xffff as libc::c_int)
                        * *x_Q16.offset(j as isize) as libc::c_short as libc::c_int
                        >> 16 as libc::c_int))
                + *ptr32
                    .offset(
                        (j as libc::c_short as libc::c_int
                            * M as libc::c_short as libc::c_int) as isize,
                    )
                    * (if 16 as libc::c_int == 1 as libc::c_int {
                        (*x_Q16.offset(j as isize) >> 1 as libc::c_int)
                            + (*x_Q16.offset(j as isize) & 1 as libc::c_int)
                    } else {
                        (*x_Q16.offset(j as isize)
                            >> 16 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                            >> 1 as libc::c_int
                    });
            j -= 1;
            j;
        }
        *x_Q16.offset(i as isize) = *b.offset(i as isize) - tmp_32;
        i -= 1;
        i;
    }
}
