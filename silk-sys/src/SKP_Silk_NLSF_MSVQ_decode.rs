#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]

use once_cell::sync::Lazy;

use crate::{skp_silk_nlsf_stabilize::skp_silk_nlsf_stabilize, skp_l_shift, skp_s_mul_b_b};
#[derive(Copy, Clone,Debug)]
#[repr(C)]
pub struct SKP_Silk_NLSF_CBS {
    pub nVectors: i32,
    pub CB_NLSF_Q15: &'static [i16],
    pub Rates_Q5: &'static[i16],
}
#[derive(Clone,Debug)]
#[repr(C)]
pub struct SKP_Silk_NLSF_CB_struct {
    pub nStages: libc::c_int,
    pub CBStages: &'static Lazy<Vec<SKP_Silk_NLSF_CBS>>,
    pub NDeltaMin_Q15: &'static [i32],
    pub CDF: &'static [u16],
    pub StartPtr: &'static Lazy<Vec<&'static [u16]>>,
    pub MiddleIx: &'static [i32],
}
#[no_mangle]
pub fn SKP_Silk_NLSF_MSVQ_decode(
    mut pNLSF_Q15: &mut [i32],
    psNLSF_CB: &SKP_Silk_NLSF_CB_struct,
    NLSFIndices: &[i32],
    LPC_order: i32,
) {
    let mut pCB_element = &psNLSF_CB.CBStages[0].CB_NLSF_Q15[(NLSFIndices[0] * LPC_order) as usize..];
    for i in 0..LPC_order {
        pNLSF_Q15[i as usize] = pCB_element[i as usize] as i32;
    }
    for s in  1..psNLSF_CB.nStages {
        if LPC_order == 16 {
            pCB_element = &psNLSF_CB.CBStages[s as usize].CB_NLSF_Q15[skp_l_shift!(NLSFIndices[s as usize], 4) as usize..];
            pNLSF_Q15[0]
                += pCB_element[0] as i32;
            pNLSF_Q15[1]
                += pCB_element[1] as i32;
            pNLSF_Q15[2]
                += pCB_element[2] as i32;
            pNLSF_Q15[3]
                += pCB_element[3] as i32;
            pNLSF_Q15[4]
                += pCB_element[4] as i32;
            pNLSF_Q15[5]
                += pCB_element[5] as i32;
            pNLSF_Q15[6]
                += pCB_element[6] as i32;
            pNLSF_Q15[7]
                += pCB_element[7] as i32;
            pNLSF_Q15[8]
                += pCB_element[8] as i32;
            pNLSF_Q15[9]
                += pCB_element[9] as i32;
            pNLSF_Q15[10]
                += pCB_element[10] as i32;
            pNLSF_Q15[11]
                += pCB_element[11] as i32;
            pNLSF_Q15[12]
                += pCB_element[12] as i32;
            pNLSF_Q15[13]
                += pCB_element[13] as i32;
            pNLSF_Q15[14]
                += pCB_element[14] as i32;
            pNLSF_Q15[15]
                += pCB_element[15] as i32;
        } else {
            pCB_element = &psNLSF_CB.CBStages[s as usize].CB_NLSF_Q15[skp_s_mul_b_b!(NLSFIndices[s as usize],LPC_order) as usize..];
            for i in 0..LPC_order {
                pNLSF_Q15[i as usize]
                    += pCB_element[i as usize] as i32;
            }
        }
    }
    skp_silk_nlsf_stabilize(pNLSF_Q15, psNLSF_CB.NDeltaMin_Q15, LPC_order as usize);
}
