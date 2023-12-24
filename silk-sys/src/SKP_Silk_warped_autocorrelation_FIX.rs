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
unsafe extern "C" fn SKP_Silk_CLZ64(mut in_0: int64_t) -> libc::c_int {
    let mut in_upper: libc::c_int = 0;
    in_upper = (in_0 >> 32 as libc::c_int) as libc::c_int;
    if in_upper == 0 as libc::c_int {
        return 32 as libc::c_int + SKP_Silk_CLZ32(in_0 as libc::c_int)
    } else {
        return SKP_Silk_CLZ32(in_upper)
    };
}
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_warped_autocorrelation_FIX(
    mut corr: *mut libc::c_int,
    mut scale: *mut libc::c_int,
    mut input: *const libc::c_short,
    warping_Q16: libc::c_short,
    length: libc::c_int,
    order: libc::c_int,
) {
    let mut n: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut lsh: libc::c_int = 0;
    let mut tmp1_QS: libc::c_int = 0;
    let mut tmp2_QS: libc::c_int = 0;
    let mut state_QS: [libc::c_int; 17] = [
        0 as libc::c_int,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let mut corr_QC: [int64_t; 17] = [
        0 as libc::c_int as int64_t,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    n = 0 as libc::c_int;
    while n < length {
        tmp1_QS = (*input.offset(n as isize) as libc::c_int) << 14 as libc::c_int;
        i = 0 as libc::c_int;
        while i < order {
            tmp2_QS = state_QS[i as usize]
                + ((state_QS[(i + 1 as libc::c_int) as usize] - tmp1_QS
                    >> 16 as libc::c_int) * warping_Q16 as libc::c_int
                    + ((state_QS[(i + 1 as libc::c_int) as usize] - tmp1_QS
                        & 0xffff as libc::c_int) * warping_Q16 as libc::c_int
                        >> 16 as libc::c_int));
            state_QS[i as usize] = tmp1_QS;
            corr_QC[i as usize]
                += tmp1_QS as int64_t * state_QS[0 as libc::c_int as usize] as int64_t
                    >> 2 as libc::c_int * 14 as libc::c_int - 10 as libc::c_int;
            tmp1_QS = state_QS[(i + 1 as libc::c_int) as usize]
                + ((state_QS[(i + 2 as libc::c_int) as usize] - tmp2_QS
                    >> 16 as libc::c_int) * warping_Q16 as libc::c_int
                    + ((state_QS[(i + 2 as libc::c_int) as usize] - tmp2_QS
                        & 0xffff as libc::c_int) * warping_Q16 as libc::c_int
                        >> 16 as libc::c_int));
            state_QS[(i + 1 as libc::c_int) as usize] = tmp2_QS;
            corr_QC[(i + 1 as libc::c_int) as usize]
                += tmp2_QS as int64_t * state_QS[0 as libc::c_int as usize] as int64_t
                    >> 2 as libc::c_int * 14 as libc::c_int - 10 as libc::c_int;
            i += 2 as libc::c_int;
        }
        state_QS[order as usize] = tmp1_QS;
        corr_QC[order as usize]
            += tmp1_QS as int64_t * state_QS[0 as libc::c_int as usize] as int64_t
                >> 2 as libc::c_int * 14 as libc::c_int - 10 as libc::c_int;
        n += 1;
        n;
    }
    lsh = SKP_Silk_CLZ64(corr_QC[0 as libc::c_int as usize]) - 35 as libc::c_int;
    lsh = if -(12 as libc::c_int) - 10 as libc::c_int
        > 30 as libc::c_int - 10 as libc::c_int
    {
        if lsh > -(12 as libc::c_int) - 10 as libc::c_int {
            -(12 as libc::c_int) - 10 as libc::c_int
        } else if lsh < 30 as libc::c_int - 10 as libc::c_int {
            30 as libc::c_int - 10 as libc::c_int
        } else {
            lsh
        }
    } else if lsh > 30 as libc::c_int - 10 as libc::c_int {
        30 as libc::c_int - 10 as libc::c_int
    } else if lsh < -(12 as libc::c_int) - 10 as libc::c_int {
        -(12 as libc::c_int) - 10 as libc::c_int
    } else {
        lsh
    };
    *scale = -(10 as libc::c_int + lsh);
    if lsh >= 0 as libc::c_int {
        i = 0 as libc::c_int;
        while i < order + 1 as libc::c_int {
            *corr.offset(i as isize) = (corr_QC[i as usize] << lsh) as libc::c_int;
            i += 1;
            i;
        }
    } else {
        i = 0 as libc::c_int;
        while i < order + 1 as libc::c_int {
            *corr.offset(i as isize) = (corr_QC[i as usize] >> -lsh) as libc::c_int;
            i += 1;
            i;
        }
    };
}
