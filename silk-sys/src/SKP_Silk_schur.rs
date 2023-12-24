#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
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
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_schur(
    mut rc_Q15: *mut libc::c_short,
    mut c: *const libc::c_int,
    order: libc::c_int,
) -> libc::c_int {
    let mut k: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut lz: libc::c_int = 0;
    let mut C: [[libc::c_int; 2]; 17] = [[0; 2]; 17];
    let mut Ctmp1: libc::c_int = 0;
    let mut Ctmp2: libc::c_int = 0;
    let mut rc_tmp_Q15: libc::c_int = 0;
    lz = SKP_Silk_CLZ32(*c.offset(0 as libc::c_int as isize));
    if lz < 2 as libc::c_int {
        k = 0 as libc::c_int;
        while k < order + 1 as libc::c_int {
            C[k
                as usize][1 as libc::c_int
                as usize] = *c.offset(k as isize) >> 1 as libc::c_int;
            C[k
                as usize][0 as libc::c_int
                as usize] = C[k as usize][1 as libc::c_int as usize];
            k += 1;
            k;
        }
    } else if lz > 2 as libc::c_int {
        lz -= 2 as libc::c_int;
        k = 0 as libc::c_int;
        while k < order + 1 as libc::c_int {
            C[k as usize][1 as libc::c_int as usize] = *c.offset(k as isize) << lz;
            C[k
                as usize][0 as libc::c_int
                as usize] = C[k as usize][1 as libc::c_int as usize];
            k += 1;
            k;
        }
    } else {
        k = 0 as libc::c_int;
        while k < order + 1 as libc::c_int {
            C[k as usize][1 as libc::c_int as usize] = *c.offset(k as isize);
            C[k
                as usize][0 as libc::c_int
                as usize] = C[k as usize][1 as libc::c_int as usize];
            k += 1;
            k;
        }
    }
    k = 0 as libc::c_int;
    while k < order {
        rc_tmp_Q15 = -(C[(k + 1 as libc::c_int) as usize][0 as libc::c_int as usize]
            / SKP_max_32(
                C[0 as libc::c_int as usize][1 as libc::c_int as usize]
                    >> 15 as libc::c_int,
                1 as libc::c_int,
            ));
        rc_tmp_Q15 = if rc_tmp_Q15 > 0x7fff as libc::c_int {
            0x7fff as libc::c_int
        } else if rc_tmp_Q15 < 0x8000 as libc::c_int as libc::c_short as libc::c_int {
            0x8000 as libc::c_int as libc::c_short as libc::c_int
        } else {
            rc_tmp_Q15
        };
        *rc_Q15.offset(k as isize) = rc_tmp_Q15 as libc::c_short;
        n = 0 as libc::c_int;
        while n < order - k {
            Ctmp1 = C[(n + k + 1 as libc::c_int) as usize][0 as libc::c_int as usize];
            Ctmp2 = C[n as usize][1 as libc::c_int as usize];
            C[(n + k + 1 as libc::c_int)
                as usize][0 as libc::c_int
                as usize] = Ctmp1
                + ((Ctmp2 << 1 as libc::c_int >> 16 as libc::c_int)
                    * rc_tmp_Q15 as libc::c_short as libc::c_int
                    + ((Ctmp2 << 1 as libc::c_int & 0xffff as libc::c_int)
                        * rc_tmp_Q15 as libc::c_short as libc::c_int
                        >> 16 as libc::c_int));
            C[n
                as usize][1 as libc::c_int
                as usize] = Ctmp2
                + ((Ctmp1 << 1 as libc::c_int >> 16 as libc::c_int)
                    * rc_tmp_Q15 as libc::c_short as libc::c_int
                    + ((Ctmp1 << 1 as libc::c_int & 0xffff as libc::c_int)
                        * rc_tmp_Q15 as libc::c_short as libc::c_int
                        >> 16 as libc::c_int));
            n += 1;
            n;
        }
        k += 1;
        k;
    }
    return C[0 as libc::c_int as usize][1 as libc::c_int as usize];
}
