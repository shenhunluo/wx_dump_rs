#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
static mut freq_table_Q16: [libc::c_short; 27] = [
    12111 as libc::c_int as libc::c_short,
    9804 as libc::c_int as libc::c_short,
    8235 as libc::c_int as libc::c_short,
    7100 as libc::c_int as libc::c_short,
    6239 as libc::c_int as libc::c_short,
    5565 as libc::c_int as libc::c_short,
    5022 as libc::c_int as libc::c_short,
    4575 as libc::c_int as libc::c_short,
    4202 as libc::c_int as libc::c_short,
    3885 as libc::c_int as libc::c_short,
    3612 as libc::c_int as libc::c_short,
    3375 as libc::c_int as libc::c_short,
    3167 as libc::c_int as libc::c_short,
    2984 as libc::c_int as libc::c_short,
    2820 as libc::c_int as libc::c_short,
    2674 as libc::c_int as libc::c_short,
    2542 as libc::c_int as libc::c_short,
    2422 as libc::c_int as libc::c_short,
    2313 as libc::c_int as libc::c_short,
    2214 as libc::c_int as libc::c_short,
    2123 as libc::c_int as libc::c_short,
    2038 as libc::c_int as libc::c_short,
    1961 as libc::c_int as libc::c_short,
    1889 as libc::c_int as libc::c_short,
    1822 as libc::c_int as libc::c_short,
    1760 as libc::c_int as libc::c_short,
    1702 as libc::c_int as libc::c_short,
];
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_apply_sine_window(
    mut px_win: *mut libc::c_short,
    mut px: *const libc::c_short,
    win_type: libc::c_int,
    length: libc::c_int,
) {
    let mut k: libc::c_int = 0;
    let mut f_Q16: libc::c_int = 0;
    let mut c_Q16: libc::c_int = 0;
    let mut S0_Q16: libc::c_int = 0;
    let mut S1_Q16: libc::c_int = 0;
    let mut px32: libc::c_int = 0;
    k = (length >> 2 as libc::c_int) - 4 as libc::c_int;
    f_Q16 = freq_table_Q16[k as usize] as libc::c_int;
    c_Q16 = (f_Q16 >> 16 as libc::c_int) * -f_Q16 as libc::c_short as libc::c_int
        + ((f_Q16 & 0xffff as libc::c_int) * -f_Q16 as libc::c_short as libc::c_int
            >> 16 as libc::c_int);
    if win_type == 1 as libc::c_int {
        S0_Q16 = 0 as libc::c_int;
        S1_Q16 = f_Q16 + (length >> 3 as libc::c_int);
    } else {
        S0_Q16 = (1 as libc::c_int) << 16 as libc::c_int;
        S1_Q16 = ((1 as libc::c_int) << 16 as libc::c_int) + (c_Q16 >> 1 as libc::c_int)
            + (length >> 4 as libc::c_int);
    }
    k = 0 as libc::c_int;
    while k < length {
        px32 = *(&*px.offset(k as isize) as *const libc::c_short as *mut libc::c_int);
        *px_win
            .offset(
                k as isize,
            ) = ((S0_Q16 + S1_Q16 >> 1 as libc::c_int >> 16 as libc::c_int)
            * px32 as libc::c_short as libc::c_int
            + ((S0_Q16 + S1_Q16 >> 1 as libc::c_int & 0xffff as libc::c_int)
                * px32 as libc::c_short as libc::c_int >> 16 as libc::c_int))
            as libc::c_short;
        *px_win
            .offset(
                (k + 1 as libc::c_int) as isize,
            ) = ((S1_Q16 >> 16 as libc::c_int) * (px32 >> 16 as libc::c_int)
            + ((S1_Q16 & 0xffff as libc::c_int) * (px32 >> 16 as libc::c_int)
                >> 16 as libc::c_int)) as libc::c_short;
        S0_Q16 = (S1_Q16 >> 16 as libc::c_int) * c_Q16 as libc::c_short as libc::c_int
            + ((S1_Q16 & 0xffff as libc::c_int) * c_Q16 as libc::c_short as libc::c_int
                >> 16 as libc::c_int) + (S1_Q16 << 1 as libc::c_int) - S0_Q16
            + 1 as libc::c_int;
        S0_Q16 = if S0_Q16 < (1 as libc::c_int) << 16 as libc::c_int {
            S0_Q16
        } else {
            (1 as libc::c_int) << 16 as libc::c_int
        };
        px32 = *(&*px.offset((k + 2 as libc::c_int) as isize) as *const libc::c_short
            as *mut libc::c_int);
        *px_win
            .offset(
                (k + 2 as libc::c_int) as isize,
            ) = ((S0_Q16 + S1_Q16 >> 1 as libc::c_int >> 16 as libc::c_int)
            * px32 as libc::c_short as libc::c_int
            + ((S0_Q16 + S1_Q16 >> 1 as libc::c_int & 0xffff as libc::c_int)
                * px32 as libc::c_short as libc::c_int >> 16 as libc::c_int))
            as libc::c_short;
        *px_win
            .offset(
                (k + 3 as libc::c_int) as isize,
            ) = ((S0_Q16 >> 16 as libc::c_int) * (px32 >> 16 as libc::c_int)
            + ((S0_Q16 & 0xffff as libc::c_int) * (px32 >> 16 as libc::c_int)
                >> 16 as libc::c_int)) as libc::c_short;
        S1_Q16 = (S0_Q16 >> 16 as libc::c_int) * c_Q16 as libc::c_short as libc::c_int
            + ((S0_Q16 & 0xffff as libc::c_int) * c_Q16 as libc::c_short as libc::c_int
                >> 16 as libc::c_int) + (S0_Q16 << 1 as libc::c_int) - S1_Q16;
        S1_Q16 = if S1_Q16 < (1 as libc::c_int) << 16 as libc::c_int {
            S1_Q16
        } else {
            (1 as libc::c_int) << 16 as libc::c_int
        };
        k += 4 as libc::c_int;
    }
}
