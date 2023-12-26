#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]

use crate::{SKP_Silk_range_coder::{SKP_Silk_range_coder_state, SKP_Silk_range_encoder, SKP_Silk_range_decoder}, skp_silk_tables_sign::SKP_SILK_SIGN_CDF, skp_s_mul_b_b, skp_l_shift, skp_dec_map};
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_encode_signs(
    mut sRC: &mut SKP_Silk_range_coder_state,
    mut q: *const libc::c_schar,
    length: libc::c_int,
    sigtype: libc::c_int,
    QuantOffsetType: libc::c_int,
    RateLevelIndex: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut inData: libc::c_int = 0;
    let mut cdf: [libc::c_ushort; 3] = [0; 3];
    i = (10 as libc::c_int - 1 as libc::c_int) as libc::c_short as libc::c_int
        * ((sigtype << 1 as libc::c_int) + QuantOffsetType) as libc::c_short
            as libc::c_int + RateLevelIndex;
    cdf[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_ushort;
    cdf[1 as libc::c_int as usize] = SKP_SILK_SIGN_CDF[i as usize];
    cdf[2 as libc::c_int as usize] = 65535 as libc::c_int as libc::c_ushort;
    i = 0 as libc::c_int;
    while i < length {
        if *q.offset(i as isize) as libc::c_int != 0 as libc::c_int {
            inData = (*q.offset(i as isize) as libc::c_int >> 15 as libc::c_int)
                + 1 as libc::c_int;
            SKP_Silk_range_encoder(
                sRC,
                inData,
                &cdf,
            );
        }
        i += 1;
    }
}

#[no_mangle]
pub unsafe fn SKP_Silk_decode_signs(
    sRC: &mut SKP_Silk_range_coder_state,
    mut q: *mut libc::c_int,
    length: i32,
    sig_type: i32,
    quant_offset_type: i32,
    rate_level_index: i32,
) {
    let mut cdf = [
        0,
        SKP_SILK_SIGN_CDF[
            (skp_s_mul_b_b!(10 - 1, skp_l_shift!(sig_type,1) + quant_offset_type) + rate_level_index) as usize
        ],
        65535,
    ];
    for i in 0..length {
        if *q.offset(i as isize) > 0 {
            let data = SKP_Silk_range_decoder(
                sRC,
                &cdf,
                1,
            );
            *q.offset(i as isize) *= skp_dec_map!(data);
        }
    }
}
