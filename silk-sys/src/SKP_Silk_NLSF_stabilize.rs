use crate::{
    skp_limit_32, skp_r_shift, skp_r_shift_round,
    SKP_Silk_sort::SKP_Silk_insertion_sort_increasing_all_values,
};

pub fn skp_silk_nlsf_stabilize(mut nlsf_q15: &mut [i32], n_delta_min_q15: &[i32], len: usize) {
    let mut center_freq_q15 = 0;
    let mut diff_q15 = 0;
    let mut min_center_q15 = 0;
    let mut max_center_q15 = 0;
    let mut min_diff_q15 = 0;
    let mut loops = 0;
    let mut i_len = 0;
    while loops < 20 {
        min_diff_q15 = nlsf_q15[0] - n_delta_min_q15[0];
        i_len = 0;
        for i in 1..len {
            diff_q15 = nlsf_q15[i] - (nlsf_q15[i - 1] + n_delta_min_q15[i]);
            if diff_q15 < min_diff_q15 {
                min_diff_q15 = diff_q15;
                i_len = i;
            }
        }
        diff_q15 = (1 << 15) - (nlsf_q15[len - 1] + n_delta_min_q15[len]);
        if diff_q15 < min_diff_q15 {
            min_diff_q15 = diff_q15;
            i_len = len;
        }
        if min_diff_q15 >= 0 {
            return;
        }
        if i_len == 0 {
            nlsf_q15[0] = n_delta_min_q15[0];
        } else if i_len == len {
            nlsf_q15[len - 1] = (1 << 15) - n_delta_min_q15[len];
        } else {
            min_center_q15 = 0;
            for k in 0..i_len {
                min_center_q15 += n_delta_min_q15[k];
            }
            min_center_q15 += skp_r_shift!(n_delta_min_q15[i_len], 1);
            max_center_q15 = 1 << 15;
            for k in (i_len+1..=len).rev() {
                max_center_q15 -= n_delta_min_q15[k];
            }
            max_center_q15 -= n_delta_min_q15[i_len] - skp_r_shift!(n_delta_min_q15[i_len], 1);
            center_freq_q15 = skp_limit_32!(
                skp_r_shift_round!(nlsf_q15[i_len - 1] + nlsf_q15[i_len], 1),
                min_center_q15,
                max_center_q15
            );
            nlsf_q15[i_len - 1] = center_freq_q15 - skp_r_shift!(n_delta_min_q15[i_len], 1);
            nlsf_q15[i_len] = nlsf_q15[i_len - 1] + n_delta_min_q15[i_len];
        }
        loops += 1;
    }
    if loops == 20 {
        SKP_Silk_insertion_sort_increasing_all_values(&mut nlsf_q15, len);
        nlsf_q15[0] = i32::max(nlsf_q15[0], n_delta_min_q15[0]);
        for i in 1..len {
            nlsf_q15[i] = i32::max(nlsf_q15[i], nlsf_q15[i - 1] + n_delta_min_q15[i]);
        }
        nlsf_q15[len - 1] = i32::min(nlsf_q15[len - 1], (1 << 15) - n_delta_min_q15[len as usize]);
        for i in (0..=len - 2).rev() {
            nlsf_q15[i] = i32::min(nlsf_q15[i], nlsf_q15[i + 1] - n_delta_min_q15[i + 1]);
        }
    }
}
