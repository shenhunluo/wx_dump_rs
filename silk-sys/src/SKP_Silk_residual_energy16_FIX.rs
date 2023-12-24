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
unsafe extern "C" fn SKP_min_int(mut a: libc::c_int, mut b: libc::c_int) -> libc::c_int {
    return if a < b { a } else { b };
}
#[inline]
unsafe extern "C" fn SKP_max_int(mut a: libc::c_int, mut b: libc::c_int) -> libc::c_int {
    return if a > b { a } else { b };
}
#[inline]
unsafe extern "C" fn SKP_max_32(mut a: libc::c_int, mut b: libc::c_int) -> libc::c_int {
    return if a > b { a } else { b };
}
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_residual_energy16_covar_FIX(
    mut c: *const libc::c_short,
    mut wXX: *const libc::c_int,
    mut wXx: *const libc::c_int,
    mut wxx: libc::c_int,
    mut D: libc::c_int,
    mut cQ: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut lshifts: libc::c_int = 0;
    let mut Qxtra: libc::c_int = 0;
    let mut c_max: libc::c_int = 0;
    let mut w_max: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp2: libc::c_int = 0;
    let mut nrg: libc::c_int = 0;
    let mut cn: [libc::c_int; 16] = [0; 16];
    let mut pRow: *const libc::c_int = 0 as *const libc::c_int;
    lshifts = 16 as libc::c_int - cQ;
    Qxtra = lshifts;
    c_max = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < D {
        c_max = SKP_max_32(
            c_max,
            if *c.offset(i as isize) as libc::c_int > 0 as libc::c_int {
                *c.offset(i as isize) as libc::c_int
            } else {
                -(*c.offset(i as isize) as libc::c_int)
            },
        );
        i += 1;
        i;
    }
    Qxtra = SKP_min_int(Qxtra, SKP_Silk_CLZ32(c_max) - 17 as libc::c_int);
    w_max = SKP_max_32(
        *wXX.offset(0 as libc::c_int as isize),
        *wXX.offset((D * D - 1 as libc::c_int) as isize),
    );
    Qxtra = SKP_min_int(
        Qxtra,
        SKP_Silk_CLZ32(
            D
                * ((w_max >> 16 as libc::c_int) * c_max as libc::c_short as libc::c_int
                    + ((w_max & 0xffff as libc::c_int)
                        * c_max as libc::c_short as libc::c_int >> 16 as libc::c_int)
                    >> 4 as libc::c_int),
        ) - 5 as libc::c_int,
    );
    Qxtra = SKP_max_int(Qxtra, 0 as libc::c_int);
    i = 0 as libc::c_int;
    while i < D {
        cn[i as usize] = (*c.offset(i as isize) as libc::c_int) << Qxtra;
        i += 1;
        i;
    }
    lshifts -= Qxtra;
    tmp = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < D {
        tmp = tmp
            + ((*wXx.offset(i as isize) >> 16 as libc::c_int)
                * cn[i as usize] as libc::c_short as libc::c_int
                + ((*wXx.offset(i as isize) & 0xffff as libc::c_int)
                    * cn[i as usize] as libc::c_short as libc::c_int
                    >> 16 as libc::c_int));
        i += 1;
        i;
    }
    nrg = (wxx >> 1 as libc::c_int + lshifts) - tmp;
    tmp2 = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < D {
        tmp = 0 as libc::c_int;
        pRow = &*wXX.offset((i * D) as isize) as *const libc::c_int;
        j = i + 1 as libc::c_int;
        while j < D {
            tmp = tmp
                + ((*pRow.offset(j as isize) >> 16 as libc::c_int)
                    * cn[j as usize] as libc::c_short as libc::c_int
                    + ((*pRow.offset(j as isize) & 0xffff as libc::c_int)
                        * cn[j as usize] as libc::c_short as libc::c_int
                        >> 16 as libc::c_int));
            j += 1;
            j;
        }
        tmp = tmp
            + ((*pRow.offset(i as isize) >> 1 as libc::c_int >> 16 as libc::c_int)
                * cn[i as usize] as libc::c_short as libc::c_int
                + ((*pRow.offset(i as isize) >> 1 as libc::c_int & 0xffff as libc::c_int)
                    * cn[i as usize] as libc::c_short as libc::c_int
                    >> 16 as libc::c_int));
        tmp2 = tmp2
            + ((tmp >> 16 as libc::c_int)
                * cn[i as usize] as libc::c_short as libc::c_int
                + ((tmp & 0xffff as libc::c_int)
                    * cn[i as usize] as libc::c_short as libc::c_int
                    >> 16 as libc::c_int));
        i += 1;
        i;
    }
    nrg = nrg + (tmp2 << lshifts);
    if nrg < 1 as libc::c_int {
        nrg = 1 as libc::c_int;
    } else if nrg > 0x7fffffff as libc::c_int >> lshifts + 2 as libc::c_int {
        nrg = 0x7fffffff as libc::c_int >> 1 as libc::c_int;
    } else {
        nrg = nrg << lshifts + 1 as libc::c_int;
    }
    return nrg;
}
