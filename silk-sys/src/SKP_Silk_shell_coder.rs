#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]

use crate::{SKP_Silk_range_coder::{SKP_Silk_range_coder_state, SKP_Silk_range_encoder, SKP_Silk_range_decoder}, SKP_Silk_tables_pulses_per_block::{SKP_Silk_shell_code_table_offsets, SKP_Silk_shell_code_table3, SKP_Silk_shell_code_table2, SKP_Silk_shell_code_table1, SKP_Silk_shell_code_table0}};
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
unsafe extern "C" fn encode_split(
    mut sRC: *mut SKP_Silk_range_coder_state,
    p_child1: libc::c_int,
    p: libc::c_int,
    mut shell_table: *const libc::c_ushort,
) {
    let mut cdf: *const libc::c_ushort = 0 as *const libc::c_ushort;
    if p > 0 as libc::c_int {
        cdf = &*shell_table
            .offset(
                *SKP_Silk_shell_code_table_offsets.as_ptr().offset(p as isize) as isize,
            ) as *const libc::c_ushort;
        SKP_Silk_range_encoder(sRC, p_child1, cdf);
    }
}
#[inline]
unsafe fn decode_split(
    mut p_child1: *mut libc::c_int,
    mut p_child2: *mut libc::c_int,
    mut sRC: *mut SKP_Silk_range_coder_state,
    p: libc::c_int,
    shell_table: &[u16],
) {
    let mut cdf_middle: libc::c_int = 0;
    if p > 0 as libc::c_int {
        cdf_middle = p >> 1 as libc::c_int;
        let cdf = &shell_table[SKP_Silk_shell_code_table_offsets[p as usize] as usize..];
        SKP_Silk_range_decoder(p_child1, sRC, cdf, cdf_middle);
        *p_child2
            .offset(
                0 as libc::c_int as isize,
            ) = p - *p_child1.offset(0 as libc::c_int as isize);
    } else {
        *p_child1.offset(0 as libc::c_int as isize) = 0 as libc::c_int;
        *p_child2.offset(0 as libc::c_int as isize) = 0 as libc::c_int;
    };
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
        SKP_Silk_shell_code_table3.as_ptr(),
    );
    encode_split(
        sRC,
        pulses2[0 as libc::c_int as usize],
        pulses3[0 as libc::c_int as usize],
        SKP_Silk_shell_code_table2.as_ptr(),
    );
    encode_split(
        sRC,
        pulses1[0 as libc::c_int as usize],
        pulses2[0 as libc::c_int as usize],
        SKP_Silk_shell_code_table1.as_ptr(),
    );
    encode_split(
        sRC,
        *pulses0.offset(0 as libc::c_int as isize),
        pulses1[0 as libc::c_int as usize],
        SKP_Silk_shell_code_table0.as_ptr(),
    );
    encode_split(
        sRC,
        *pulses0.offset(2 as libc::c_int as isize),
        pulses1[1 as libc::c_int as usize],
        SKP_Silk_shell_code_table0.as_ptr(),
    );
    encode_split(
        sRC,
        pulses1[2 as libc::c_int as usize],
        pulses2[1 as libc::c_int as usize],
        SKP_Silk_shell_code_table1.as_ptr(),
    );
    encode_split(
        sRC,
        *pulses0.offset(4 as libc::c_int as isize),
        pulses1[2 as libc::c_int as usize],
        SKP_Silk_shell_code_table0.as_ptr(),
    );
    encode_split(
        sRC,
        *pulses0.offset(6 as libc::c_int as isize),
        pulses1[3 as libc::c_int as usize],
        SKP_Silk_shell_code_table0.as_ptr(),
    );
    encode_split(
        sRC,
        pulses2[2 as libc::c_int as usize],
        pulses3[1 as libc::c_int as usize],
        SKP_Silk_shell_code_table2.as_ptr(),
    );
    encode_split(
        sRC,
        pulses1[4 as libc::c_int as usize],
        pulses2[2 as libc::c_int as usize],
        SKP_Silk_shell_code_table1.as_ptr(),
    );
    encode_split(
        sRC,
        *pulses0.offset(8 as libc::c_int as isize),
        pulses1[4 as libc::c_int as usize],
        SKP_Silk_shell_code_table0.as_ptr(),
    );
    encode_split(
        sRC,
        *pulses0.offset(10 as libc::c_int as isize),
        pulses1[5 as libc::c_int as usize],
        SKP_Silk_shell_code_table0.as_ptr(),
    );
    encode_split(
        sRC,
        pulses1[6 as libc::c_int as usize],
        pulses2[3 as libc::c_int as usize],
        SKP_Silk_shell_code_table1.as_ptr(),
    );
    encode_split(
        sRC,
        *pulses0.offset(12 as libc::c_int as isize),
        pulses1[6 as libc::c_int as usize],
        SKP_Silk_shell_code_table0.as_ptr(),
    );
    encode_split(
        sRC,
        *pulses0.offset(14 as libc::c_int as isize),
        pulses1[7 as libc::c_int as usize],
        SKP_Silk_shell_code_table0.as_ptr(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_shell_decoder(
    mut pulses0: *mut libc::c_int,
    mut sRC: *mut SKP_Silk_range_coder_state,
    pulses4: libc::c_int,
) {
    let mut pulses3: [libc::c_int; 2] = [0; 2];
    let mut pulses2: [libc::c_int; 4] = [0; 4];
    let mut pulses1: [libc::c_int; 8] = [0; 8];
    decode_split(
        &mut *pulses3.as_mut_ptr().offset(0 as libc::c_int as isize),
        &mut *pulses3.as_mut_ptr().offset(1 as libc::c_int as isize),
        sRC,
        pulses4,
        &SKP_Silk_shell_code_table3,
    );
    decode_split(
        &mut *pulses2.as_mut_ptr().offset(0 as libc::c_int as isize),
        &mut *pulses2.as_mut_ptr().offset(1 as libc::c_int as isize),
        sRC,
        pulses3[0 as libc::c_int as usize],
        &SKP_Silk_shell_code_table2,
    );
    decode_split(
        &mut *pulses1.as_mut_ptr().offset(0 as libc::c_int as isize),
        &mut *pulses1.as_mut_ptr().offset(1 as libc::c_int as isize),
        sRC,
        pulses2[0 as libc::c_int as usize],
        &SKP_Silk_shell_code_table1,
    );
    decode_split(
        &mut *pulses0.offset(0 as libc::c_int as isize),
        &mut *pulses0.offset(1 as libc::c_int as isize),
        sRC,
        pulses1[0 as libc::c_int as usize],
        &SKP_Silk_shell_code_table0,
    );
    decode_split(
        &mut *pulses0.offset(2 as libc::c_int as isize),
        &mut *pulses0.offset(3 as libc::c_int as isize),
        sRC,
        pulses1[1 as libc::c_int as usize],
        &SKP_Silk_shell_code_table0,
    );
    decode_split(
        &mut *pulses1.as_mut_ptr().offset(2 as libc::c_int as isize),
        &mut *pulses1.as_mut_ptr().offset(3 as libc::c_int as isize),
        sRC,
        pulses2[1 as libc::c_int as usize],
        &SKP_Silk_shell_code_table1,
    );
    decode_split(
        &mut *pulses0.offset(4 as libc::c_int as isize),
        &mut *pulses0.offset(5 as libc::c_int as isize),
        sRC,
        pulses1[2 as libc::c_int as usize],
        &SKP_Silk_shell_code_table0,
    );
    decode_split(
        &mut *pulses0.offset(6 as libc::c_int as isize),
        &mut *pulses0.offset(7 as libc::c_int as isize),
        sRC,
        pulses1[3 as libc::c_int as usize],
        &SKP_Silk_shell_code_table0,
    );
    decode_split(
        &mut *pulses2.as_mut_ptr().offset(2 as libc::c_int as isize),
        &mut *pulses2.as_mut_ptr().offset(3 as libc::c_int as isize),
        sRC,
        pulses3[1 as libc::c_int as usize],
        &SKP_Silk_shell_code_table2,
    );
    decode_split(
        &mut *pulses1.as_mut_ptr().offset(4 as libc::c_int as isize),
        &mut *pulses1.as_mut_ptr().offset(5 as libc::c_int as isize),
        sRC,
        pulses2[2 as libc::c_int as usize],
        &SKP_Silk_shell_code_table1,
    );
    decode_split(
        &mut *pulses0.offset(8 as libc::c_int as isize),
        &mut *pulses0.offset(9 as libc::c_int as isize),
        sRC,
        pulses1[4 as libc::c_int as usize],
        &SKP_Silk_shell_code_table0,
    );
    decode_split(
        &mut *pulses0.offset(10 as libc::c_int as isize),
        &mut *pulses0.offset(11 as libc::c_int as isize),
        sRC,
        pulses1[5 as libc::c_int as usize],
        &SKP_Silk_shell_code_table0,
    );
    decode_split(
        &mut *pulses1.as_mut_ptr().offset(6 as libc::c_int as isize),
        &mut *pulses1.as_mut_ptr().offset(7 as libc::c_int as isize),
        sRC,
        pulses2[3 as libc::c_int as usize],
        &SKP_Silk_shell_code_table1,
    );
    decode_split(
        &mut *pulses0.offset(12 as libc::c_int as isize),
        &mut *pulses0.offset(13 as libc::c_int as isize),
        sRC,
        pulses1[6 as libc::c_int as usize],
        &SKP_Silk_shell_code_table0,
    );
    decode_split(
        &mut *pulses0.offset(14 as libc::c_int as isize),
        &mut *pulses0.offset(15 as libc::c_int as isize),
        sRC,
        pulses1[7 as libc::c_int as usize],
        &SKP_Silk_shell_code_table0,
    );
}
