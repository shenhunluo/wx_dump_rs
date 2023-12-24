#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_biquad(
    mut in_0: *const libc::c_short,
    mut B: *const libc::c_short,
    mut A: *const libc::c_short,
    mut S: *mut libc::c_int,
    mut out: *mut libc::c_short,
    len: libc::c_int,
) {
    let mut k: libc::c_int = 0;
    let mut in16: libc::c_int = 0;
    let mut A0_neg: libc::c_int = 0;
    let mut A1_neg: libc::c_int = 0;
    let mut S0: libc::c_int = 0;
    let mut S1: libc::c_int = 0;
    let mut out32: libc::c_int = 0;
    let mut tmp32: libc::c_int = 0;
    S0 = *S.offset(0 as libc::c_int as isize);
    S1 = *S.offset(1 as libc::c_int as isize);
    A0_neg = -(*A.offset(0 as libc::c_int as isize) as libc::c_int);
    A1_neg = -(*A.offset(1 as libc::c_int as isize) as libc::c_int);
    k = 0 as libc::c_int;
    while k < len {
        in16 = *in_0.offset(k as isize) as libc::c_int;
        out32 = S0
            + in16 as libc::c_short as libc::c_int
                * *B.offset(0 as libc::c_int as isize) as libc::c_int;
        S0 = S1
            + in16 as libc::c_short as libc::c_int
                * *B.offset(1 as libc::c_int as isize) as libc::c_int;
        S0
            += (out32 >> 16 as libc::c_int) * A0_neg as libc::c_short as libc::c_int
                + ((out32 & 0xffff as libc::c_int)
                    * A0_neg as libc::c_short as libc::c_int >> 16 as libc::c_int)
                << 3 as libc::c_int;
        S1 = (out32 >> 16 as libc::c_int) * A1_neg as libc::c_short as libc::c_int
            + ((out32 & 0xffff as libc::c_int) * A1_neg as libc::c_short as libc::c_int
                >> 16 as libc::c_int) << 3 as libc::c_int;
        S1 = S1
            + in16 as libc::c_short as libc::c_int
                * *B.offset(2 as libc::c_int as isize) as libc::c_int;
        tmp32 = (if 13 as libc::c_int == 1 as libc::c_int {
            (out32 >> 1 as libc::c_int) + (out32 & 1 as libc::c_int)
        } else {
            (out32 >> 13 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                >> 1 as libc::c_int
        }) + 1 as libc::c_int;
        *out
            .offset(
                k as isize,
            ) = (if tmp32 > 0x7fff as libc::c_int {
            0x7fff as libc::c_int
        } else if tmp32 < 0x8000 as libc::c_int as libc::c_short as libc::c_int {
            0x8000 as libc::c_int as libc::c_short as libc::c_int
        } else {
            tmp32
        }) as libc::c_short;
        k += 1;
    }
    *S.offset(0 as libc::c_int as isize) = S0;
    *S.offset(1 as libc::c_int as isize) = S1;
}
