#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]

pub static TargetRate_table_NB: [i32; 8] = [
    0,
    8000,
    9000,
    11000,
    13000,
    16000,
    22000,
    100000,
];

pub static TargetRate_table_MB: [i32; 8] = [
    0,
    10000,
    12000,
    14000,
    17000,
    21000,
    28000,
    100000,
];

pub static TargetRate_table_WB: [i32; 8] = [
    0,
    11000,
    14000,
    17000,
    21000,
    26000,
    36000,
    100000,
];

pub static TargetRate_table_SWB: [i32; 8] = [
    0,
    13000,
    16000,
    19000,
    25000,
    32000,
    46000,
    100000,
];

pub static SNR_table_Q1: [i32; 8] = [
    19,
    31,
    35,
    39,
    43,
    47,
    54,
    64,
];

pub static SNR_table_one_bit_per_sample_Q7: [i32; 4] = [
    1984,
    2240,
    2408,
    2708,
];

pub static SKP_Silk_SWB_detect_B_HP_Q13: [[i16; 3]; 3] = [
    [
        575i16,
        -948i16,
        575i16,
    ],
    [
        575i16,
        -221i16,
        575i16,
    ],
    [
        575i16,
        104i16,
        575i16,
    ],
];

pub static SKP_Silk_SWB_detect_A_HP_Q13: [[i16; 2]; 3] = [
    [14613i16, 6868i16],
    [12883i16, 7337i16],
    [11586i16, 7911i16],
];

pub static SKP_Silk_Dec_A_HP_24: [i16; 2] = [
    -16220i16,
    8030i16,
];

pub static SKP_Silk_Dec_B_HP_24: [i16; 3] = [
    8000i16,
    -16000i16,
    8000i16,
];

pub static SKP_Silk_Dec_A_HP_16: [i16; 2] = [
    -16127i16,
    7940i16,
];

pub static SKP_Silk_Dec_B_HP_16: [i16; 3] = [
    8000i16,
    -16000i16,
    8000i16,
];

pub static SKP_Silk_Dec_A_HP_12: [i16; 2] = [
    -16043i16,
    7859i16,
];

pub static SKP_Silk_Dec_B_HP_12: [i16; 3] = [
    8000i16,
    -16000i16,
    8000i16,
];

pub static SKP_Silk_Dec_A_HP_8: [i16; 2] = [
    -15885i16,
    7710i16,
];

pub static SKP_Silk_Dec_B_HP_8: [i16; 3] = [
    8000i16,
    -16000i16,
    8000i16,
];

pub static SKP_Silk_lsb_CDF: [u16; 3] = [
    0u16,
    40000u16,
    65535u16,
];

pub static SKP_Silk_LTPscale_CDF: [u16; 4] = [
    0u16,
    32000u16,
    48000u16,
    65535u16,
];

pub static SKP_Silk_LTPscale_offset: i32 = 2;

pub static SKP_Silk_vadflag_CDF: [u16; 3] = [
    0u16,
    22000u16,
    65535u16,
];

pub static SKP_Silk_vadflag_offset: i32 = 1;

pub static SKP_Silk_SamplingRates_table: [i32; 4] = [
    8,
    12,
    16,
    24,
];

pub static SKP_Silk_SamplingRates_CDF: [u16; 5] = [
    0u16,
    16000u16,
    32000u16,
    48000u16,
    65535u16,
];

pub static SKP_Silk_SamplingRates_offset: i32 = 2;

pub static SKP_Silk_NLSF_interpolation_factor_CDF: [u16; 6] = [
    0u16,
    3706u16,
    8703u16,
    19226u16,
    30926u16,
    65535u16,
];

pub static SKP_Silk_NLSF_interpolation_factor_offset: i32 = 4;

pub static SKP_Silk_FrameTermination_CDF: [u16; 5] = [
    0u16,
    20000u16,
    45000u16,
    56000u16,
    65535u16,
];

pub static SKP_Silk_FrameTermination_offset: i32 = 2;

pub static SKP_Silk_Seed_CDF: [u16; 5] = [
    0u16,
    16384u16,
    32768u16,
    49152u16,
    65535u16,
];

pub static SKP_Silk_Seed_offset: i32 = 2;

pub static SKP_Silk_Quantization_Offsets_Q10: [[i16; 2]; 2] = [
    [32i16, 100i16],
    [100i16, 256i16],
];

pub static SKP_Silk_LTPScales_table_Q14: [i16; 3] = [
    15565i16,
    11469i16,
    8192i16,
];

pub static SKP_Silk_Transition_LP_B_Q28: [[i32; 3]; 5] = [
    [250767114, 501534038, 250767114],
    [209867381, 419732057, 209867381],
    [170987846, 341967853, 170987846],
    [131531482, 263046905, 131531482],
    [89306658, 178584282, 89306658],
];

pub static SKP_Silk_Transition_LP_A_Q28: [[i32; 2]; 5] = [
    [506393414, 239854379],
    [411067935, 169683996],
    [306733530, 116694253],
    [185807084, 77959395],
    [35497197, 57401098],
];
