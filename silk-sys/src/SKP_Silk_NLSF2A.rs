#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]

use crate::{skp_silk_bwexpander_32::skp_silk_bwexpander_32, skp_silk_lsf_cos_table::SKP_SILK_LSF_COS_TAB_FIX_Q12, skp_l_shift, skp_s_mul_l, skp_r_shift_round};
pub type __int64_t = libc::c_longlong;
pub type int64_t = __int64_t;

fn skp_silk_nlsf2a_find_poly(
    out: &mut [i32],
    cLSF: &[i32],
    dd: usize,
) {
    out[0] = skp_l_shift!(1,20);
    out[1] = -cLSF[0];
    for k in 1..dd {
        let f_tmp = cLSF[2 * k];
        out[k+1] = skp_l_shift!(out[k-1],1) - skp_r_shift_round!(skp_s_mul_l!(f_tmp,out[k]),20) as i32;
        for n in (2..=k).rev() {
            out[n] += out[n-2] -  skp_r_shift_round!(skp_s_mul_l!(f_tmp,out[n-1]),20) as i32;
        }
        out[1] -= f_tmp;
    }
}
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_NLSF2A(
    mut a: *mut libc::c_short,
    mut NLSF: *const libc::c_int,
    d: libc::c_int,
) {
    let mut k: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut dd: libc::c_int = 0;
    let mut cos_LSF_Q20: [libc::c_int; 16] = [0; 16];
    let mut P: [libc::c_int; 9] = [0; 9];
    let mut Q: [libc::c_int; 9] = [0; 9];
    let mut Ptmp: libc::c_int = 0;
    let mut Qtmp: libc::c_int = 0;
    let mut f_int: libc::c_int = 0;
    let mut f_frac: libc::c_int = 0;
    let mut cos_val: libc::c_int = 0;
    let mut delta: libc::c_int = 0;
    let mut a_int32: [libc::c_int; 16] = [0; 16];
    let mut maxabs: libc::c_int = 0;
    let mut absval: libc::c_int = 0;
    let mut idx: libc::c_int = 0 as libc::c_int;
    let mut sc_Q16: libc::c_int = 0;
    k = 0 as libc::c_int;
    while k < d {
        f_int = *NLSF.offset(k as isize) >> 15 as libc::c_int - 7 as libc::c_int;
        f_frac = *NLSF.offset(k as isize)
            - (f_int << 15 as libc::c_int - 7 as libc::c_int);
        cos_val = SKP_SILK_LSF_COS_TAB_FIX_Q12[f_int as usize];
        delta = SKP_SILK_LSF_COS_TAB_FIX_Q12[(f_int + 1 as libc::c_int) as usize]
            - cos_val;
        cos_LSF_Q20[k as usize] = (cos_val << 8 as libc::c_int) + delta * f_frac;
        k += 1;
    }
    dd = d >> 1 as libc::c_int;
    skp_silk_nlsf2a_find_poly(
        &mut P,
        &cos_LSF_Q20[0..],
        dd as usize,
    );
    skp_silk_nlsf2a_find_poly(
        &mut Q,
        &cos_LSF_Q20[1..],
        dd as usize,
    );
    k = 0 as libc::c_int;
    while k < dd {
        Ptmp = P[(k + 1 as libc::c_int) as usize] + P[k as usize];
        Qtmp = Q[(k + 1 as libc::c_int) as usize] - Q[k as usize];
        a_int32[k
            as usize] = -if 9 as libc::c_int == 1 as libc::c_int {
            (Ptmp + Qtmp >> 1 as libc::c_int) + (Ptmp + Qtmp & 1 as libc::c_int)
        } else {
            (Ptmp + Qtmp >> 9 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                >> 1 as libc::c_int
        };
        a_int32[(d - k - 1 as libc::c_int)
            as usize] = if 9 as libc::c_int == 1 as libc::c_int {
            (Qtmp - Ptmp >> 1 as libc::c_int) + (Qtmp - Ptmp & 1 as libc::c_int)
        } else {
            (Qtmp - Ptmp >> 9 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                >> 1 as libc::c_int
        };
        k += 1;
    }
    i = 0 as libc::c_int;
    while i < 10 as libc::c_int {
        maxabs = 0 as libc::c_int;
        k = 0 as libc::c_int;
        while k < d {
            absval = if a_int32[k as usize] > 0 as libc::c_int {
                a_int32[k as usize]
            } else {
                -a_int32[k as usize]
            };
            if absval > maxabs {
                maxabs = absval;
                idx = k;
            }
            k += 1;
        }
        if !(maxabs > 0x7fff as libc::c_int) {
            break;
        }
        maxabs = if maxabs < 98369 as libc::c_int {
            maxabs
        } else {
            98369 as libc::c_int
        };
        sc_Q16 = 65470 as libc::c_int
            - (65470 as libc::c_int >> 2 as libc::c_int)
                * (maxabs - 0x7fff as libc::c_int)
                / (maxabs * (idx + 1 as libc::c_int) >> 2 as libc::c_int);
        skp_silk_bwexpander_32(&mut a_int32, d as usize, sc_Q16);
        i += 1;
    }
    if i == 10 as libc::c_int {
        k = 0 as libc::c_int;
        while k < d {
            a_int32[k
                as usize] = if a_int32[k as usize] > 0x7fff as libc::c_int {
                0x7fff as libc::c_int
            } else if a_int32[k as usize]
                < 0x8000 as libc::c_int as libc::c_short as libc::c_int
            {
                0x8000 as libc::c_int as libc::c_short as libc::c_int
            } else {
                a_int32[k as usize]
            };
            k += 1;
        }
    }
    k = 0 as libc::c_int;
    while k < d {
        *a.offset(k as isize) = a_int32[k as usize] as libc::c_short;
        k += 1;
    }
}
