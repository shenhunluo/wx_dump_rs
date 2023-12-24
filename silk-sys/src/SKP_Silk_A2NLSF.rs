#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn SKP_Silk_bwexpander_32(
        ar: *mut libc::c_int,
        d: libc::c_int,
        chirp_Q16: libc::c_int,
    );
    static SKP_Silk_LSFCosTab_FIX_Q12: [libc::c_int; 129];
}
#[inline]
unsafe extern "C" fn SKP_min_32(mut a: libc::c_int, mut b: libc::c_int) -> libc::c_int {
    return if a < b { a } else { b };
}
#[inline]
unsafe extern "C" fn SKP_Silk_A2NLSF_trans_poly(
    mut p: *mut libc::c_int,
    dd: libc::c_int,
) {
    let mut k: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    k = 2 as libc::c_int;
    while k <= dd {
        n = dd;
        while n > k {
            *p.offset((n - 2 as libc::c_int) as isize) -= *p.offset(n as isize);
            n -= 1;
            n;
        }
        *p.offset((k - 2 as libc::c_int) as isize)
            -= *p.offset(k as isize) << 1 as libc::c_int;
        k += 1;
        k;
    }
}
#[inline]
unsafe extern "C" fn SKP_Silk_A2NLSF_eval_poly(
    mut p: *mut libc::c_int,
    x: libc::c_int,
    dd: libc::c_int,
) -> libc::c_int {
    let mut n: libc::c_int = 0;
    let mut x_Q16: libc::c_int = 0;
    let mut y32: libc::c_int = 0;
    y32 = *p.offset(dd as isize);
    x_Q16 = x << 4 as libc::c_int;
    n = dd - 1 as libc::c_int;
    while n >= 0 as libc::c_int {
        y32 = *p.offset(n as isize)
            + ((y32 >> 16 as libc::c_int) * x_Q16 as libc::c_short as libc::c_int
                + ((y32 & 0xffff as libc::c_int) * x_Q16 as libc::c_short as libc::c_int
                    >> 16 as libc::c_int))
            + y32
                * (if 16 as libc::c_int == 1 as libc::c_int {
                    (x_Q16 >> 1 as libc::c_int) + (x_Q16 & 1 as libc::c_int)
                } else {
                    (x_Q16 >> 16 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                        >> 1 as libc::c_int
                });
        n -= 1;
        n;
    }
    return y32;
}
#[inline]
unsafe extern "C" fn SKP_Silk_A2NLSF_init(
    mut a_Q16: *const libc::c_int,
    mut P: *mut libc::c_int,
    mut Q: *mut libc::c_int,
    dd: libc::c_int,
) {
    let mut k: libc::c_int = 0;
    *P.offset(dd as isize) = (1 as libc::c_int) << 16 as libc::c_int;
    *Q.offset(dd as isize) = (1 as libc::c_int) << 16 as libc::c_int;
    k = 0 as libc::c_int;
    while k < dd {
        *P
            .offset(
                k as isize,
            ) = -*a_Q16.offset((dd - k - 1 as libc::c_int) as isize)
            - *a_Q16.offset((dd + k) as isize);
        *Q
            .offset(
                k as isize,
            ) = -*a_Q16.offset((dd - k - 1 as libc::c_int) as isize)
            + *a_Q16.offset((dd + k) as isize);
        k += 1;
        k;
    }
    k = dd;
    while k > 0 as libc::c_int {
        *P.offset((k - 1 as libc::c_int) as isize) -= *P.offset(k as isize);
        *Q.offset((k - 1 as libc::c_int) as isize) += *Q.offset(k as isize);
        k -= 1;
        k;
    }
    SKP_Silk_A2NLSF_trans_poly(P, dd);
    SKP_Silk_A2NLSF_trans_poly(Q, dd);
}
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_A2NLSF(
    mut NLSF: *mut libc::c_int,
    mut a_Q16: *mut libc::c_int,
    d: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut dd: libc::c_int = 0;
    let mut root_ix: libc::c_int = 0;
    let mut ffrac: libc::c_int = 0;
    let mut xlo: libc::c_int = 0;
    let mut xhi: libc::c_int = 0;
    let mut xmid: libc::c_int = 0;
    let mut ylo: libc::c_int = 0;
    let mut yhi: libc::c_int = 0;
    let mut ymid: libc::c_int = 0;
    let mut nom: libc::c_int = 0;
    let mut den: libc::c_int = 0;
    let mut P: [libc::c_int; 9] = [0; 9];
    let mut Q: [libc::c_int; 9] = [0; 9];
    let mut PQ: [*mut libc::c_int; 2] = [0 as *mut libc::c_int; 2];
    let mut p: *mut libc::c_int = 0 as *mut libc::c_int;
    PQ[0 as libc::c_int as usize] = P.as_mut_ptr();
    PQ[1 as libc::c_int as usize] = Q.as_mut_ptr();
    dd = d >> 1 as libc::c_int;
    SKP_Silk_A2NLSF_init(a_Q16, P.as_mut_ptr(), Q.as_mut_ptr(), dd);
    p = P.as_mut_ptr();
    xlo = SKP_Silk_LSFCosTab_FIX_Q12[0 as libc::c_int as usize];
    ylo = SKP_Silk_A2NLSF_eval_poly(p, xlo, dd);
    if ylo < 0 as libc::c_int {
        *NLSF.offset(0 as libc::c_int as isize) = 0 as libc::c_int;
        p = Q.as_mut_ptr();
        ylo = SKP_Silk_A2NLSF_eval_poly(p, xlo, dd);
        root_ix = 1 as libc::c_int;
    } else {
        root_ix = 0 as libc::c_int;
    }
    k = 1 as libc::c_int;
    i = 0 as libc::c_int;
    loop {
        xhi = SKP_Silk_LSFCosTab_FIX_Q12[k as usize];
        yhi = SKP_Silk_A2NLSF_eval_poly(p, xhi, dd);
        if ylo <= 0 as libc::c_int && yhi >= 0 as libc::c_int
            || ylo >= 0 as libc::c_int && yhi <= 0 as libc::c_int
        {
            ffrac = -(256 as libc::c_int);
            m = 0 as libc::c_int;
            while m < 3 as libc::c_int {
                xmid = if 1 as libc::c_int == 1 as libc::c_int {
                    (xlo + xhi >> 1 as libc::c_int) + (xlo + xhi & 1 as libc::c_int)
                } else {
                    (xlo + xhi >> 1 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                        >> 1 as libc::c_int
                };
                ymid = SKP_Silk_A2NLSF_eval_poly(p, xmid, dd);
                if ylo <= 0 as libc::c_int && ymid >= 0 as libc::c_int
                    || ylo >= 0 as libc::c_int && ymid <= 0 as libc::c_int
                {
                    xhi = xmid;
                    yhi = ymid;
                } else {
                    xlo = xmid;
                    ylo = ymid;
                    ffrac = ffrac + (128 as libc::c_int >> m);
                }
                m += 1;
                m;
            }
            if (if ylo > 0 as libc::c_int { ylo } else { -ylo }) < 65536 as libc::c_int {
                den = ylo - yhi;
                nom = (ylo << 8 as libc::c_int - 3 as libc::c_int)
                    + (den >> 1 as libc::c_int);
                if den != 0 as libc::c_int {
                    ffrac += nom / den;
                }
            } else {
                ffrac += ylo / (ylo - yhi >> 8 as libc::c_int - 3 as libc::c_int);
            }
            *NLSF
                .offset(
                    root_ix as isize,
                ) = SKP_min_32((k << 8 as libc::c_int) + ffrac, 0x7fff as libc::c_int);
            root_ix += 1;
            root_ix;
            if root_ix >= d {
                break;
            }
            p = PQ[(root_ix & 1 as libc::c_int) as usize];
            xlo = SKP_Silk_LSFCosTab_FIX_Q12[(k - 1 as libc::c_int) as usize];
            ylo = 1 as libc::c_int - (root_ix & 2 as libc::c_int) << 12 as libc::c_int;
        } else {
            k += 1;
            k;
            xlo = xhi;
            ylo = yhi;
            if k > 128 as libc::c_int {
                i += 1;
                i;
                if i > 30 as libc::c_int {
                    *NLSF
                        .offset(
                            0 as libc::c_int as isize,
                        ) = ((1 as libc::c_int) << 15 as libc::c_int)
                        / (d + 1 as libc::c_int);
                    k = 1 as libc::c_int;
                    while k < d {
                        *NLSF
                            .offset(
                                k as isize,
                            ) = (k + 1 as libc::c_int) as libc::c_short as libc::c_int
                            * *NLSF.offset(0 as libc::c_int as isize) as libc::c_short
                                as libc::c_int;
                        k += 1;
                        k;
                    }
                    return;
                }
                SKP_Silk_bwexpander_32(
                    a_Q16,
                    d,
                    65536 as libc::c_int
                        - (10 as libc::c_int + i) as libc::c_short as libc::c_int
                            * i as libc::c_short as libc::c_int,
                );
                SKP_Silk_A2NLSF_init(a_Q16, P.as_mut_ptr(), Q.as_mut_ptr(), dd);
                p = P.as_mut_ptr();
                xlo = SKP_Silk_LSFCosTab_FIX_Q12[0 as libc::c_int as usize];
                ylo = SKP_Silk_A2NLSF_eval_poly(p, xlo, dd);
                if ylo < 0 as libc::c_int {
                    *NLSF.offset(0 as libc::c_int as isize) = 0 as libc::c_int;
                    p = Q.as_mut_ptr();
                    ylo = SKP_Silk_A2NLSF_eval_poly(p, xlo, dd);
                    root_ix = 1 as libc::c_int;
                } else {
                    root_ix = 0 as libc::c_int;
                }
                k = 1 as libc::c_int;
            }
        }
    };
}
