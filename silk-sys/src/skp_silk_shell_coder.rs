use crate::{
    skp_silk_tables_pulses_per_block::{
        SKP_SILK_SHELL_CODE_TABLE0, SKP_SILK_SHELL_CODE_TABLE1, SKP_SILK_SHELL_CODE_TABLE2,
        SKP_SILK_SHELL_CODE_TABLE3, SKP_SILK_SHELL_CODE_TABLE_OFFSETS,
    },
    SKP_Silk_range_coder::{
        SKP_Silk_range_coder_state, SKP_Silk_range_decoder, SKP_Silk_range_encoder,
    },
};

fn combine_pulses(out: &mut [i32], in_0: &[i32], len: usize) {
    for k in 0..len {
        out[k] = in_0[2 * k] + in_0[2 * k + 1];
    }
}

fn encode_split(
    s_r_c: &mut SKP_Silk_range_coder_state,
    p_child1: i32,
    p: i32,
    shell_table: &[u16],
) {
    if p > 0 {
        let cdf = &shell_table[SKP_SILK_SHELL_CODE_TABLE_OFFSETS[p as usize] as usize..];
        SKP_Silk_range_encoder(s_r_c, p_child1 as usize, cdf);
    }
}

fn decode_split(
    s_r_c: &mut SKP_Silk_range_coder_state,
    p: i32,
    shell_table: &[u16],
) -> (i32, i32) {
    if p > 0 {
        let cdf_middle = p >> 1;
        let cdf = &shell_table[SKP_SILK_SHELL_CODE_TABLE_OFFSETS[p as usize] as usize..];
        let r = SKP_Silk_range_decoder(s_r_c, cdf, cdf_middle);
        (r, p - r)
    } else {
        (0, 0)
    }
}

pub fn skp_silk_shell_encoder(s_r_c: &mut SKP_Silk_range_coder_state, pulses0: &mut [i32]) {
    let mut pulses1 = [0; 8];
    let mut pulses2 = [0; 4];
    let mut pulses3 = [0; 2];
    let mut pulses4 = [0; 1];

    combine_pulses(&mut pulses1, pulses0, 8);
    combine_pulses(&mut pulses2, &pulses1, 4);
    combine_pulses(&mut pulses3, &pulses2, 2);
    combine_pulses(&mut pulses4, &pulses3, 1);

    encode_split(s_r_c, pulses3[0], pulses4[0], &SKP_SILK_SHELL_CODE_TABLE3);
    encode_split(s_r_c, pulses2[0], pulses3[0], &SKP_SILK_SHELL_CODE_TABLE2);
    encode_split(s_r_c, pulses1[0], pulses2[0], &SKP_SILK_SHELL_CODE_TABLE1);
    encode_split(s_r_c, pulses0[0], pulses1[0], &SKP_SILK_SHELL_CODE_TABLE0);
    encode_split(s_r_c, pulses0[2], pulses1[1], &SKP_SILK_SHELL_CODE_TABLE0);
    encode_split(s_r_c, pulses1[2], pulses2[1], &SKP_SILK_SHELL_CODE_TABLE1);
    encode_split(s_r_c, pulses0[4], pulses1[2], &SKP_SILK_SHELL_CODE_TABLE0);
    encode_split(s_r_c, pulses0[6], pulses1[3], &SKP_SILK_SHELL_CODE_TABLE0);
    encode_split(s_r_c, pulses2[2], pulses3[1], &SKP_SILK_SHELL_CODE_TABLE2);
    encode_split(s_r_c, pulses1[4], pulses2[2], &SKP_SILK_SHELL_CODE_TABLE1);
    encode_split(s_r_c, pulses0[8], pulses1[4], &SKP_SILK_SHELL_CODE_TABLE0);
    encode_split(s_r_c, pulses0[10], pulses1[5], &SKP_SILK_SHELL_CODE_TABLE0);
    encode_split(s_r_c, pulses1[6], pulses2[3], &SKP_SILK_SHELL_CODE_TABLE1);
    encode_split(s_r_c, pulses0[12], pulses1[6], &SKP_SILK_SHELL_CODE_TABLE0);
    encode_split(s_r_c, pulses0[14], pulses1[7], &SKP_SILK_SHELL_CODE_TABLE0);
}

pub fn skp_silk_shell_decoder(
    pulses0: &mut [i32],
    s_r_c: &mut SKP_Silk_range_coder_state,
    pulses4: i32,
) {
    let mut pulses3 = [0; 2];
    let mut pulses2 = [0; 4];
    let mut pulses1 = [0; 8];
    (pulses3[0], pulses3[1]) = decode_split(s_r_c, pulses4, &SKP_SILK_SHELL_CODE_TABLE3);
    (pulses2[0], pulses2[1]) = decode_split(s_r_c, pulses3[0], &SKP_SILK_SHELL_CODE_TABLE2);
    (pulses1[0], pulses1[1]) = decode_split(s_r_c, pulses2[0], &SKP_SILK_SHELL_CODE_TABLE1);
    (pulses0[0], pulses0[1]) = decode_split(s_r_c, pulses1[0], &SKP_SILK_SHELL_CODE_TABLE0);
    (pulses0[2], pulses0[3]) = decode_split(s_r_c, pulses1[1], &SKP_SILK_SHELL_CODE_TABLE0);
    (pulses1[2], pulses1[3]) = decode_split(s_r_c, pulses2[1], &SKP_SILK_SHELL_CODE_TABLE1);
    (pulses0[4], pulses0[5]) = decode_split(s_r_c, pulses1[2], &SKP_SILK_SHELL_CODE_TABLE0);
    (pulses0[6], pulses0[7]) = decode_split(s_r_c, pulses1[3], &SKP_SILK_SHELL_CODE_TABLE0);
    (pulses2[2], pulses2[3]) = decode_split(s_r_c, pulses3[1], &SKP_SILK_SHELL_CODE_TABLE2);
    (pulses1[4], pulses1[5]) = decode_split(s_r_c, pulses2[2], &SKP_SILK_SHELL_CODE_TABLE1);
    (pulses0[8], pulses0[9]) = decode_split(s_r_c, pulses1[4], &SKP_SILK_SHELL_CODE_TABLE0);
    (pulses0[10], pulses0[11]) = decode_split(s_r_c, pulses1[5], &SKP_SILK_SHELL_CODE_TABLE0);
    (pulses1[6], pulses1[7]) = decode_split(s_r_c, pulses2[3], &SKP_SILK_SHELL_CODE_TABLE1);
    (pulses0[12], pulses0[13]) = decode_split(s_r_c, pulses1[6], &SKP_SILK_SHELL_CODE_TABLE0);
    (pulses0[14], pulses0[15]) = decode_split(s_r_c, pulses1[7], &SKP_SILK_SHELL_CODE_TABLE0);
}
