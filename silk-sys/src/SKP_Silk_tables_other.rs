#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#[no_mangle]
pub static mut TargetRate_table_NB: [libc::c_int; 8] = [
    0 as libc::c_int,
    8000 as libc::c_int,
    9000 as libc::c_int,
    11000 as libc::c_int,
    13000 as libc::c_int,
    16000 as libc::c_int,
    22000 as libc::c_int,
    100000 as libc::c_int,
];
#[no_mangle]
pub static mut TargetRate_table_MB: [libc::c_int; 8] = [
    0 as libc::c_int,
    10000 as libc::c_int,
    12000 as libc::c_int,
    14000 as libc::c_int,
    17000 as libc::c_int,
    21000 as libc::c_int,
    28000 as libc::c_int,
    100000 as libc::c_int,
];
#[no_mangle]
pub static mut TargetRate_table_WB: [libc::c_int; 8] = [
    0 as libc::c_int,
    11000 as libc::c_int,
    14000 as libc::c_int,
    17000 as libc::c_int,
    21000 as libc::c_int,
    26000 as libc::c_int,
    36000 as libc::c_int,
    100000 as libc::c_int,
];
#[no_mangle]
pub static mut TargetRate_table_SWB: [libc::c_int; 8] = [
    0 as libc::c_int,
    13000 as libc::c_int,
    16000 as libc::c_int,
    19000 as libc::c_int,
    25000 as libc::c_int,
    32000 as libc::c_int,
    46000 as libc::c_int,
    100000 as libc::c_int,
];
#[no_mangle]
pub static mut SNR_table_Q1: [libc::c_int; 8] = [
    19 as libc::c_int,
    31 as libc::c_int,
    35 as libc::c_int,
    39 as libc::c_int,
    43 as libc::c_int,
    47 as libc::c_int,
    54 as libc::c_int,
    64 as libc::c_int,
];
#[no_mangle]
pub static mut SNR_table_one_bit_per_sample_Q7: [libc::c_int; 4] = [
    1984 as libc::c_int,
    2240 as libc::c_int,
    2408 as libc::c_int,
    2708 as libc::c_int,
];
#[no_mangle]
pub static mut SKP_Silk_SWB_detect_B_HP_Q13: [[libc::c_short; 3]; 3] = [
    [
        575 as libc::c_int as libc::c_short,
        -(948 as libc::c_int) as libc::c_short,
        575 as libc::c_int as libc::c_short,
    ],
    [
        575 as libc::c_int as libc::c_short,
        -(221 as libc::c_int) as libc::c_short,
        575 as libc::c_int as libc::c_short,
    ],
    [
        575 as libc::c_int as libc::c_short,
        104 as libc::c_int as libc::c_short,
        575 as libc::c_int as libc::c_short,
    ],
];
#[no_mangle]
pub static mut SKP_Silk_SWB_detect_A_HP_Q13: [[libc::c_short; 2]; 3] = [
    [14613 as libc::c_int as libc::c_short, 6868 as libc::c_int as libc::c_short],
    [12883 as libc::c_int as libc::c_short, 7337 as libc::c_int as libc::c_short],
    [11586 as libc::c_int as libc::c_short, 7911 as libc::c_int as libc::c_short],
];
#[no_mangle]
pub static mut SKP_Silk_Dec_A_HP_24: [libc::c_short; 2] = [
    -(16220 as libc::c_int) as libc::c_short,
    8030 as libc::c_int as libc::c_short,
];
#[no_mangle]
pub static mut SKP_Silk_Dec_B_HP_24: [libc::c_short; 3] = [
    8000 as libc::c_int as libc::c_short,
    -(16000 as libc::c_int) as libc::c_short,
    8000 as libc::c_int as libc::c_short,
];
#[no_mangle]
pub static mut SKP_Silk_Dec_A_HP_16: [libc::c_short; 2] = [
    -(16127 as libc::c_int) as libc::c_short,
    7940 as libc::c_int as libc::c_short,
];
#[no_mangle]
pub static mut SKP_Silk_Dec_B_HP_16: [libc::c_short; 3] = [
    8000 as libc::c_int as libc::c_short,
    -(16000 as libc::c_int) as libc::c_short,
    8000 as libc::c_int as libc::c_short,
];
#[no_mangle]
pub static mut SKP_Silk_Dec_A_HP_12: [libc::c_short; 2] = [
    -(16043 as libc::c_int) as libc::c_short,
    7859 as libc::c_int as libc::c_short,
];
#[no_mangle]
pub static mut SKP_Silk_Dec_B_HP_12: [libc::c_short; 3] = [
    8000 as libc::c_int as libc::c_short,
    -(16000 as libc::c_int) as libc::c_short,
    8000 as libc::c_int as libc::c_short,
];
#[no_mangle]
pub static mut SKP_Silk_Dec_A_HP_8: [libc::c_short; 2] = [
    -(15885 as libc::c_int) as libc::c_short,
    7710 as libc::c_int as libc::c_short,
];
#[no_mangle]
pub static mut SKP_Silk_Dec_B_HP_8: [libc::c_short; 3] = [
    8000 as libc::c_int as libc::c_short,
    -(16000 as libc::c_int) as libc::c_short,
    8000 as libc::c_int as libc::c_short,
];
#[no_mangle]
pub static mut SKP_Silk_lsb_CDF: [libc::c_ushort; 3] = [
    0 as libc::c_int as libc::c_ushort,
    40000 as libc::c_int as libc::c_ushort,
    65535 as libc::c_int as libc::c_ushort,
];
#[no_mangle]
pub static mut SKP_Silk_LTPscale_CDF: [libc::c_ushort; 4] = [
    0 as libc::c_int as libc::c_ushort,
    32000 as libc::c_int as libc::c_ushort,
    48000 as libc::c_int as libc::c_ushort,
    65535 as libc::c_int as libc::c_ushort,
];
#[no_mangle]
pub static mut SKP_Silk_LTPscale_offset: libc::c_int = 2 as libc::c_int;
#[no_mangle]
pub static mut SKP_Silk_vadflag_CDF: [libc::c_ushort; 3] = [
    0 as libc::c_int as libc::c_ushort,
    22000 as libc::c_int as libc::c_ushort,
    65535 as libc::c_int as libc::c_ushort,
];
#[no_mangle]
pub static mut SKP_Silk_vadflag_offset: libc::c_int = 1 as libc::c_int;
#[no_mangle]
pub static mut SKP_Silk_SamplingRates_table: [libc::c_int; 4] = [
    8 as libc::c_int,
    12 as libc::c_int,
    16 as libc::c_int,
    24 as libc::c_int,
];
#[no_mangle]
pub static mut SKP_Silk_SamplingRates_CDF: [libc::c_ushort; 5] = [
    0 as libc::c_int as libc::c_ushort,
    16000 as libc::c_int as libc::c_ushort,
    32000 as libc::c_int as libc::c_ushort,
    48000 as libc::c_int as libc::c_ushort,
    65535 as libc::c_int as libc::c_ushort,
];
#[no_mangle]
pub static mut SKP_Silk_SamplingRates_offset: libc::c_int = 2 as libc::c_int;
#[no_mangle]
pub static mut SKP_Silk_NLSF_interpolation_factor_CDF: [libc::c_ushort; 6] = [
    0 as libc::c_int as libc::c_ushort,
    3706 as libc::c_int as libc::c_ushort,
    8703 as libc::c_int as libc::c_ushort,
    19226 as libc::c_int as libc::c_ushort,
    30926 as libc::c_int as libc::c_ushort,
    65535 as libc::c_int as libc::c_ushort,
];
#[no_mangle]
pub static mut SKP_Silk_NLSF_interpolation_factor_offset: libc::c_int = 4 as libc::c_int;
#[no_mangle]
pub static mut SKP_Silk_FrameTermination_CDF: [libc::c_ushort; 5] = [
    0 as libc::c_int as libc::c_ushort,
    20000 as libc::c_int as libc::c_ushort,
    45000 as libc::c_int as libc::c_ushort,
    56000 as libc::c_int as libc::c_ushort,
    65535 as libc::c_int as libc::c_ushort,
];
#[no_mangle]
pub static mut SKP_Silk_FrameTermination_offset: libc::c_int = 2 as libc::c_int;
#[no_mangle]
pub static mut SKP_Silk_Seed_CDF: [libc::c_ushort; 5] = [
    0 as libc::c_int as libc::c_ushort,
    16384 as libc::c_int as libc::c_ushort,
    32768 as libc::c_int as libc::c_ushort,
    49152 as libc::c_int as libc::c_ushort,
    65535 as libc::c_int as libc::c_ushort,
];
#[no_mangle]
pub static mut SKP_Silk_Seed_offset: libc::c_int = 2 as libc::c_int;
#[no_mangle]
pub static mut SKP_Silk_Quantization_Offsets_Q10: [[libc::c_short; 2]; 2] = [
    [32 as libc::c_int as libc::c_short, 100 as libc::c_int as libc::c_short],
    [100 as libc::c_int as libc::c_short, 256 as libc::c_int as libc::c_short],
];
#[no_mangle]
pub static mut SKP_Silk_LTPScales_table_Q14: [libc::c_short; 3] = [
    15565 as libc::c_int as libc::c_short,
    11469 as libc::c_int as libc::c_short,
    8192 as libc::c_int as libc::c_short,
];
#[no_mangle]
pub static mut SKP_Silk_Transition_LP_B_Q28: [[libc::c_int; 3]; 5] = [
    [250767114 as libc::c_int, 501534038 as libc::c_int, 250767114 as libc::c_int],
    [209867381 as libc::c_int, 419732057 as libc::c_int, 209867381 as libc::c_int],
    [170987846 as libc::c_int, 341967853 as libc::c_int, 170987846 as libc::c_int],
    [131531482 as libc::c_int, 263046905 as libc::c_int, 131531482 as libc::c_int],
    [89306658 as libc::c_int, 178584282 as libc::c_int, 89306658 as libc::c_int],
];
#[no_mangle]
pub static mut SKP_Silk_Transition_LP_A_Q28: [[libc::c_int; 2]; 5] = [
    [506393414 as libc::c_int, 239854379 as libc::c_int],
    [411067935 as libc::c_int, 169683996 as libc::c_int],
    [306733530 as libc::c_int, 116694253 as libc::c_int],
    [185807084 as libc::c_int, 77959395 as libc::c_int],
    [35497197 as libc::c_int, 57401098 as libc::c_int],
];
