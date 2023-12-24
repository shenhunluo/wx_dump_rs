#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn SKP_Silk_NLSF_VQ_sum_error_FIX(
        err_Q20: *mut libc::c_int,
        in_Q15: *const libc::c_int,
        w_Q6: *const libc::c_int,
        pCB_Q15: *const libc::c_short,
        N: libc::c_int,
        K: libc::c_int,
        LPC_order: libc::c_int,
    );
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SKP_Silk_NLSF_CBS {
    pub nVectors: libc::c_int,
    pub CB_NLSF_Q15: *const libc::c_short,
    pub Rates_Q5: *const libc::c_short,
}
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_NLSF_VQ_rate_distortion_FIX(
    mut pRD_Q20: *mut libc::c_int,
    mut psNLSF_CBS: *const SKP_Silk_NLSF_CBS,
    mut in_Q15: *const libc::c_int,
    mut w_Q6: *const libc::c_int,
    mut rate_acc_Q5: *const libc::c_int,
    mu_Q15: libc::c_int,
    N: libc::c_int,
    LPC_order: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut pRD_vec_Q20: *mut libc::c_int = 0 as *mut libc::c_int;
    SKP_Silk_NLSF_VQ_sum_error_FIX(
        pRD_Q20,
        in_Q15,
        w_Q6,
        (*psNLSF_CBS).CB_NLSF_Q15,
        N,
        (*psNLSF_CBS).nVectors,
        LPC_order,
    );
    pRD_vec_Q20 = pRD_Q20;
    n = 0 as libc::c_int;
    while n < N {
        i = 0 as libc::c_int;
        while i < (*psNLSF_CBS).nVectors {
            *pRD_vec_Q20
                .offset(
                    i as isize,
                ) = *pRD_vec_Q20.offset(i as isize)
                + (*rate_acc_Q5.offset(n as isize)
                    + *((*psNLSF_CBS).Rates_Q5).offset(i as isize) as libc::c_int)
                    as libc::c_short as libc::c_int
                    * mu_Q15 as libc::c_short as libc::c_int;
            i += 1;
            i;
        }
        pRD_vec_Q20 = pRD_vec_Q20.offset((*psNLSF_CBS).nVectors as isize);
        n += 1;
        n;
    }
}
