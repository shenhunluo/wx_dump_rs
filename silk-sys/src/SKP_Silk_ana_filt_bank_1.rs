#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
static mut A_fb1_20: [libc::c_short; 1] = [
    ((5394 as libc::c_int) << 1 as libc::c_int) as libc::c_short,
];
static mut A_fb1_21: [libc::c_short; 1] = [
    ((20623 as libc::c_int) << 1 as libc::c_int) as libc::c_short,
];
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_ana_filt_bank_1(
    mut in_0: *const libc::c_short,
    mut S: *mut libc::c_int,
    mut outL: *mut libc::c_short,
    mut outH: *mut libc::c_short,
    mut scratch: *mut libc::c_int,
    N: libc::c_int,
) {
    let mut k: libc::c_int = 0;
    let mut N2: libc::c_int = N >> 1 as libc::c_int;
    let mut in32: libc::c_int = 0;
    let mut X: libc::c_int = 0;
    let mut Y: libc::c_int = 0;
    let mut out_1: libc::c_int = 0;
    let mut out_2: libc::c_int = 0;
    k = 0 as libc::c_int;
    while k < N2 {
        in32 = (*in_0.offset((2 as libc::c_int * k) as isize) as libc::c_int)
            << 10 as libc::c_int;
        Y = in32 - *S.offset(0 as libc::c_int as isize);
        X = Y
            + ((Y >> 16 as libc::c_int)
                * A_fb1_21[0 as libc::c_int as usize] as libc::c_int
                + ((Y & 0xffff as libc::c_int)
                    * A_fb1_21[0 as libc::c_int as usize] as libc::c_int
                    >> 16 as libc::c_int));
        out_1 = *S.offset(0 as libc::c_int as isize) + X;
        *S.offset(0 as libc::c_int as isize) = in32 + X;
        in32 = (*in_0.offset((2 as libc::c_int * k + 1 as libc::c_int) as isize)
            as libc::c_int) << 10 as libc::c_int;
        Y = in32 - *S.offset(1 as libc::c_int as isize);
        X = (Y >> 16 as libc::c_int) * A_fb1_20[0 as libc::c_int as usize] as libc::c_int
            + ((Y & 0xffff as libc::c_int)
                * A_fb1_20[0 as libc::c_int as usize] as libc::c_int
                >> 16 as libc::c_int);
        out_2 = *S.offset(1 as libc::c_int as isize) + X;
        *S.offset(1 as libc::c_int as isize) = in32 + X;
        *outL
            .offset(
                k as isize,
            ) = (if (if 11 as libc::c_int == 1 as libc::c_int {
            (out_2 + out_1 >> 1 as libc::c_int) + (out_2 + out_1 & 1 as libc::c_int)
        } else {
            (out_2 + out_1 >> 11 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                >> 1 as libc::c_int
        }) > 0x7fff as libc::c_int
        {
            0x7fff as libc::c_int
        } else if (if 11 as libc::c_int == 1 as libc::c_int {
            (out_2 + out_1 >> 1 as libc::c_int) + (out_2 + out_1 & 1 as libc::c_int)
        } else {
            (out_2 + out_1 >> 11 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                >> 1 as libc::c_int
        }) < 0x8000 as libc::c_int as libc::c_short as libc::c_int
        {
            0x8000 as libc::c_int as libc::c_short as libc::c_int
        } else if 11 as libc::c_int == 1 as libc::c_int {
            (out_2 + out_1 >> 1 as libc::c_int) + (out_2 + out_1 & 1 as libc::c_int)
        } else {
            (out_2 + out_1 >> 11 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                >> 1 as libc::c_int
        }) as libc::c_short;
        *outH
            .offset(
                k as isize,
            ) = (if (if 11 as libc::c_int == 1 as libc::c_int {
            (out_2 - out_1 >> 1 as libc::c_int) + (out_2 - out_1 & 1 as libc::c_int)
        } else {
            (out_2 - out_1 >> 11 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                >> 1 as libc::c_int
        }) > 0x7fff as libc::c_int
        {
            0x7fff as libc::c_int
        } else if (if 11 as libc::c_int == 1 as libc::c_int {
            (out_2 - out_1 >> 1 as libc::c_int) + (out_2 - out_1 & 1 as libc::c_int)
        } else {
            (out_2 - out_1 >> 11 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                >> 1 as libc::c_int
        }) < 0x8000 as libc::c_int as libc::c_short as libc::c_int
        {
            0x8000 as libc::c_int as libc::c_short as libc::c_int
        } else if 11 as libc::c_int == 1 as libc::c_int {
            (out_2 - out_1 >> 1 as libc::c_int) + (out_2 - out_1 & 1 as libc::c_int)
        } else {
            (out_2 - out_1 >> 11 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                >> 1 as libc::c_int
        }) as libc::c_short;
        k += 1;
        k;
    }
}
