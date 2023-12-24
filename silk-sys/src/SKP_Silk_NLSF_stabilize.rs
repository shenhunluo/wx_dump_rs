#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]

use crate::SKP_Silk_sort::SKP_Silk_insertion_sort_increasing_all_values;
#[inline]
unsafe extern "C" fn SKP_max_int(mut a: libc::c_int, mut b: libc::c_int) -> libc::c_int {
    return if a > b { a } else { b };
}
#[inline]
unsafe extern "C" fn SKP_min_int(mut a: libc::c_int, mut b: libc::c_int) -> libc::c_int {
    return if a < b { a } else { b };
}
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_NLSF_stabilize(
    mut NLSF_Q15: *mut libc::c_int,
    mut NDeltaMin_Q15: *const libc::c_int,
    L: libc::c_int,
) {
    let mut center_freq_Q15: libc::c_int = 0;
    let mut diff_Q15: libc::c_int = 0;
    let mut min_center_Q15: libc::c_int = 0;
    let mut max_center_Q15: libc::c_int = 0;
    let mut min_diff_Q15: libc::c_int = 0;
    let mut loops: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut I: libc::c_int = 0 as libc::c_int;
    let mut k: libc::c_int = 0;
    loops = 0 as libc::c_int;
    while loops < 20 as libc::c_int {
        min_diff_Q15 = *NLSF_Q15.offset(0 as libc::c_int as isize)
            - *NDeltaMin_Q15.offset(0 as libc::c_int as isize);
        I = 0 as libc::c_int;
        i = 1 as libc::c_int;
        while i <= L - 1 as libc::c_int {
            diff_Q15 = *NLSF_Q15.offset(i as isize)
                - (*NLSF_Q15.offset((i - 1 as libc::c_int) as isize)
                    + *NDeltaMin_Q15.offset(i as isize));
            if diff_Q15 < min_diff_Q15 {
                min_diff_Q15 = diff_Q15;
                I = i;
            }
            i += 1;
        }
        diff_Q15 = ((1 as libc::c_int) << 15 as libc::c_int)
            - (*NLSF_Q15.offset((L - 1 as libc::c_int) as isize)
                + *NDeltaMin_Q15.offset(L as isize));
        if diff_Q15 < min_diff_Q15 {
            min_diff_Q15 = diff_Q15;
            I = L;
        }
        if min_diff_Q15 >= 0 as libc::c_int {
            return;
        }
        if I == 0 as libc::c_int {
            *NLSF_Q15
                .offset(
                    0 as libc::c_int as isize,
                ) = *NDeltaMin_Q15.offset(0 as libc::c_int as isize);
        } else if I == L {
            *NLSF_Q15
                .offset(
                    (L - 1 as libc::c_int) as isize,
                ) = ((1 as libc::c_int) << 15 as libc::c_int)
                - *NDeltaMin_Q15.offset(L as isize);
        } else {
            min_center_Q15 = 0 as libc::c_int;
            k = 0 as libc::c_int;
            while k < I {
                min_center_Q15 += *NDeltaMin_Q15.offset(k as isize);
                k += 1;
            }
            min_center_Q15 += *NDeltaMin_Q15.offset(I as isize) >> 1 as libc::c_int;
            max_center_Q15 = (1 as libc::c_int) << 15 as libc::c_int;
            k = L;
            while k > I {
                max_center_Q15 -= *NDeltaMin_Q15.offset(k as isize);
                k -= 1;
            }
            max_center_Q15
                -= *NDeltaMin_Q15.offset(I as isize)
                    - (*NDeltaMin_Q15.offset(I as isize) >> 1 as libc::c_int);
            center_freq_Q15 = if min_center_Q15 > max_center_Q15 {
                if (if 1 as libc::c_int == 1 as libc::c_int {
                    (*NLSF_Q15.offset((I - 1 as libc::c_int) as isize)
                        + *NLSF_Q15.offset(I as isize) >> 1 as libc::c_int)
                        + (*NLSF_Q15.offset((I - 1 as libc::c_int) as isize)
                            + *NLSF_Q15.offset(I as isize) & 1 as libc::c_int)
                } else {
                    (*NLSF_Q15.offset((I - 1 as libc::c_int) as isize)
                        + *NLSF_Q15.offset(I as isize)
                        >> 1 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                        >> 1 as libc::c_int
                }) > min_center_Q15
                {
                    min_center_Q15
                } else if (if 1 as libc::c_int == 1 as libc::c_int {
                    (*NLSF_Q15.offset((I - 1 as libc::c_int) as isize)
                        + *NLSF_Q15.offset(I as isize) >> 1 as libc::c_int)
                        + (*NLSF_Q15.offset((I - 1 as libc::c_int) as isize)
                            + *NLSF_Q15.offset(I as isize) & 1 as libc::c_int)
                } else {
                    (*NLSF_Q15.offset((I - 1 as libc::c_int) as isize)
                        + *NLSF_Q15.offset(I as isize)
                        >> 1 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                        >> 1 as libc::c_int
                }) < max_center_Q15
                {
                    max_center_Q15
                } else if 1 as libc::c_int == 1 as libc::c_int {
                    (*NLSF_Q15.offset((I - 1 as libc::c_int) as isize)
                        + *NLSF_Q15.offset(I as isize) >> 1 as libc::c_int)
                        + (*NLSF_Q15.offset((I - 1 as libc::c_int) as isize)
                            + *NLSF_Q15.offset(I as isize) & 1 as libc::c_int)
                } else {
                    (*NLSF_Q15.offset((I - 1 as libc::c_int) as isize)
                        + *NLSF_Q15.offset(I as isize)
                        >> 1 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                        >> 1 as libc::c_int
                }
            } else if (if 1 as libc::c_int == 1 as libc::c_int {
                (*NLSF_Q15.offset((I - 1 as libc::c_int) as isize)
                    + *NLSF_Q15.offset(I as isize) >> 1 as libc::c_int)
                    + (*NLSF_Q15.offset((I - 1 as libc::c_int) as isize)
                        + *NLSF_Q15.offset(I as isize) & 1 as libc::c_int)
            } else {
                (*NLSF_Q15.offset((I - 1 as libc::c_int) as isize)
                    + *NLSF_Q15.offset(I as isize)
                    >> 1 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                    >> 1 as libc::c_int
            }) > max_center_Q15
            {
                max_center_Q15
            } else if (if 1 as libc::c_int == 1 as libc::c_int {
                (*NLSF_Q15.offset((I - 1 as libc::c_int) as isize)
                    + *NLSF_Q15.offset(I as isize) >> 1 as libc::c_int)
                    + (*NLSF_Q15.offset((I - 1 as libc::c_int) as isize)
                        + *NLSF_Q15.offset(I as isize) & 1 as libc::c_int)
            } else {
                (*NLSF_Q15.offset((I - 1 as libc::c_int) as isize)
                    + *NLSF_Q15.offset(I as isize)
                    >> 1 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                    >> 1 as libc::c_int
            }) < min_center_Q15
            {
                min_center_Q15
            } else if 1 as libc::c_int == 1 as libc::c_int {
                (*NLSF_Q15.offset((I - 1 as libc::c_int) as isize)
                    + *NLSF_Q15.offset(I as isize) >> 1 as libc::c_int)
                    + (*NLSF_Q15.offset((I - 1 as libc::c_int) as isize)
                        + *NLSF_Q15.offset(I as isize) & 1 as libc::c_int)
            } else {
                (*NLSF_Q15.offset((I - 1 as libc::c_int) as isize)
                    + *NLSF_Q15.offset(I as isize)
                    >> 1 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                    >> 1 as libc::c_int
            };
            *NLSF_Q15
                .offset(
                    (I - 1 as libc::c_int) as isize,
                ) = center_freq_Q15
                - (*NDeltaMin_Q15.offset(I as isize) >> 1 as libc::c_int);
            *NLSF_Q15
                .offset(
                    I as isize,
                ) = *NLSF_Q15.offset((I - 1 as libc::c_int) as isize)
                + *NDeltaMin_Q15.offset(I as isize);
        }
        loops += 1;
    }
    if loops == 20 as libc::c_int {
        SKP_Silk_insertion_sort_increasing_all_values(
            &mut *NLSF_Q15.offset(0 as libc::c_int as isize),
            L,
        );
        *NLSF_Q15
            .offset(
                0 as libc::c_int as isize,
            ) = SKP_max_int(
            *NLSF_Q15.offset(0 as libc::c_int as isize),
            *NDeltaMin_Q15.offset(0 as libc::c_int as isize),
        );
        i = 1 as libc::c_int;
        while i < L {
            *NLSF_Q15
                .offset(
                    i as isize,
                ) = SKP_max_int(
                *NLSF_Q15.offset(i as isize),
                *NLSF_Q15.offset((i - 1 as libc::c_int) as isize)
                    + *NDeltaMin_Q15.offset(i as isize),
            );
            i += 1;
        }
        *NLSF_Q15
            .offset(
                (L - 1 as libc::c_int) as isize,
            ) = SKP_min_int(
            *NLSF_Q15.offset((L - 1 as libc::c_int) as isize),
            ((1 as libc::c_int) << 15 as libc::c_int) - *NDeltaMin_Q15.offset(L as isize),
        );
        i = L - 2 as libc::c_int;
        while i >= 0 as libc::c_int {
            *NLSF_Q15
                .offset(
                    i as isize,
                ) = SKP_min_int(
                *NLSF_Q15.offset(i as isize),
                *NLSF_Q15.offset((i + 1 as libc::c_int) as isize)
                    - *NDeltaMin_Q15.offset((i + 1 as libc::c_int) as isize),
            );
            i -= 1;
        }
    }
}
