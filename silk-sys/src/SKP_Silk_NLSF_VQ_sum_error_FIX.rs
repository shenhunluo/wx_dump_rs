#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_NLSF_VQ_sum_error_FIX(
    mut err_Q20: *mut libc::c_int,
    mut in_Q15: *const libc::c_int,
    mut w_Q6: *const libc::c_int,
    mut pCB_Q15: *const libc::c_short,
    N: libc::c_int,
    K: libc::c_int,
    LPC_order: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut diff_Q15: libc::c_int = 0;
    let mut sum_error: libc::c_int = 0;
    let mut Wtmp_Q6: libc::c_int = 0;
    let mut Wcpy_Q6: [libc::c_int; 8] = [0; 8];
    let mut cb_vec_Q15: *const libc::c_short = 0 as *const libc::c_short;
    m = 0 as libc::c_int;
    while m < LPC_order >> 1 as libc::c_int {
        Wcpy_Q6[m
            as usize] = *w_Q6.offset((2 as libc::c_int * m) as isize)
            | *w_Q6.offset((2 as libc::c_int * m + 1 as libc::c_int) as isize)
                << 16 as libc::c_int;
        m += 1;
        m;
    }
    n = 0 as libc::c_int;
    while n < N {
        cb_vec_Q15 = pCB_Q15;
        i = 0 as libc::c_int;
        while i < K {
            sum_error = 0 as libc::c_int;
            m = 0 as libc::c_int;
            while m < LPC_order {
                Wtmp_Q6 = Wcpy_Q6[(m >> 1 as libc::c_int) as usize];
                let fresh0 = cb_vec_Q15;
                cb_vec_Q15 = cb_vec_Q15.offset(1);
                diff_Q15 = *in_Q15.offset(m as isize) - *fresh0 as libc::c_int;
                sum_error = sum_error
                    + ((diff_Q15 as libc::c_short as libc::c_int
                        * diff_Q15 as libc::c_short as libc::c_int >> 16 as libc::c_int)
                        * Wtmp_Q6 as libc::c_short as libc::c_int
                        + ((diff_Q15 as libc::c_short as libc::c_int
                            * diff_Q15 as libc::c_short as libc::c_int
                            & 0xffff as libc::c_int)
                            * Wtmp_Q6 as libc::c_short as libc::c_int
                            >> 16 as libc::c_int));
                let fresh1 = cb_vec_Q15;
                cb_vec_Q15 = cb_vec_Q15.offset(1);
                diff_Q15 = *in_Q15.offset((m + 1 as libc::c_int) as isize)
                    - *fresh1 as libc::c_int;
                sum_error = sum_error
                    + (diff_Q15 as libc::c_short as libc::c_int
                        * diff_Q15 as libc::c_short as libc::c_int >> 16 as libc::c_int)
                        * (Wtmp_Q6 >> 16 as libc::c_int)
                    + ((diff_Q15 as libc::c_short as libc::c_int
                        * diff_Q15 as libc::c_short as libc::c_int
                        & 0xffff as libc::c_int) * (Wtmp_Q6 >> 16 as libc::c_int)
                        >> 16 as libc::c_int);
                m += 2 as libc::c_int;
            }
            *err_Q20.offset(i as isize) = sum_error;
            i += 1;
            i;
        }
        err_Q20 = err_Q20.offset(K as isize);
        in_Q15 = in_Q15.offset(LPC_order as isize);
        n += 1;
        n;
    }
}
