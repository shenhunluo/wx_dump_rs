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
unsafe extern "C" fn SKP_Silk_CLZ_FRAC(
    mut in_0: libc::c_int,
    mut lz: *mut libc::c_int,
    mut frac_Q7: *mut libc::c_int,
) {
    let mut lzeros: libc::c_int = SKP_Silk_CLZ32(in_0);
    *lz = lzeros;
    *frac_Q7 = SKP_ROR32(in_0, 24 as libc::c_int - lzeros) & 0x7f as libc::c_int;
}
#[inline]
unsafe extern "C" fn SKP_ROR32(
    mut a32: libc::c_int,
    mut rot: libc::c_int,
) -> libc::c_int {
    let mut x: libc::c_uint = a32 as libc::c_uint;
    let mut r: libc::c_uint = rot as libc::c_uint;
    let mut m: libc::c_uint = -rot as libc::c_uint;
    if rot <= 0 as libc::c_int {
        return (x << m | x >> (32 as libc::c_int as libc::c_uint).wrapping_sub(m))
            as libc::c_int
    } else {
        return (x << (32 as libc::c_int as libc::c_uint).wrapping_sub(r) | x >> r)
            as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_lin2log(inLin: libc::c_int) -> libc::c_int {
    let mut lz: libc::c_int = 0;
    let mut frac_Q7: libc::c_int = 0;
    SKP_Silk_CLZ_FRAC(inLin, &mut lz, &mut frac_Q7);
    return (31 as libc::c_int - lz << 7 as libc::c_int)
        + (frac_Q7
            + ((frac_Q7 * (128 as libc::c_int - frac_Q7) >> 16 as libc::c_int)
                * 179 as libc::c_int as libc::c_short as libc::c_int
                + ((frac_Q7 * (128 as libc::c_int - frac_Q7) & 0xffff as libc::c_int)
                    * 179 as libc::c_int as libc::c_short as libc::c_int
                    >> 16 as libc::c_int)));
}
