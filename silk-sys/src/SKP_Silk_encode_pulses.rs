#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    static SKP_Silk_pulses_per_block_CDF: [[libc::c_ushort; 21]; 10];
    static SKP_Silk_pulses_per_block_BITS_Q6: [[libc::c_short; 20]; 9];
    static SKP_Silk_rate_levels_CDF: [[libc::c_ushort; 10]; 2];
    static SKP_Silk_rate_levels_BITS_Q6: [[libc::c_short; 9]; 2];
    static SKP_Silk_max_pulses_table: [libc::c_int; 4];
    static SKP_Silk_lsb_CDF: [libc::c_ushort; 3];
    fn SKP_Silk_encode_signs(
        psRC: *mut SKP_Silk_range_coder_state,
        q: *const libc::c_schar,
        length: libc::c_int,
        sigtype: libc::c_int,
        QuantOffsetType: libc::c_int,
        RateLevelIndex: libc::c_int,
    );
    fn SKP_Silk_range_encoder(
        psRC: *mut SKP_Silk_range_coder_state,
        data: libc::c_int,
        prob: *const libc::c_ushort,
    );
    fn SKP_Silk_shell_encoder(
        psRC: *mut SKP_Silk_range_coder_state,
        pulses0: *const libc::c_int,
    );
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SKP_Silk_range_coder_state {
    pub bufferLength: libc::c_int,
    pub bufferIx: libc::c_int,
    pub base_Q32: libc::c_uint,
    pub range_Q16: libc::c_uint,
    pub error: libc::c_int,
    pub buffer: [libc::c_uchar; 1024],
}
#[inline]
unsafe extern "C" fn combine_and_check(
    mut pulses_comb: *mut libc::c_int,
    mut pulses_in: *const libc::c_int,
    mut max_pulses: libc::c_int,
    mut len: libc::c_int,
) -> libc::c_int {
    let mut k: libc::c_int = 0;
    let mut sum: libc::c_int = 0;
    k = 0 as libc::c_int;
    while k < len {
        sum = *pulses_in.offset((2 as libc::c_int * k) as isize)
            + *pulses_in.offset((2 as libc::c_int * k + 1 as libc::c_int) as isize);
        if sum > max_pulses {
            return 1 as libc::c_int;
        }
        *pulses_comb.offset(k as isize) = sum;
        k += 1;
        k;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_encode_pulses(
    mut psRC: *mut SKP_Silk_range_coder_state,
    sigtype: libc::c_int,
    QuantOffsetType: libc::c_int,
    mut q: *const libc::c_schar,
    frame_length: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut iter: libc::c_int = 0;
    let mut bit: libc::c_int = 0;
    let mut nLS: libc::c_int = 0;
    let mut scale_down: libc::c_int = 0;
    let mut RateLevelIndex: libc::c_int = 0 as libc::c_int;
    let mut abs_q: libc::c_int = 0;
    let mut minSumBits_Q6: libc::c_int = 0;
    let mut sumBits_Q6: libc::c_int = 0;
    let mut abs_pulses: [libc::c_int; 480] = [0; 480];
    let mut sum_pulses: [libc::c_int; 30] = [0; 30];
    let mut nRshifts: [libc::c_int; 30] = [0; 30];
    let mut pulses_comb: [libc::c_int; 8] = [0; 8];
    let mut abs_pulses_ptr: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut pulses_ptr: *const libc::c_schar = 0 as *const libc::c_schar;
    let mut cdf_ptr: *const libc::c_ushort = 0 as *const libc::c_ushort;
    let mut nBits_ptr: *const libc::c_short = 0 as *const libc::c_short;
    memset(
        pulses_comb.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        (8 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    iter = frame_length / 16 as libc::c_int;
    i = 0 as libc::c_int;
    while i < frame_length {
        abs_pulses[(i + 0 as libc::c_int)
            as usize] = if *q.offset((i + 0 as libc::c_int) as isize) as libc::c_int
            > 0 as libc::c_int
        {
            *q.offset((i + 0 as libc::c_int) as isize) as libc::c_int
        } else {
            -(*q.offset((i + 0 as libc::c_int) as isize) as libc::c_int)
        };
        abs_pulses[(i + 1 as libc::c_int)
            as usize] = if *q.offset((i + 1 as libc::c_int) as isize) as libc::c_int
            > 0 as libc::c_int
        {
            *q.offset((i + 1 as libc::c_int) as isize) as libc::c_int
        } else {
            -(*q.offset((i + 1 as libc::c_int) as isize) as libc::c_int)
        };
        abs_pulses[(i + 2 as libc::c_int)
            as usize] = if *q.offset((i + 2 as libc::c_int) as isize) as libc::c_int
            > 0 as libc::c_int
        {
            *q.offset((i + 2 as libc::c_int) as isize) as libc::c_int
        } else {
            -(*q.offset((i + 2 as libc::c_int) as isize) as libc::c_int)
        };
        abs_pulses[(i + 3 as libc::c_int)
            as usize] = if *q.offset((i + 3 as libc::c_int) as isize) as libc::c_int
            > 0 as libc::c_int
        {
            *q.offset((i + 3 as libc::c_int) as isize) as libc::c_int
        } else {
            -(*q.offset((i + 3 as libc::c_int) as isize) as libc::c_int)
        };
        i += 4 as libc::c_int;
    }
    abs_pulses_ptr = abs_pulses.as_mut_ptr();
    i = 0 as libc::c_int;
    while i < iter {
        nRshifts[i as usize] = 0 as libc::c_int;
        loop {
            scale_down = combine_and_check(
                pulses_comb.as_mut_ptr(),
                abs_pulses_ptr,
                SKP_Silk_max_pulses_table[0 as libc::c_int as usize],
                8 as libc::c_int,
            );
            scale_down
                += combine_and_check(
                    pulses_comb.as_mut_ptr(),
                    pulses_comb.as_mut_ptr(),
                    SKP_Silk_max_pulses_table[1 as libc::c_int as usize],
                    4 as libc::c_int,
                );
            scale_down
                += combine_and_check(
                    pulses_comb.as_mut_ptr(),
                    pulses_comb.as_mut_ptr(),
                    SKP_Silk_max_pulses_table[2 as libc::c_int as usize],
                    2 as libc::c_int,
                );
            sum_pulses[i
                as usize] = pulses_comb[0 as libc::c_int as usize]
                + pulses_comb[1 as libc::c_int as usize];
            if sum_pulses[i as usize]
                > SKP_Silk_max_pulses_table[3 as libc::c_int as usize]
            {
                scale_down += 1;
                scale_down;
            }
            if !(scale_down != 0) {
                break;
            }
            nRshifts[i as usize] += 1;
            nRshifts[i as usize];
            k = 0 as libc::c_int;
            while k < 16 as libc::c_int {
                *abs_pulses_ptr
                    .offset(
                        k as isize,
                    ) = *abs_pulses_ptr.offset(k as isize) >> 1 as libc::c_int;
                k += 1;
                k;
            }
        }
        abs_pulses_ptr = abs_pulses_ptr.offset(16 as libc::c_int as isize);
        i += 1;
        i;
    }
    minSumBits_Q6 = 0x7fffffff as libc::c_int;
    k = 0 as libc::c_int;
    while k < 10 as libc::c_int - 1 as libc::c_int {
        nBits_ptr = (SKP_Silk_pulses_per_block_BITS_Q6[k as usize]).as_ptr();
        sumBits_Q6 = SKP_Silk_rate_levels_BITS_Q6[sigtype as usize][k as usize]
            as libc::c_int;
        i = 0 as libc::c_int;
        while i < iter {
            if nRshifts[i as usize] > 0 as libc::c_int {
                sumBits_Q6
                    += *nBits_ptr.offset((18 as libc::c_int + 1 as libc::c_int) as isize)
                        as libc::c_int;
            } else {
                sumBits_Q6
                    += *nBits_ptr.offset(sum_pulses[i as usize] as isize) as libc::c_int;
            }
            i += 1;
            i;
        }
        if sumBits_Q6 < minSumBits_Q6 {
            minSumBits_Q6 = sumBits_Q6;
            RateLevelIndex = k;
        }
        k += 1;
        k;
    }
    SKP_Silk_range_encoder(
        psRC,
        RateLevelIndex,
        (SKP_Silk_rate_levels_CDF[sigtype as usize]).as_ptr(),
    );
    cdf_ptr = (SKP_Silk_pulses_per_block_CDF[RateLevelIndex as usize]).as_ptr();
    i = 0 as libc::c_int;
    while i < iter {
        if nRshifts[i as usize] == 0 as libc::c_int {
            SKP_Silk_range_encoder(psRC, sum_pulses[i as usize], cdf_ptr);
        } else {
            SKP_Silk_range_encoder(psRC, 18 as libc::c_int + 1 as libc::c_int, cdf_ptr);
            k = 0 as libc::c_int;
            while k < nRshifts[i as usize] - 1 as libc::c_int {
                SKP_Silk_range_encoder(
                    psRC,
                    18 as libc::c_int + 1 as libc::c_int,
                    (SKP_Silk_pulses_per_block_CDF[(10 as libc::c_int - 1 as libc::c_int)
                        as usize])
                        .as_ptr(),
                );
                k += 1;
                k;
            }
            SKP_Silk_range_encoder(
                psRC,
                sum_pulses[i as usize],
                (SKP_Silk_pulses_per_block_CDF[(10 as libc::c_int - 1 as libc::c_int)
                    as usize])
                    .as_ptr(),
            );
        }
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < iter {
        if sum_pulses[i as usize] > 0 as libc::c_int {
            SKP_Silk_shell_encoder(
                psRC,
                &mut abs_pulses[(i * 16) as usize..],
            );
        }
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < iter {
        if nRshifts[i as usize] > 0 as libc::c_int {
            pulses_ptr = &*q.offset((i * 16 as libc::c_int) as isize)
                as *const libc::c_schar;
            nLS = nRshifts[i as usize] - 1 as libc::c_int;
            k = 0 as libc::c_int;
            while k < 16 as libc::c_int {
                abs_q = (if *pulses_ptr.offset(k as isize) as libc::c_int
                    > 0 as libc::c_int
                {
                    *pulses_ptr.offset(k as isize) as libc::c_int
                } else {
                    -(*pulses_ptr.offset(k as isize) as libc::c_int)
                }) as libc::c_schar as libc::c_int;
                j = nLS;
                while j > 0 as libc::c_int {
                    bit = abs_q >> j & 1 as libc::c_int;
                    SKP_Silk_range_encoder(psRC, bit, SKP_Silk_lsb_CDF.as_ptr());
                    j -= 1;
                    j;
                }
                bit = abs_q & 1 as libc::c_int;
                SKP_Silk_range_encoder(psRC, bit, SKP_Silk_lsb_CDF.as_ptr());
                k += 1;
                k;
            }
        }
        i += 1;
        i;
    }
    SKP_Silk_encode_signs(
        psRC,
        q,
        frame_length,
        sigtype,
        QuantOffsetType,
        RateLevelIndex,
    );
}
