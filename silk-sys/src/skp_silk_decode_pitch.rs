use crate::{
    skp_s_mul_b_b,
    skp_silk_pitch_est_tables::{SKP_SILK_CB_LAGS_STAGE2, SKP_SILK_CB_LAGS_STAGE3},
};

pub fn skp_silk_decode_pitch(
    lag_index: usize,
    contour_index: usize,
    pitch_lags: &mut [i32],
    fs_k_hz: i32,
) {
    let min_lag = skp_s_mul_b_b!(2, fs_k_hz);
    let lag = min_lag + lag_index as i32;
    if fs_k_hz == 8 {
        for i in 0..4 {
            pitch_lags[i] = lag + SKP_SILK_CB_LAGS_STAGE2[i][contour_index] as i32;
        }
    } else {
        for i in 0..4 {
            pitch_lags[i] = lag + SKP_SILK_CB_LAGS_STAGE3[i][contour_index] as i32;
        }
    };
}
