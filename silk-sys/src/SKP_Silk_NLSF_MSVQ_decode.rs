#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]

use crate::skp_silk_nlsf_stabilize::skp_silk_nlsf_stabilize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SKP_Silk_NLSF_CBS {
    pub nVectors: libc::c_int,
    pub CB_NLSF_Q15: *const libc::c_short,
    pub Rates_Q5: *const libc::c_short,
}
#[derive(Copy, Clone,Debug)]
#[repr(C)]
pub struct SKP_Silk_NLSF_CB_struct {
    pub nStages: libc::c_int,
    pub CBStages: *const SKP_Silk_NLSF_CBS,
    pub NDeltaMin_Q15: &'static [i32],
    pub CDF: *const libc::c_ushort,
    pub StartPtr: *const *const libc::c_ushort,
    pub MiddleIx: *const libc::c_int,
}
#[no_mangle]
pub unsafe fn SKP_Silk_NLSF_MSVQ_decode(
    mut pNLSF_Q15: &mut [i32],
    mut psNLSF_CB: *const SKP_Silk_NLSF_CB_struct,
    mut NLSFIndices: *const libc::c_int,
    LPC_order: libc::c_int,
) {
    let mut pCB_element: *const libc::c_short = 0 as *const libc::c_short;
    let mut s: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    pCB_element = &*((*((*psNLSF_CB).CBStages).offset(0 as libc::c_int as isize))
        .CB_NLSF_Q15)
        .offset((*NLSFIndices.offset(0 as libc::c_int as isize) * LPC_order) as isize)
        as *const libc::c_short;
    i = 0 as libc::c_int;
    while i < LPC_order {
        pNLSF_Q15[i as usize] = *pCB_element.offset(i as isize) as libc::c_int;
        i += 1;
    }
    s = 1 as libc::c_int;
    while s < (*psNLSF_CB).nStages {
        if LPC_order == 16 as libc::c_int {
            pCB_element = &*((*((*psNLSF_CB).CBStages).offset(s as isize)).CB_NLSF_Q15)
                .offset((*NLSFIndices.offset(s as isize) << 4 as libc::c_int) as isize)
                as *const libc::c_short;
            pNLSF_Q15[0]
                += *pCB_element.offset(0 as libc::c_int as isize) as libc::c_int;
            pNLSF_Q15[1]
                += *pCB_element.offset(1 as libc::c_int as isize) as libc::c_int;
            pNLSF_Q15[2]
                += *pCB_element.offset(2 as libc::c_int as isize) as libc::c_int;
            pNLSF_Q15[3]
                += *pCB_element.offset(3 as libc::c_int as isize) as libc::c_int;
            pNLSF_Q15[4]
                += *pCB_element.offset(4 as libc::c_int as isize) as libc::c_int;
            pNLSF_Q15[5]
                += *pCB_element.offset(5 as libc::c_int as isize) as libc::c_int;
            pNLSF_Q15[6]
                += *pCB_element.offset(6 as libc::c_int as isize) as libc::c_int;
            pNLSF_Q15[7]
                += *pCB_element.offset(7 as libc::c_int as isize) as libc::c_int;
            pNLSF_Q15[8]
                += *pCB_element.offset(8 as libc::c_int as isize) as libc::c_int;
            pNLSF_Q15[9]
                += *pCB_element.offset(9 as libc::c_int as isize) as libc::c_int;
            pNLSF_Q15[10]
                += *pCB_element.offset(10 as libc::c_int as isize) as libc::c_int;
            pNLSF_Q15[11]
                += *pCB_element.offset(11 as libc::c_int as isize) as libc::c_int;
            pNLSF_Q15[12]
                += *pCB_element.offset(12 as libc::c_int as isize) as libc::c_int;
            pNLSF_Q15[13]
                += *pCB_element.offset(13 as libc::c_int as isize) as libc::c_int;
            pNLSF_Q15[14]
                += *pCB_element.offset(14 as libc::c_int as isize) as libc::c_int;
            pNLSF_Q15[15]
                += *pCB_element.offset(15 as libc::c_int as isize) as libc::c_int;
        } else {
            pCB_element = &*((*((*psNLSF_CB).CBStages).offset(s as isize)).CB_NLSF_Q15)
                .offset(
                    (*NLSFIndices.offset(s as isize) as libc::c_short as libc::c_int
                        * LPC_order as libc::c_short as libc::c_int) as isize,
                ) as *const libc::c_short;
            i = 0 as libc::c_int;
            while i < LPC_order {
                pNLSF_Q15[i as usize]
                    += *pCB_element.offset(i as isize) as libc::c_int;
                i += 1;
            }
        }
        s += 1;
    }
    skp_silk_nlsf_stabilize(pNLSF_Q15, (*psNLSF_CB).NDeltaMin_Q15, LPC_order as usize);
}
