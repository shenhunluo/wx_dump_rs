#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]

use crate::{SKP_Silk_range_coder::{SKP_Silk_range_coder_state, SKP_Silk_range_decoder}, SKP_Silk_dec_API::SKP_Silk_decoder_control, SKP_Silk_shell_coder::SKP_Silk_shell_decoder, SKP_Silk_code_signs::SKP_Silk_decode_signs, skp_silk_tables_pulses_per_block::{SKP_SILK_RATE_LEVELS_CDF, SKP_SILK_RATE_LEVELS_CDF_OFFSET, SKP_SILK_PULSES_PER_BLOCK_CDF, SKP_SILK_PULSES_PER_BLOCK_CDF_OFFSET}, skp_silk_tables_other::SKP_SILK_LSB_CDF};
extern "C" {
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_decode_pulses(
    mut psRC: &mut SKP_Silk_range_coder_state,
    mut psDecCtrl: *mut SKP_Silk_decoder_control,
    mut q: *mut libc::c_int,
    frame_length: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut iter: libc::c_int = 0;
    let mut abs_q: libc::c_int = 0;
    let mut nLS: libc::c_int = 0;
    let mut bit: libc::c_int = 0;
    let mut sum_pulses: [libc::c_int; 30] = [0; 30];
    let mut nLshifts: [libc::c_int; 30] = [0; 30];
    let mut pulses_ptr: *mut libc::c_int = 0 as *mut libc::c_int;
    (*psDecCtrl).RateLevelIndex = SKP_Silk_range_decoder(
        psRC,
        &SKP_SILK_RATE_LEVELS_CDF[(*psDecCtrl).sigtype as usize],
        SKP_SILK_RATE_LEVELS_CDF_OFFSET,
    );
    iter = frame_length / 16 as libc::c_int;
    let cdf_ptr = &SKP_SILK_PULSES_PER_BLOCK_CDF[(*psDecCtrl).RateLevelIndex as usize];
    i = 0 as libc::c_int;
    while i < iter {
        nLshifts[i as usize] = 0 as libc::c_int;
        sum_pulses[i as usize] = SKP_Silk_range_decoder(
            psRC,
            cdf_ptr,
            SKP_SILK_PULSES_PER_BLOCK_CDF_OFFSET,
        );
        while sum_pulses[i as usize] == 18 as libc::c_int + 1 as libc::c_int {
            nLshifts[i as usize] += 1;
            nLshifts[i as usize];
            sum_pulses[i as usize] = SKP_Silk_range_decoder(
                psRC,
                &SKP_SILK_PULSES_PER_BLOCK_CDF[(10 as libc::c_int - 1 as libc::c_int)
                    as usize],
                    SKP_SILK_PULSES_PER_BLOCK_CDF_OFFSET,
            );
        }
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < iter {
        if sum_pulses[i as usize] > 0 as libc::c_int {
            SKP_Silk_shell_decoder(
                &mut *q
                    .offset(
                        (i as libc::c_short as libc::c_int
                            * 16 as libc::c_int as libc::c_short as libc::c_int) as isize,
                    ),
                psRC,
                sum_pulses[i as usize],
            );
        } else {
            memset(
                &mut *q
                    .offset(
                        (i as libc::c_short as libc::c_int
                            * 16 as libc::c_int as libc::c_short as libc::c_int) as isize,
                    ) as *mut libc::c_int as *mut libc::c_void,
                0 as libc::c_int,
                (16 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
            );
        }
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < iter {
        if nLshifts[i as usize] > 0 as libc::c_int {
            nLS = nLshifts[i as usize];
            pulses_ptr = &mut *q
                .offset(
                    (i as libc::c_short as libc::c_int
                        * 16 as libc::c_int as libc::c_short as libc::c_int) as isize,
                ) as *mut libc::c_int;
            k = 0 as libc::c_int;
            while k < 16 as libc::c_int {
                abs_q = *pulses_ptr.offset(k as isize);
                j = 0 as libc::c_int;
                while j < nLS {
                    abs_q = abs_q << 1 as libc::c_int;
                    bit = SKP_Silk_range_decoder(
                        psRC,
                        &SKP_SILK_LSB_CDF,
                        1 as libc::c_int,
                    );
                    abs_q += bit;
                    j += 1;
                }
                *pulses_ptr.offset(k as isize) = abs_q;
                k += 1;
            }
        }
        i += 1;
    }
    SKP_Silk_decode_signs(
        psRC,
        q,
        frame_length,
        (*psDecCtrl).sigtype,
        (*psDecCtrl).QuantOffsetType,
        (*psDecCtrl).RateLevelIndex,
    );
}
