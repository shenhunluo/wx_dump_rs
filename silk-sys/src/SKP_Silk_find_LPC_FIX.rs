#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn SKP_Silk_LPC_analysis_filter(
        in_0: *const libc::c_short,
        B: *const libc::c_short,
        S: *mut libc::c_short,
        out: *mut libc::c_short,
        len: libc::c_int,
        Order: libc::c_int,
    );
    fn SKP_Silk_bwexpander_32(
        ar: *mut libc::c_int,
        d: libc::c_int,
        chirp_Q16: libc::c_int,
    );
    fn SKP_Silk_sum_sqr_shift(
        energy: *mut libc::c_int,
        shift: *mut libc::c_int,
        x: *const libc::c_short,
        len: libc::c_int,
    );
    fn SKP_Silk_A2NLSF(NLSF: *mut libc::c_int, a_Q16: *mut libc::c_int, d: libc::c_int);
    fn SKP_Silk_burg_modified(
        res_nrg: *mut libc::c_int,
        res_nrgQ: *mut libc::c_int,
        A_Q16: *mut libc::c_int,
        x: *const libc::c_short,
        subfr_length: libc::c_int,
        nb_subfr: libc::c_int,
        WhiteNoiseFrac_Q32: libc::c_int,
        D: libc::c_int,
    );
    fn SKP_Silk_NLSF2A_stable(
        pAR_Q12: *mut libc::c_short,
        pNLSF: *const libc::c_int,
        LPC_order: libc::c_int,
    );
    fn SKP_Silk_interpolate(
        xi: *mut libc::c_int,
        x0: *const libc::c_int,
        x1: *const libc::c_int,
        ifact_Q2: libc::c_int,
        d: libc::c_int,
    );
}
pub type __int64_t = libc::c_longlong;
pub type int64_t = __int64_t;
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_find_LPC_FIX(
    mut NLSF_Q15: *mut libc::c_int,
    mut interpIndex: *mut libc::c_int,
    mut prev_NLSFq_Q15: *const libc::c_int,
    useInterpolatedNLSFs: libc::c_int,
    LPC_order: libc::c_int,
    mut x: *const libc::c_short,
    subfr_length: libc::c_int,
) {
    let mut k: libc::c_int = 0;
    let mut a_Q16: [libc::c_int; 16] = [0; 16];
    let mut isInterpLower: libc::c_int = 0;
    let mut shift: libc::c_int = 0;
    let mut S: [libc::c_short; 16] = [0; 16];
    let mut res_nrg0: libc::c_int = 0;
    let mut res_nrg1: libc::c_int = 0;
    let mut rshift0: libc::c_int = 0;
    let mut rshift1: libc::c_int = 0;
    let mut a_tmp_Q16: [libc::c_int; 16] = [0; 16];
    let mut res_nrg_interp: libc::c_int = 0;
    let mut res_nrg: libc::c_int = 0;
    let mut res_tmp_nrg: libc::c_int = 0;
    let mut res_nrg_interp_Q: libc::c_int = 0;
    let mut res_nrg_Q: libc::c_int = 0;
    let mut res_tmp_nrg_Q: libc::c_int = 0;
    let mut a_tmp_Q12: [libc::c_short; 16] = [0; 16];
    let mut NLSF0_Q15: [libc::c_int; 16] = [0; 16];
    let mut LPC_res: [libc::c_short; 272] = [0; 272];
    *interpIndex = 4 as libc::c_int;
    SKP_Silk_burg_modified(
        &mut res_nrg,
        &mut res_nrg_Q,
        a_Q16.as_mut_ptr(),
        x,
        subfr_length,
        4 as libc::c_int,
        ((2.5e-5f32
            * ((1 as libc::c_int as int64_t) << 32 as libc::c_int) as libc::c_float)
            as libc::c_double + 0.5f64) as libc::c_int,
        LPC_order,
    );
    SKP_Silk_bwexpander_32(
        a_Q16.as_mut_ptr(),
        LPC_order,
        ((0.99995f32
            * ((1 as libc::c_int as int64_t) << 16 as libc::c_int) as libc::c_float)
            as libc::c_double + 0.5f64) as libc::c_int,
    );
    if useInterpolatedNLSFs == 1 as libc::c_int {
        SKP_Silk_burg_modified(
            &mut res_tmp_nrg,
            &mut res_tmp_nrg_Q,
            a_tmp_Q16.as_mut_ptr(),
            x.offset(((4 as libc::c_int >> 1 as libc::c_int) * subfr_length) as isize),
            subfr_length,
            4 as libc::c_int >> 1 as libc::c_int,
            ((2.5e-5f32
                * ((1 as libc::c_int as int64_t) << 32 as libc::c_int) as libc::c_float)
                as libc::c_double + 0.5f64) as libc::c_int,
            LPC_order,
        );
        SKP_Silk_bwexpander_32(
            a_tmp_Q16.as_mut_ptr(),
            LPC_order,
            ((0.99995f32
                * ((1 as libc::c_int as int64_t) << 16 as libc::c_int) as libc::c_float)
                as libc::c_double + 0.5f64) as libc::c_int,
        );
        shift = res_tmp_nrg_Q - res_nrg_Q;
        if shift >= 0 as libc::c_int {
            if shift < 32 as libc::c_int {
                res_nrg = res_nrg - (res_tmp_nrg >> shift);
            }
        } else {
            res_nrg = (res_nrg >> -shift) - res_tmp_nrg;
            res_nrg_Q = res_tmp_nrg_Q;
        }
        SKP_Silk_A2NLSF(NLSF_Q15, a_tmp_Q16.as_mut_ptr(), LPC_order);
        k = 3 as libc::c_int;
        while k >= 0 as libc::c_int {
            SKP_Silk_interpolate(
                NLSF0_Q15.as_mut_ptr(),
                prev_NLSFq_Q15,
                NLSF_Q15 as *const libc::c_int,
                k,
                LPC_order,
            );
            SKP_Silk_NLSF2A_stable(
                a_tmp_Q12.as_mut_ptr(),
                NLSF0_Q15.as_mut_ptr() as *const libc::c_int,
                LPC_order,
            );
            memset(
                S.as_mut_ptr() as *mut libc::c_void,
                0 as libc::c_int,
                (LPC_order as libc::c_ulong)
                    .wrapping_mul(
                        ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                    ),
            );
            SKP_Silk_LPC_analysis_filter(
                x,
                a_tmp_Q12.as_mut_ptr(),
                S.as_mut_ptr(),
                LPC_res.as_mut_ptr(),
                2 as libc::c_int * subfr_length,
                LPC_order,
            );
            SKP_Silk_sum_sqr_shift(
                &mut res_nrg0,
                &mut rshift0,
                LPC_res.as_mut_ptr().offset(LPC_order as isize),
                subfr_length - LPC_order,
            );
            SKP_Silk_sum_sqr_shift(
                &mut res_nrg1,
                &mut rshift1,
                LPC_res
                    .as_mut_ptr()
                    .offset(LPC_order as isize)
                    .offset(subfr_length as isize),
                subfr_length - LPC_order,
            );
            shift = rshift0 - rshift1;
            if shift >= 0 as libc::c_int {
                res_nrg1 = res_nrg1 >> shift;
                res_nrg_interp_Q = -rshift0;
            } else {
                res_nrg0 = res_nrg0 >> -shift;
                res_nrg_interp_Q = -rshift1;
            }
            res_nrg_interp = res_nrg0 + res_nrg1;
            shift = res_nrg_interp_Q - res_nrg_Q;
            if shift >= 0 as libc::c_int {
                if res_nrg_interp >> shift < res_nrg {
                    isInterpLower = 1 as libc::c_int;
                } else {
                    isInterpLower = 0 as libc::c_int;
                }
            } else if -shift < 32 as libc::c_int {
                if res_nrg_interp < res_nrg >> -shift {
                    isInterpLower = 1 as libc::c_int;
                } else {
                    isInterpLower = 0 as libc::c_int;
                }
            } else {
                isInterpLower = 0 as libc::c_int;
            }
            if isInterpLower == 1 as libc::c_int {
                res_nrg = res_nrg_interp;
                res_nrg_Q = res_nrg_interp_Q;
                *interpIndex = k;
            }
            k -= 1;
            k;
        }
    }
    if *interpIndex == 4 as libc::c_int {
        SKP_Silk_A2NLSF(NLSF_Q15, a_Q16.as_mut_ptr(), LPC_order);
    }
}
