#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
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
unsafe extern "C" fn SKP_Silk_CLZ64(mut in_0: int64_t) -> libc::c_int {
    let mut in_upper: libc::c_int = 0;
    in_upper = (in_0 >> 32 as libc::c_int) as libc::c_int;
    if in_upper == 0 as libc::c_int {
        return 32 as libc::c_int + SKP_Silk_CLZ32(in_0 as libc::c_int)
    } else {
        return SKP_Silk_CLZ32(in_upper)
    };
}
#[inline]
unsafe extern "C" fn SKP_min_int(mut a: libc::c_int, mut b: libc::c_int) -> libc::c_int {
    return if a < b { a } else { b };
}
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_autocorr(
    mut results: *mut libc::c_int,
    mut scale: *mut libc::c_int,
    mut inputData: *const libc::c_short,
    inputDataSize: libc::c_int,
    correlationCount: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut lz: libc::c_int = 0;
    let mut nRightShifts: libc::c_int = 0;
    let mut corrCount: libc::c_int = 0;
    let mut corr64: int64_t = 0;
    corrCount = SKP_min_int(inputDataSize, correlationCount);
    corr64 = SKP_Silk_inner_prod16_aligned_64(inputData, inputData, inputDataSize);
    corr64 += 1 as libc::c_int as int64_t;
    lz = SKP_Silk_CLZ64(corr64);
    nRightShifts = 35 as libc::c_int - lz;
    *scale = nRightShifts;
    if nRightShifts <= 0 as libc::c_int {
        *results
            .offset(
                0 as libc::c_int as isize,
            ) = (corr64 as libc::c_int) << -nRightShifts;
        i = 1 as libc::c_int;
        while i < corrCount {
            *results
                .offset(
                    i as isize,
                ) = SKP_Silk_inner_prod_aligned(
                inputData,
                inputData.offset(i as isize),
                inputDataSize - i,
            ) << -nRightShifts;
            i += 1;
            i;
        }
    } else {
        *results
            .offset(0 as libc::c_int as isize) = (corr64 >> nRightShifts) as libc::c_int;
        i = 1 as libc::c_int;
        while i < corrCount {
            *results
                .offset(
                    i as isize,
                ) = (SKP_Silk_inner_prod16_aligned_64(
                inputData,
                inputData.offset(i as isize),
                inputDataSize - i,
            ) >> nRightShifts) as libc::c_int;
            i += 1;
            i;
        }
    };
}
