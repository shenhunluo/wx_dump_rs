#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]

use crate::{SKP_Silk_LPC_inv_pred_gain::SKP_Silk_LPC_inverse_pred_gain, SKP_Silk_bwexpander::SKP_Silk_bwexpander, SKP_Silk_NLSF2A::SKP_Silk_NLSF2A};
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_NLSF2A_stable(
    mut pAR_Q12: *mut libc::c_short,
    mut pNLSF: *const libc::c_int,
    LPC_order: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut invGain_Q30: libc::c_int = 0;
    SKP_Silk_NLSF2A(pAR_Q12, pNLSF, LPC_order);
    i = 0 as libc::c_int;
    while i < 20 as libc::c_int {
        if !(SKP_Silk_LPC_inverse_pred_gain(
            &mut invGain_Q30,
            pAR_Q12 as *const libc::c_short,
            LPC_order,
        ) == 1 as libc::c_int)
        {
            break;
        }
        SKP_Silk_bwexpander(
            pAR_Q12,
            LPC_order,
            65536 as libc::c_int
                - (10 as libc::c_int + i) as libc::c_short as libc::c_int
                    * i as libc::c_short as libc::c_int,
        );
        i += 1;
    }
    if i == 20 as libc::c_int {
        i = 0 as libc::c_int;
        while i < LPC_order {
            *pAR_Q12.offset(i as isize) = 0 as libc::c_int as libc::c_short;
            i += 1;
        }
    }
}
