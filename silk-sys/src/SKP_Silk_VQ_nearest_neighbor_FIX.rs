#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_VQ_WMat_EC_FIX(
    mut ind: *mut libc::c_int,
    mut rate_dist_Q14: *mut libc::c_int,
    mut in_Q14: *const libc::c_short,
    mut W_Q18: *const libc::c_int,
    mut cb_Q14: *const libc::c_short,
    mut cl_Q6: *const libc::c_short,
    mu_Q8: libc::c_int,
    mut L: libc::c_int,
) {
    let mut k: libc::c_int = 0;
    let mut cb_row_Q14: *const libc::c_short = 0 as *const libc::c_short;
    let mut sum1_Q14: libc::c_int = 0;
    let mut sum2_Q16: libc::c_int = 0;
    let mut diff_Q14_01: libc::c_int = 0;
    let mut diff_Q14_23: libc::c_int = 0;
    let mut diff_Q14_4: libc::c_int = 0;
    *rate_dist_Q14 = 0x7fffffff as libc::c_int;
    cb_row_Q14 = cb_Q14;
    k = 0 as libc::c_int;
    while k < L {
        diff_Q14_01 = (*in_Q14.offset(0 as libc::c_int as isize) as libc::c_int
            - *cb_row_Q14.offset(0 as libc::c_int as isize) as libc::c_int)
            as libc::c_ushort as libc::c_int
            | (*in_Q14.offset(1 as libc::c_int as isize) as libc::c_int
                - *cb_row_Q14.offset(1 as libc::c_int as isize) as libc::c_int)
                << 16 as libc::c_int;
        diff_Q14_23 = (*in_Q14.offset(2 as libc::c_int as isize) as libc::c_int
            - *cb_row_Q14.offset(2 as libc::c_int as isize) as libc::c_int)
            as libc::c_ushort as libc::c_int
            | (*in_Q14.offset(3 as libc::c_int as isize) as libc::c_int
                - *cb_row_Q14.offset(3 as libc::c_int as isize) as libc::c_int)
                << 16 as libc::c_int;
        diff_Q14_4 = *in_Q14.offset(4 as libc::c_int as isize) as libc::c_int
            - *cb_row_Q14.offset(4 as libc::c_int as isize) as libc::c_int;
        sum1_Q14 = mu_Q8 as libc::c_short as libc::c_int
            * *cl_Q6.offset(k as isize) as libc::c_int;
        sum2_Q16 = (*W_Q18.offset(1 as libc::c_int as isize) >> 16 as libc::c_int)
            * (diff_Q14_01 >> 16 as libc::c_int)
            + ((*W_Q18.offset(1 as libc::c_int as isize) & 0xffff as libc::c_int)
                * (diff_Q14_01 >> 16 as libc::c_int) >> 16 as libc::c_int);
        sum2_Q16 = sum2_Q16
            + ((*W_Q18.offset(2 as libc::c_int as isize) >> 16 as libc::c_int)
                * diff_Q14_23 as libc::c_short as libc::c_int
                + ((*W_Q18.offset(2 as libc::c_int as isize) & 0xffff as libc::c_int)
                    * diff_Q14_23 as libc::c_short as libc::c_int >> 16 as libc::c_int));
        sum2_Q16 = sum2_Q16
            + (*W_Q18.offset(3 as libc::c_int as isize) >> 16 as libc::c_int)
                * (diff_Q14_23 >> 16 as libc::c_int)
            + ((*W_Q18.offset(3 as libc::c_int as isize) & 0xffff as libc::c_int)
                * (diff_Q14_23 >> 16 as libc::c_int) >> 16 as libc::c_int);
        sum2_Q16 = sum2_Q16
            + ((*W_Q18.offset(4 as libc::c_int as isize) >> 16 as libc::c_int)
                * diff_Q14_4 as libc::c_short as libc::c_int
                + ((*W_Q18.offset(4 as libc::c_int as isize) & 0xffff as libc::c_int)
                    * diff_Q14_4 as libc::c_short as libc::c_int >> 16 as libc::c_int));
        sum2_Q16 = sum2_Q16 << 1 as libc::c_int;
        sum2_Q16 = sum2_Q16
            + ((*W_Q18.offset(0 as libc::c_int as isize) >> 16 as libc::c_int)
                * diff_Q14_01 as libc::c_short as libc::c_int
                + ((*W_Q18.offset(0 as libc::c_int as isize) & 0xffff as libc::c_int)
                    * diff_Q14_01 as libc::c_short as libc::c_int >> 16 as libc::c_int));
        sum1_Q14 = sum1_Q14
            + ((sum2_Q16 >> 16 as libc::c_int)
                * diff_Q14_01 as libc::c_short as libc::c_int
                + ((sum2_Q16 & 0xffff as libc::c_int)
                    * diff_Q14_01 as libc::c_short as libc::c_int >> 16 as libc::c_int));
        sum2_Q16 = (*W_Q18.offset(7 as libc::c_int as isize) >> 16 as libc::c_int)
            * diff_Q14_23 as libc::c_short as libc::c_int
            + ((*W_Q18.offset(7 as libc::c_int as isize) & 0xffff as libc::c_int)
                * diff_Q14_23 as libc::c_short as libc::c_int >> 16 as libc::c_int);
        sum2_Q16 = sum2_Q16
            + (*W_Q18.offset(8 as libc::c_int as isize) >> 16 as libc::c_int)
                * (diff_Q14_23 >> 16 as libc::c_int)
            + ((*W_Q18.offset(8 as libc::c_int as isize) & 0xffff as libc::c_int)
                * (diff_Q14_23 >> 16 as libc::c_int) >> 16 as libc::c_int);
        sum2_Q16 = sum2_Q16
            + ((*W_Q18.offset(9 as libc::c_int as isize) >> 16 as libc::c_int)
                * diff_Q14_4 as libc::c_short as libc::c_int
                + ((*W_Q18.offset(9 as libc::c_int as isize) & 0xffff as libc::c_int)
                    * diff_Q14_4 as libc::c_short as libc::c_int >> 16 as libc::c_int));
        sum2_Q16 = sum2_Q16 << 1 as libc::c_int;
        sum2_Q16 = sum2_Q16
            + (*W_Q18.offset(6 as libc::c_int as isize) >> 16 as libc::c_int)
                * (diff_Q14_01 >> 16 as libc::c_int)
            + ((*W_Q18.offset(6 as libc::c_int as isize) & 0xffff as libc::c_int)
                * (diff_Q14_01 >> 16 as libc::c_int) >> 16 as libc::c_int);
        sum1_Q14 = sum1_Q14
            + (sum2_Q16 >> 16 as libc::c_int) * (diff_Q14_01 >> 16 as libc::c_int)
            + ((sum2_Q16 & 0xffff as libc::c_int) * (diff_Q14_01 >> 16 as libc::c_int)
                >> 16 as libc::c_int);
        sum2_Q16 = (*W_Q18.offset(13 as libc::c_int as isize) >> 16 as libc::c_int)
            * (diff_Q14_23 >> 16 as libc::c_int)
            + ((*W_Q18.offset(13 as libc::c_int as isize) & 0xffff as libc::c_int)
                * (diff_Q14_23 >> 16 as libc::c_int) >> 16 as libc::c_int);
        sum2_Q16 = sum2_Q16
            + ((*W_Q18.offset(14 as libc::c_int as isize) >> 16 as libc::c_int)
                * diff_Q14_4 as libc::c_short as libc::c_int
                + ((*W_Q18.offset(14 as libc::c_int as isize) & 0xffff as libc::c_int)
                    * diff_Q14_4 as libc::c_short as libc::c_int >> 16 as libc::c_int));
        sum2_Q16 = sum2_Q16 << 1 as libc::c_int;
        sum2_Q16 = sum2_Q16
            + ((*W_Q18.offset(12 as libc::c_int as isize) >> 16 as libc::c_int)
                * diff_Q14_23 as libc::c_short as libc::c_int
                + ((*W_Q18.offset(12 as libc::c_int as isize) & 0xffff as libc::c_int)
                    * diff_Q14_23 as libc::c_short as libc::c_int >> 16 as libc::c_int));
        sum1_Q14 = sum1_Q14
            + ((sum2_Q16 >> 16 as libc::c_int)
                * diff_Q14_23 as libc::c_short as libc::c_int
                + ((sum2_Q16 & 0xffff as libc::c_int)
                    * diff_Q14_23 as libc::c_short as libc::c_int >> 16 as libc::c_int));
        sum2_Q16 = (*W_Q18.offset(19 as libc::c_int as isize) >> 16 as libc::c_int)
            * diff_Q14_4 as libc::c_short as libc::c_int
            + ((*W_Q18.offset(19 as libc::c_int as isize) & 0xffff as libc::c_int)
                * diff_Q14_4 as libc::c_short as libc::c_int >> 16 as libc::c_int);
        sum2_Q16 = sum2_Q16 << 1 as libc::c_int;
        sum2_Q16 = sum2_Q16
            + (*W_Q18.offset(18 as libc::c_int as isize) >> 16 as libc::c_int)
                * (diff_Q14_23 >> 16 as libc::c_int)
            + ((*W_Q18.offset(18 as libc::c_int as isize) & 0xffff as libc::c_int)
                * (diff_Q14_23 >> 16 as libc::c_int) >> 16 as libc::c_int);
        sum1_Q14 = sum1_Q14
            + (sum2_Q16 >> 16 as libc::c_int) * (diff_Q14_23 >> 16 as libc::c_int)
            + ((sum2_Q16 & 0xffff as libc::c_int) * (diff_Q14_23 >> 16 as libc::c_int)
                >> 16 as libc::c_int);
        sum2_Q16 = (*W_Q18.offset(24 as libc::c_int as isize) >> 16 as libc::c_int)
            * diff_Q14_4 as libc::c_short as libc::c_int
            + ((*W_Q18.offset(24 as libc::c_int as isize) & 0xffff as libc::c_int)
                * diff_Q14_4 as libc::c_short as libc::c_int >> 16 as libc::c_int);
        sum1_Q14 = sum1_Q14
            + ((sum2_Q16 >> 16 as libc::c_int)
                * diff_Q14_4 as libc::c_short as libc::c_int
                + ((sum2_Q16 & 0xffff as libc::c_int)
                    * diff_Q14_4 as libc::c_short as libc::c_int >> 16 as libc::c_int));
        if sum1_Q14 < *rate_dist_Q14 {
            *rate_dist_Q14 = sum1_Q14;
            *ind = k;
        }
        cb_row_Q14 = cb_row_Q14.offset(5 as libc::c_int as isize);
        k += 1;
        k;
    }
}
