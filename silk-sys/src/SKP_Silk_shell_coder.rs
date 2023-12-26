#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]

use crate::{SKP_Silk_range_coder::{SKP_Silk_range_coder_state, SKP_Silk_range_encoder, SKP_Silk_range_decoder}, skp_silk_tables_pulses_per_block::{SKP_SILK_SHELL_CODE_TABLE_OFFSETS, SKP_SILK_SHELL_CODE_TABLE3, SKP_SILK_SHELL_CODE_TABLE2, SKP_SILK_SHELL_CODE_TABLE1, SKP_SILK_SHELL_CODE_TABLE0}};
#[inline]
unsafe extern "C" fn combine_pulses(
    mut out: *mut libc::c_int,
    mut in_0: *const libc::c_int,
    len: libc::c_int,
) {
    let mut k: libc::c_int = 0;
    k = 0 as libc::c_int;
    while k < len {
        *out
            .offset(
                k as isize,
            ) = *in_0.offset((2 as libc::c_int * k) as isize)
            + *in_0.offset((2 as libc::c_int * k + 1 as libc::c_int) as isize);
        k += 1;
    }
}
#[inline]
unsafe fn encode_split(
    mut sRC: *mut SKP_Silk_range_coder_state,
    p_child1: libc::c_int,
    p: libc::c_int,
    shell_table: &[u16],
) {
    if p > 0 as libc::c_int {
        let cdf = &shell_table[
                SKP_SILK_SHELL_CODE_TABLE_OFFSETS[p as usize] as usize..
            ];
        SKP_Silk_range_encoder(sRC, p_child1, cdf);
    }
}

fn decode_split(
    mut sRC: &mut SKP_Silk_range_coder_state,
    p: libc::c_int,
    shell_table: &[u16],
) -> (i32,i32) {
    let mut cdf_middle = 0;
    if p > 0 {
        cdf_middle = p >> 1;
        let cdf = &shell_table[SKP_SILK_SHELL_CODE_TABLE_OFFSETS[p as usize] as usize..];
        let r = SKP_Silk_range_decoder(sRC, cdf, cdf_middle);
        (r,p - r)
    } else {
        (0,0)
    }
}
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_shell_encoder(
    mut sRC: *mut SKP_Silk_range_coder_state,
    mut pulses0: *const libc::c_int,
) {
    let mut pulses1: [libc::c_int; 8] = [0; 8];
    let mut pulses2: [libc::c_int; 4] = [0; 4];
    let mut pulses3: [libc::c_int; 2] = [0; 2];
    let mut pulses4: [libc::c_int; 1] = [0; 1];
    combine_pulses(pulses1.as_mut_ptr(), pulses0, 8 as libc::c_int);
    combine_pulses(pulses2.as_mut_ptr(), pulses1.as_mut_ptr(), 4 as libc::c_int);
    combine_pulses(pulses3.as_mut_ptr(), pulses2.as_mut_ptr(), 2 as libc::c_int);
    combine_pulses(pulses4.as_mut_ptr(), pulses3.as_mut_ptr(), 1 as libc::c_int);
    encode_split(
        sRC,
        pulses3[0 as libc::c_int as usize],
        pulses4[0 as libc::c_int as usize],
        &SKP_SILK_SHELL_CODE_TABLE3,
    );
    encode_split(
        sRC,
        pulses2[0 as libc::c_int as usize],
        pulses3[0 as libc::c_int as usize],
        &SKP_SILK_SHELL_CODE_TABLE2,
    );
    encode_split(
        sRC,
        pulses1[0 as libc::c_int as usize],
        pulses2[0 as libc::c_int as usize],
        &SKP_SILK_SHELL_CODE_TABLE1,
    );
    encode_split(
        sRC,
        *pulses0.offset(0 as libc::c_int as isize),
        pulses1[0 as libc::c_int as usize],
        &SKP_SILK_SHELL_CODE_TABLE0,
    );
    encode_split(
        sRC,
        *pulses0.offset(2 as libc::c_int as isize),
        pulses1[1 as libc::c_int as usize],
        &SKP_SILK_SHELL_CODE_TABLE0,
    );
    encode_split(
        sRC,
        pulses1[2 as libc::c_int as usize],
        pulses2[1 as libc::c_int as usize],
        &SKP_SILK_SHELL_CODE_TABLE1,
    );
    encode_split(
        sRC,
        *pulses0.offset(4 as libc::c_int as isize),
        pulses1[2 as libc::c_int as usize],
        &SKP_SILK_SHELL_CODE_TABLE0,
    );
    encode_split(
        sRC,
        *pulses0.offset(6 as libc::c_int as isize),
        pulses1[3 as libc::c_int as usize],
        &SKP_SILK_SHELL_CODE_TABLE0,
    );
    encode_split(
        sRC,
        pulses2[2 as libc::c_int as usize],
        pulses3[1 as libc::c_int as usize],
        &SKP_SILK_SHELL_CODE_TABLE2,
    );
    encode_split(
        sRC,
        pulses1[4 as libc::c_int as usize],
        pulses2[2 as libc::c_int as usize],
        &SKP_SILK_SHELL_CODE_TABLE1,
    );
    encode_split(
        sRC,
        *pulses0.offset(8 as libc::c_int as isize),
        pulses1[4 as libc::c_int as usize],
        &SKP_SILK_SHELL_CODE_TABLE0,
    );
    encode_split(
        sRC,
        *pulses0.offset(10 as libc::c_int as isize),
        pulses1[5 as libc::c_int as usize],
        &SKP_SILK_SHELL_CODE_TABLE0,
    );
    encode_split(
        sRC,
        pulses1[6 as libc::c_int as usize],
        pulses2[3 as libc::c_int as usize],
        &SKP_SILK_SHELL_CODE_TABLE1,
    );
    encode_split(
        sRC,
        *pulses0.offset(12 as libc::c_int as isize),
        pulses1[6 as libc::c_int as usize],
        &SKP_SILK_SHELL_CODE_TABLE0,
    );
    encode_split(
        sRC,
        *pulses0.offset(14 as libc::c_int as isize),
        pulses1[7 as libc::c_int as usize],
        &SKP_SILK_SHELL_CODE_TABLE0,
    );
}
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_shell_decoder(
    mut pulses0: *mut libc::c_int,
    mut sRC: &mut SKP_Silk_range_coder_state,
    pulses4: libc::c_int,
) {
    let mut pulses3: [libc::c_int; 2] = [0; 2];
    let mut pulses2: [libc::c_int; 4] = [0; 4];
    let mut pulses1: [libc::c_int; 8] = [0; 8];
    (pulses3[0],pulses3[1]) = decode_split(
        sRC,
        pulses4,
        &SKP_SILK_SHELL_CODE_TABLE3,
    );
    (pulses2[0],pulses2[1]) = decode_split(
        sRC,
        pulses3[0 as libc::c_int as usize],
        &SKP_SILK_SHELL_CODE_TABLE2,
    );
    (pulses1[0],pulses1[1]) = decode_split(
        sRC,
        pulses2[0 as libc::c_int as usize],
        &SKP_SILK_SHELL_CODE_TABLE1,
    );
    (*pulses0.offset(0),*pulses0.offset(1)) = decode_split(
        sRC,
        pulses1[0 as libc::c_int as usize],
        &SKP_SILK_SHELL_CODE_TABLE0,
    );
    (*pulses0.offset(2),*pulses0.offset(3)) = decode_split(
        sRC,
        pulses1[1 as libc::c_int as usize],
        &SKP_SILK_SHELL_CODE_TABLE0,
    );
    (pulses1[2],pulses1[3]) = decode_split(
        sRC,
        pulses2[1 as libc::c_int as usize],
        &SKP_SILK_SHELL_CODE_TABLE1,
    );
    (*pulses0.offset(4),*pulses0.offset(5)) = decode_split(
        sRC,
        pulses1[2 as libc::c_int as usize],
        &SKP_SILK_SHELL_CODE_TABLE0,
    );
    (*pulses0.offset(6),*pulses0.offset(7)) = decode_split(
        sRC,
        pulses1[3 as libc::c_int as usize],
        &SKP_SILK_SHELL_CODE_TABLE0,
    );
    (pulses2[2],pulses2[3]) = decode_split(
        sRC,
        pulses3[1 as libc::c_int as usize],
        &SKP_SILK_SHELL_CODE_TABLE2,
    );
    (pulses1[4],pulses1[5]) = decode_split(
        sRC,
        pulses2[2 as libc::c_int as usize],
        &SKP_SILK_SHELL_CODE_TABLE1,
    );
    (*pulses0.offset(8),*pulses0.offset(9)) = decode_split(
        sRC,
        pulses1[4 as libc::c_int as usize],
        &SKP_SILK_SHELL_CODE_TABLE0,
    );
    (*pulses0.offset(10),*pulses0.offset(11)) = decode_split(
        sRC,
        pulses1[5 as libc::c_int as usize],
        &SKP_SILK_SHELL_CODE_TABLE0,
    );
    (pulses1[6],pulses1[7]) = decode_split(
        sRC,
        pulses2[3 as libc::c_int as usize],
        &SKP_SILK_SHELL_CODE_TABLE1,
    );
    (*pulses0.offset(12),*pulses0.offset(13)) = decode_split(
        sRC,
        pulses1[6 as libc::c_int as usize],
        &SKP_SILK_SHELL_CODE_TABLE0,
    );
    (*pulses0.offset(14),*pulses0.offset(15)) = decode_split(
        sRC,
        pulses1[7 as libc::c_int as usize],
        &SKP_SILK_SHELL_CODE_TABLE0,
    );
}
