use crate::{skp_l_shift, skp_s_mul_b_b, skp_silk_nlsf_stabilize::skp_silk_nlsf_stabilize};
use once_cell::sync::Lazy;
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct SkpSilkNlsfCbs {
    pub n_vectors: i32,
    pub cb_nlsf_q15: &'static [i16],
    pub rates_q5: &'static [i16],
}
#[derive(Clone, Debug)]
#[repr(C)]
pub struct SkpSilkNlsfCbStruct {
    pub n_stages: i32,
    pub cb_stages: &'static Lazy<Vec<SkpSilkNlsfCbs>>,
    pub n_delta_min_q15: &'static [i32],
    pub cdf: &'static [u16],
    pub start_ptr: &'static Lazy<Vec<&'static [u16]>>,
    pub middle_ix: &'static [i32],
}
#[no_mangle]
pub fn skp_silk_nlsf_msvq_decode(
    p_nlsf_q15: &mut [i32],
    ps_nlsf_cb: &SkpSilkNlsfCbStruct,
    nlsf_indices: &[i32],
    lpc_order: i32,
) {
    let mut p_cb_element =
        &ps_nlsf_cb.cb_stages[0].cb_nlsf_q15[(nlsf_indices[0] * lpc_order) as usize..];
    for i in 0..lpc_order {
        p_nlsf_q15[i as usize] = p_cb_element[i as usize] as i32;
    }
    for s in 1..ps_nlsf_cb.n_stages {
        if lpc_order == 16 {
            p_cb_element = &ps_nlsf_cb.cb_stages[s as usize].cb_nlsf_q15
                [skp_l_shift!(nlsf_indices[s as usize], 4) as usize..];
            p_nlsf_q15[0] += p_cb_element[0] as i32;
            p_nlsf_q15[1] += p_cb_element[1] as i32;
            p_nlsf_q15[2] += p_cb_element[2] as i32;
            p_nlsf_q15[3] += p_cb_element[3] as i32;
            p_nlsf_q15[4] += p_cb_element[4] as i32;
            p_nlsf_q15[5] += p_cb_element[5] as i32;
            p_nlsf_q15[6] += p_cb_element[6] as i32;
            p_nlsf_q15[7] += p_cb_element[7] as i32;
            p_nlsf_q15[8] += p_cb_element[8] as i32;
            p_nlsf_q15[9] += p_cb_element[9] as i32;
            p_nlsf_q15[10] += p_cb_element[10] as i32;
            p_nlsf_q15[11] += p_cb_element[11] as i32;
            p_nlsf_q15[12] += p_cb_element[12] as i32;
            p_nlsf_q15[13] += p_cb_element[13] as i32;
            p_nlsf_q15[14] += p_cb_element[14] as i32;
            p_nlsf_q15[15] += p_cb_element[15] as i32;
        } else {
            p_cb_element = &ps_nlsf_cb.cb_stages[s as usize].cb_nlsf_q15
                [skp_s_mul_b_b!(nlsf_indices[s as usize], lpc_order) as usize..];
            for i in 0..lpc_order {
                p_nlsf_q15[i as usize] += p_cb_element[i as usize] as i32;
            }
        }
    }
    skp_silk_nlsf_stabilize(p_nlsf_q15, ps_nlsf_cb.n_delta_min_q15, lpc_order as usize);
}
