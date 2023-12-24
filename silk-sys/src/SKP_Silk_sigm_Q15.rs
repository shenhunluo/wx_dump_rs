#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
static mut sigm_LUT_slope_Q10: [libc::c_int; 6] = [
    237 as libc::c_int,
    153 as libc::c_int,
    73 as libc::c_int,
    30 as libc::c_int,
    12 as libc::c_int,
    7 as libc::c_int,
];
static mut sigm_LUT_pos_Q15: [libc::c_int; 6] = [
    16384 as libc::c_int,
    23955 as libc::c_int,
    28861 as libc::c_int,
    31213 as libc::c_int,
    32178 as libc::c_int,
    32548 as libc::c_int,
];
static mut sigm_LUT_neg_Q15: [libc::c_int; 6] = [
    16384 as libc::c_int,
    8812 as libc::c_int,
    3906 as libc::c_int,
    1554 as libc::c_int,
    589 as libc::c_int,
    219 as libc::c_int,
];
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_sigm_Q15(mut in_Q5: libc::c_int) -> libc::c_int {
    let mut ind: libc::c_int = 0;
    if in_Q5 < 0 as libc::c_int {
        in_Q5 = -in_Q5;
        if in_Q5 >= 6 as libc::c_int * 32 as libc::c_int {
            return 0 as libc::c_int
        } else {
            ind = in_Q5 >> 5 as libc::c_int;
            return sigm_LUT_neg_Q15[ind as usize]
                - sigm_LUT_slope_Q10[ind as usize] as libc::c_short as libc::c_int
                    * (in_Q5 & 0x1f as libc::c_int) as libc::c_short as libc::c_int;
        }
    } else if in_Q5 >= 6 as libc::c_int * 32 as libc::c_int {
        return 32767 as libc::c_int
    } else {
        ind = in_Q5 >> 5 as libc::c_int;
        return sigm_LUT_pos_Q15[ind as usize]
            + sigm_LUT_slope_Q10[ind as usize] as libc::c_short as libc::c_int
                * (in_Q5 & 0x1f as libc::c_int) as libc::c_short as libc::c_int;
    };
}
