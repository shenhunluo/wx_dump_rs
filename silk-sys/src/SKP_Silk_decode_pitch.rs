#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]

use crate::skp_silk_pitch_est_tables::{SKP_SILK_CB_LAGS_STAGE2, SKP_SILK_CB_LAGS_STAGE3};
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_decode_pitch(
    mut lagIndex: libc::c_int,
    mut contourIndex: libc::c_int,
    mut pitch_lags: *mut libc::c_int,
    mut Fs_kHz: libc::c_int,
) {
    let mut lag: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut min_lag: libc::c_int = 0;
    min_lag = 2 as libc::c_int as libc::c_short as libc::c_int
        * Fs_kHz as libc::c_short as libc::c_int;
    lag = min_lag + lagIndex;
    if Fs_kHz == 8 as libc::c_int {
        i = 0 as libc::c_int;
        while i < 4 as libc::c_int {
            *pitch_lags
                .offset(
                    i as isize,
                ) = lag
                + SKP_SILK_CB_LAGS_STAGE2[i as usize][contourIndex as usize]
                    as libc::c_int;
            i += 1;
        }
    } else {
        i = 0 as libc::c_int;
        while i < 4 as libc::c_int {
            *pitch_lags
                .offset(
                    i as isize,
                ) = lag
                + SKP_SILK_CB_LAGS_STAGE3[i as usize][contourIndex as usize]
                    as libc::c_int;
            i += 1;
        }
    };
}
