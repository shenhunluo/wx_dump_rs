#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn SKP_Silk_insertion_sort_decreasing_int16(
        a: *mut libc::c_short,
        index: *mut libc::c_int,
        L: libc::c_int,
        K: libc::c_int,
    );
    fn SKP_Silk_inner_prod_aligned(
        inVec1: *const libc::c_short,
        inVec2: *const libc::c_short,
        len: libc::c_int,
    ) -> libc::c_int;
    fn SKP_Silk_int16_array_maxabs(
        vec: *const libc::c_short,
        len: libc::c_int,
    ) -> libc::c_short;
    fn SKP_Silk_lin2log(inLin: libc::c_int) -> libc::c_int;
    fn SKP_Silk_resampler_down2(
        S: *mut libc::c_int,
        out: *mut libc::c_short,
        in_0: *const libc::c_short,
        inLen: libc::c_int,
    );
    fn SKP_Silk_resampler_down2_3(
        S: *mut libc::c_int,
        out: *mut libc::c_short,
        in_0: *const libc::c_short,
        inLen: libc::c_int,
    );
    fn SKP_Silk_resampler_down3(
        S: *mut libc::c_int,
        out: *mut libc::c_short,
        in_0: *const libc::c_short,
        inLen: libc::c_int,
    );
    static SKP_Silk_CB_lags_stage2: [[libc::c_short; 11]; 4];
    static SKP_Silk_CB_lags_stage3: [[libc::c_short; 34]; 4];
    static SKP_Silk_Lag_range_stage3: [[[libc::c_short; 2]; 4]; 3];
    static SKP_Silk_cbk_sizes_stage3: [libc::c_short; 3];
    static SKP_Silk_cbk_offsets_stage3: [libc::c_short; 3];
}
#[inline]
unsafe extern "C" fn SKP_max_32(mut a: libc::c_int, mut b: libc::c_int) -> libc::c_int {
    return if a > b { a } else { b };
}
#[inline]
unsafe extern "C" fn SKP_max_int(mut a: libc::c_int, mut b: libc::c_int) -> libc::c_int {
    return if a > b { a } else { b };
}
#[inline]
unsafe extern "C" fn SKP_min_int(mut a: libc::c_int, mut b: libc::c_int) -> libc::c_int {
    return if a < b { a } else { b };
}
#[inline]
unsafe extern "C" fn SKP_ROR32(
    mut a32: libc::c_int,
    mut rot: libc::c_int,
) -> libc::c_int {
    let mut x: libc::c_uint = a32 as libc::c_uint;
    let mut r: libc::c_uint = rot as libc::c_uint;
    let mut m: libc::c_uint = -rot as libc::c_uint;
    if rot <= 0 as libc::c_int {
        return (x << m | x >> (32 as libc::c_int as libc::c_uint).wrapping_sub(m))
            as libc::c_int
    } else {
        return (x << (32 as libc::c_int as libc::c_uint).wrapping_sub(r) | x >> r)
            as libc::c_int
    };
}
#[inline]
unsafe extern "C" fn SKP_Silk_CLZ_FRAC(
    mut in_0: libc::c_int,
    mut lz: *mut libc::c_int,
    mut frac_Q7: *mut libc::c_int,
) {
    let mut lzeros: libc::c_int = SKP_Silk_CLZ32(in_0);
    *lz = lzeros;
    *frac_Q7 = SKP_ROR32(in_0, 24 as libc::c_int - lzeros) & 0x7f as libc::c_int;
}
#[inline]
unsafe extern "C" fn SKP_Silk_SQRT_APPROX(mut x: libc::c_int) -> libc::c_int {
    let mut y: libc::c_int = 0;
    let mut lz: libc::c_int = 0;
    let mut frac_Q7: libc::c_int = 0;
    if x <= 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    SKP_Silk_CLZ_FRAC(x, &mut lz, &mut frac_Q7);
    if lz & 1 as libc::c_int != 0 {
        y = 32768 as libc::c_int;
    } else {
        y = 46214 as libc::c_int;
    }
    y >>= lz >> 1 as libc::c_int;
    y = y
        + ((y >> 16 as libc::c_int)
            * (213 as libc::c_int as libc::c_short as libc::c_int
                * frac_Q7 as libc::c_short as libc::c_int) as libc::c_short
                as libc::c_int
            + ((y & 0xffff as libc::c_int)
                * (213 as libc::c_int as libc::c_short as libc::c_int
                    * frac_Q7 as libc::c_short as libc::c_int) as libc::c_short
                    as libc::c_int >> 16 as libc::c_int));
    return y;
}
#[inline]
unsafe extern "C" fn SKP_Silk_CLZ16(mut in16: libc::c_short) -> libc::c_int {
    let mut out32: libc::c_int = 0 as libc::c_int;
    if in16 as libc::c_int == 0 as libc::c_int {
        return 16 as libc::c_int;
    }
    if in16 as libc::c_int & 0xff00 as libc::c_int != 0 {
        if in16 as libc::c_int & 0xf000 as libc::c_int != 0 {
            in16 = (in16 as libc::c_int >> 12 as libc::c_int) as libc::c_short;
        } else {
            out32 += 4 as libc::c_int;
            in16 = (in16 as libc::c_int >> 8 as libc::c_int) as libc::c_short;
        }
    } else if in16 as libc::c_int & 0xfff0 as libc::c_int != 0 {
        out32 += 8 as libc::c_int;
        in16 = (in16 as libc::c_int >> 4 as libc::c_int) as libc::c_short;
    } else {
        out32 += 12 as libc::c_int;
    }
    if in16 as libc::c_int & 0xc as libc::c_int != 0 {
        if in16 as libc::c_int & 0x8 as libc::c_int != 0 {
            return out32 + 0 as libc::c_int
        } else {
            return out32 + 1 as libc::c_int
        }
    } else if in16 as libc::c_int & 0xe as libc::c_int != 0 {
        return out32 + 2 as libc::c_int
    } else {
        return out32 + 3 as libc::c_int
    };
}
#[inline]
unsafe extern "C" fn SKP_Silk_CLZ32(mut in32: libc::c_int) -> libc::c_int {
    if in32 as libc::c_uint & 0xffff0000 as libc::c_uint != 0 {
        return SKP_Silk_CLZ16((in32 >> 16 as libc::c_int) as libc::c_short)
    } else {
        return SKP_Silk_CLZ16(in32 as libc::c_short) + 16 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_pitch_analysis_core(
    mut signal: *const libc::c_short,
    mut pitch_out: *mut libc::c_int,
    mut lagIndex: *mut libc::c_int,
    mut contourIndex: *mut libc::c_int,
    mut LTPCorr_Q15: *mut libc::c_int,
    mut prevLag: libc::c_int,
    search_thres1_Q16: libc::c_int,
    search_thres2_Q15: libc::c_int,
    Fs_kHz: libc::c_int,
    complexity: libc::c_int,
    forLJC: libc::c_int,
) -> libc::c_int {
    let mut signal_8kHz: [libc::c_short; 480] = [0; 480];
    let mut signal_4kHz: [libc::c_short; 240] = [0; 240];
    let mut scratch_mem: [libc::c_int; 2880] = [0; 2880];
    let mut input_signal_ptr: *mut libc::c_short = 0 as *mut libc::c_short;
    let mut filt_state: [libc::c_int; 7] = [0; 7];
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut d: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut C: [[libc::c_short; 221]; 4] = [[0; 221]; 4];
    let mut target_ptr: *const libc::c_short = 0 as *const libc::c_short;
    let mut basis_ptr: *const libc::c_short = 0 as *const libc::c_short;
    let mut cross_corr: libc::c_int = 0;
    let mut normalizer: libc::c_int = 0;
    let mut energy: libc::c_int = 0;
    let mut shift: libc::c_int = 0;
    let mut energy_basis: libc::c_int = 0;
    let mut energy_target: libc::c_int = 0;
    let mut d_srch: [libc::c_int; 24] = [0; 24];
    let mut d_comp: [libc::c_short; 221] = [0; 221];
    let mut Cmax: libc::c_int = 0;
    let mut length_d_srch: libc::c_int = 0;
    let mut length_d_comp: libc::c_int = 0;
    let mut sum: libc::c_int = 0;
    let mut threshold: libc::c_int = 0;
    let mut temp32: libc::c_int = 0;
    let mut CBimax: libc::c_int = 0;
    let mut CBimax_new: libc::c_int = 0;
    let mut CBimax_old: libc::c_int = 0;
    let mut lag: libc::c_int = 0;
    let mut start_lag: libc::c_int = 0;
    let mut end_lag: libc::c_int = 0;
    let mut lag_new: libc::c_int = 0;
    let mut CC: [libc::c_int; 11] = [0; 11];
    let mut CCmax: libc::c_int = 0;
    let mut CCmax_b: libc::c_int = 0;
    let mut CCmax_new_b: libc::c_int = 0;
    let mut CCmax_new: libc::c_int = 0;
    let mut energies_st3: [[[libc::c_int; 5]; 34]; 4] = [[[0; 5]; 34]; 4];
    let mut crosscorr_st3: [[[libc::c_int; 5]; 34]; 4] = [[[0; 5]; 34]; 4];
    let mut lag_counter: libc::c_int = 0;
    let mut frame_length: libc::c_int = 0;
    let mut frame_length_8kHz: libc::c_int = 0;
    let mut frame_length_4kHz: libc::c_int = 0;
    let mut max_sum_sq_length: libc::c_int = 0;
    let mut sf_length: libc::c_int = 0;
    let mut sf_length_8kHz: libc::c_int = 0;
    let mut min_lag: libc::c_int = 0;
    let mut min_lag_8kHz: libc::c_int = 0;
    let mut min_lag_4kHz: libc::c_int = 0;
    let mut max_lag: libc::c_int = 0;
    let mut max_lag_8kHz: libc::c_int = 0;
    let mut max_lag_4kHz: libc::c_int = 0;
    let mut contour_bias: libc::c_int = 0;
    let mut diff: libc::c_int = 0;
    let mut lz: libc::c_int = 0;
    let mut lshift: libc::c_int = 0;
    let mut cbk_offset: libc::c_int = 0;
    let mut cbk_size: libc::c_int = 0;
    let mut nb_cbks_stage2: libc::c_int = 0;
    let mut delta_lag_log2_sqr_Q7: libc::c_int = 0;
    let mut lag_log2_Q7: libc::c_int = 0;
    let mut prevLag_log2_Q7: libc::c_int = 0;
    let mut prev_lag_bias_Q15: libc::c_int = 0;
    let mut corr_thres_Q15: libc::c_int = 0;
    frame_length = 40 as libc::c_int * Fs_kHz;
    frame_length_4kHz = 40 as libc::c_int * 4 as libc::c_int;
    frame_length_8kHz = 40 as libc::c_int * 8 as libc::c_int;
    sf_length = frame_length >> 3 as libc::c_int;
    sf_length_8kHz = frame_length_8kHz >> 3 as libc::c_int;
    min_lag = 2 as libc::c_int * Fs_kHz;
    min_lag_4kHz = 2 as libc::c_int * 4 as libc::c_int;
    min_lag_8kHz = 2 as libc::c_int * 8 as libc::c_int;
    max_lag = 18 as libc::c_int * Fs_kHz;
    max_lag_4kHz = 18 as libc::c_int * 4 as libc::c_int;
    max_lag_8kHz = 18 as libc::c_int * 8 as libc::c_int;
    memset(
        C.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        (::core::mem::size_of::<libc::c_short>() as libc::c_ulong)
            .wrapping_mul(4 as libc::c_int as libc::c_ulong)
            .wrapping_mul(
                ((18 as libc::c_int * 24 as libc::c_int >> 1 as libc::c_int)
                    + 5 as libc::c_int) as libc::c_ulong,
            ),
    );
    if Fs_kHz == 16 as libc::c_int {
        memset(
            filt_state.as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
        );
        SKP_Silk_resampler_down2(
            filt_state.as_mut_ptr(),
            signal_8kHz.as_mut_ptr(),
            signal,
            frame_length,
        );
    } else if Fs_kHz == 12 as libc::c_int {
        let mut R23: [libc::c_int; 6] = [0; 6];
        memset(
            R23.as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            (6 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
        );
        SKP_Silk_resampler_down2_3(
            R23.as_mut_ptr(),
            signal_8kHz.as_mut_ptr(),
            signal,
            40 as libc::c_int * 12 as libc::c_int,
        );
    } else if Fs_kHz == 24 as libc::c_int {
        let mut filt_state_fix: [libc::c_int; 8] = [0; 8];
        memset(
            filt_state_fix.as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            (8 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
        );
        SKP_Silk_resampler_down3(
            filt_state_fix.as_mut_ptr(),
            signal_8kHz.as_mut_ptr(),
            signal,
            24 as libc::c_int * 40 as libc::c_int,
        );
    } else {
        memcpy(
            signal_8kHz.as_mut_ptr() as *mut libc::c_void,
            signal as *const libc::c_void,
            (frame_length_8kHz as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_short>() as libc::c_ulong),
        );
    }
    memset(
        filt_state.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        (2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    SKP_Silk_resampler_down2(
        filt_state.as_mut_ptr(),
        signal_4kHz.as_mut_ptr(),
        signal_8kHz.as_mut_ptr(),
        frame_length_8kHz,
    );
    i = frame_length_4kHz - 1 as libc::c_int;
    while i > 0 as libc::c_int {
        signal_4kHz[i
            as usize] = (if signal_4kHz[i as usize] as libc::c_int
            + signal_4kHz[(i - 1 as libc::c_int) as usize] as libc::c_int
            > 0x7fff as libc::c_int
        {
            0x7fff as libc::c_int
        } else if (signal_4kHz[i as usize] as libc::c_int
            + signal_4kHz[(i - 1 as libc::c_int) as usize] as libc::c_int)
            < 0x8000 as libc::c_int as libc::c_short as libc::c_int
        {
            0x8000 as libc::c_int as libc::c_short as libc::c_int
        } else {
            signal_4kHz[i as usize] as libc::c_int
                + signal_4kHz[(i - 1 as libc::c_int) as usize] as libc::c_int
        }) as libc::c_short;
        i -= 1;
        i;
    }
    max_sum_sq_length = SKP_max_32(
        sf_length_8kHz,
        frame_length_4kHz >> 1 as libc::c_int,
    );
    shift = SKP_FIX_P_Ana_find_scaling(
        signal_4kHz.as_mut_ptr(),
        frame_length_4kHz,
        max_sum_sq_length,
    );
    if shift > 0 as libc::c_int {
        i = 0 as libc::c_int;
        while i < frame_length_4kHz {
            signal_4kHz[i
                as usize] = (signal_4kHz[i as usize] as libc::c_int >> shift)
                as libc::c_short;
            i += 1;
            i;
        }
    }
    target_ptr = &mut *signal_4kHz
        .as_mut_ptr()
        .offset((frame_length_4kHz >> 1 as libc::c_int) as isize) as *mut libc::c_short;
    k = 0 as libc::c_int;
    while k < 2 as libc::c_int {
        basis_ptr = target_ptr.offset(-(min_lag_4kHz as isize));
        normalizer = 0 as libc::c_int;
        cross_corr = 0 as libc::c_int;
        cross_corr = SKP_Silk_inner_prod_aligned(target_ptr, basis_ptr, sf_length_8kHz);
        normalizer = SKP_Silk_inner_prod_aligned(basis_ptr, basis_ptr, sf_length_8kHz);
        normalizer = if (normalizer
            + sf_length_8kHz as libc::c_short as libc::c_int
                * 4000 as libc::c_int as libc::c_short as libc::c_int) as libc::c_uint
            & 0x80000000 as libc::c_uint == 0 as libc::c_int as libc::c_uint
        {
            if (normalizer
                & sf_length_8kHz as libc::c_short as libc::c_int
                    * 4000 as libc::c_int as libc::c_short as libc::c_int)
                as libc::c_uint & 0x80000000 as libc::c_uint
                != 0 as libc::c_int as libc::c_uint
            {
                0x80000000 as libc::c_uint as libc::c_int
            } else {
                normalizer
                    + sf_length_8kHz as libc::c_short as libc::c_int
                        * 4000 as libc::c_int as libc::c_short as libc::c_int
            }
        } else if (normalizer
            | sf_length_8kHz as libc::c_short as libc::c_int
                * 4000 as libc::c_int as libc::c_short as libc::c_int) as libc::c_uint
            & 0x80000000 as libc::c_uint == 0 as libc::c_int as libc::c_uint
        {
            0x7fffffff as libc::c_int
        } else {
            normalizer
                + sf_length_8kHz as libc::c_short as libc::c_int
                    * 4000 as libc::c_int as libc::c_short as libc::c_int
        };
        temp32 = cross_corr / (SKP_Silk_SQRT_APPROX(normalizer) + 1 as libc::c_int);
        C[k
            as usize][min_lag_4kHz
            as usize] = (if temp32 > 0x7fff as libc::c_int {
            0x7fff as libc::c_int
        } else if temp32 < 0x8000 as libc::c_int as libc::c_short as libc::c_int {
            0x8000 as libc::c_int as libc::c_short as libc::c_int
        } else {
            temp32
        }) as libc::c_short;
        d = min_lag_4kHz + 1 as libc::c_int;
        while d <= max_lag_4kHz {
            basis_ptr = basis_ptr.offset(-1);
            basis_ptr;
            cross_corr = SKP_Silk_inner_prod_aligned(
                target_ptr,
                basis_ptr,
                sf_length_8kHz,
            );
            normalizer
                += *basis_ptr.offset(0 as libc::c_int as isize) as libc::c_int
                    * *basis_ptr.offset(0 as libc::c_int as isize) as libc::c_int
                    - *basis_ptr.offset(sf_length_8kHz as isize) as libc::c_int
                        * *basis_ptr.offset(sf_length_8kHz as isize) as libc::c_int;
            temp32 = cross_corr / (SKP_Silk_SQRT_APPROX(normalizer) + 1 as libc::c_int);
            C[k
                as usize][d
                as usize] = (if temp32 > 0x7fff as libc::c_int {
                0x7fff as libc::c_int
            } else if temp32 < 0x8000 as libc::c_int as libc::c_short as libc::c_int {
                0x8000 as libc::c_int as libc::c_short as libc::c_int
            } else {
                temp32
            }) as libc::c_short;
            d += 1;
            d;
        }
        target_ptr = target_ptr.offset(sf_length_8kHz as isize);
        k += 1;
        k;
    }
    i = max_lag_4kHz;
    while i >= min_lag_4kHz {
        sum = C[0 as libc::c_int as usize][i as usize] as libc::c_int
            + C[1 as libc::c_int as usize][i as usize] as libc::c_int;
        sum = sum >> 1 as libc::c_int;
        sum = sum
            + ((sum >> 16 as libc::c_int)
                * (-i << 4 as libc::c_int) as libc::c_short as libc::c_int
                + ((sum & 0xffff as libc::c_int)
                    * (-i << 4 as libc::c_int) as libc::c_short as libc::c_int
                    >> 16 as libc::c_int));
        C[0 as libc::c_int as usize][i as usize] = sum as libc::c_short;
        i -= 1;
        i;
    }
    length_d_srch = 4 as libc::c_int + 2 as libc::c_int * complexity;
    SKP_Silk_insertion_sort_decreasing_int16(
        &mut *(*C.as_mut_ptr().offset(0 as libc::c_int as isize))
            .as_mut_ptr()
            .offset(min_lag_4kHz as isize),
        d_srch.as_mut_ptr(),
        max_lag_4kHz - min_lag_4kHz + 1 as libc::c_int,
        length_d_srch,
    );
    target_ptr = &mut *signal_4kHz
        .as_mut_ptr()
        .offset((frame_length_4kHz >> 1 as libc::c_int) as isize) as *mut libc::c_short;
    energy = SKP_Silk_inner_prod_aligned(
        target_ptr,
        target_ptr,
        frame_length_4kHz >> 1 as libc::c_int,
    );
    energy = if (energy + 1000 as libc::c_int) as libc::c_uint
        & 0x80000000 as libc::c_uint != 0
    {
        0x7fffffff as libc::c_int
    } else {
        energy + 1000 as libc::c_int
    };
    Cmax = C[0 as libc::c_int as usize][min_lag_4kHz as usize] as libc::c_int;
    threshold = Cmax as libc::c_short as libc::c_int
        * Cmax as libc::c_short as libc::c_int;
    if energy >> 4 as libc::c_int + 2 as libc::c_int > threshold {
        memset(
            pitch_out as *mut libc::c_void,
            0 as libc::c_int,
            (4 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
        );
        *LTPCorr_Q15 = 0 as libc::c_int;
        *lagIndex = 0 as libc::c_int;
        *contourIndex = 0 as libc::c_int;
        return 1 as libc::c_int;
    }
    threshold = (search_thres1_Q16 >> 16 as libc::c_int)
        * Cmax as libc::c_short as libc::c_int
        + ((search_thres1_Q16 & 0xffff as libc::c_int)
            * Cmax as libc::c_short as libc::c_int >> 16 as libc::c_int);
    i = 0 as libc::c_int;
    while i < length_d_srch {
        if C[0 as libc::c_int as usize][(min_lag_4kHz + i) as usize] as libc::c_int
            > threshold
        {
            d_srch[i as usize] = d_srch[i as usize] + min_lag_4kHz << 1 as libc::c_int;
            i += 1;
            i;
        } else {
            length_d_srch = i;
            break;
        }
    }
    i = min_lag_8kHz - 5 as libc::c_int;
    while i < max_lag_8kHz + 5 as libc::c_int {
        d_comp[i as usize] = 0 as libc::c_int as libc::c_short;
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < length_d_srch {
        d_comp[d_srch[i as usize] as usize] = 1 as libc::c_int as libc::c_short;
        i += 1;
        i;
    }
    i = max_lag_8kHz + 3 as libc::c_int;
    while i >= min_lag_8kHz {
        d_comp[i
            as usize] = (d_comp[i as usize] as libc::c_int
            + (d_comp[(i - 1 as libc::c_int) as usize] as libc::c_int
                + d_comp[(i - 2 as libc::c_int) as usize] as libc::c_int))
            as libc::c_short;
        i -= 1;
        i;
    }
    length_d_srch = 0 as libc::c_int;
    i = min_lag_8kHz;
    while i < max_lag_8kHz + 1 as libc::c_int {
        if d_comp[(i + 1 as libc::c_int) as usize] as libc::c_int > 0 as libc::c_int {
            d_srch[length_d_srch as usize] = i;
            length_d_srch += 1;
            length_d_srch;
        }
        i += 1;
        i;
    }
    i = max_lag_8kHz + 3 as libc::c_int;
    while i >= min_lag_8kHz {
        d_comp[i
            as usize] = (d_comp[i as usize] as libc::c_int
            + (d_comp[(i - 1 as libc::c_int) as usize] as libc::c_int
                + d_comp[(i - 2 as libc::c_int) as usize] as libc::c_int
                + d_comp[(i - 3 as libc::c_int) as usize] as libc::c_int))
            as libc::c_short;
        i -= 1;
        i;
    }
    length_d_comp = 0 as libc::c_int;
    i = min_lag_8kHz;
    while i < max_lag_8kHz + 4 as libc::c_int {
        if d_comp[i as usize] as libc::c_int > 0 as libc::c_int {
            d_comp[length_d_comp as usize] = (i - 2 as libc::c_int) as libc::c_short;
            length_d_comp += 1;
            length_d_comp;
        }
        i += 1;
        i;
    }
    shift = SKP_FIX_P_Ana_find_scaling(
        signal_8kHz.as_mut_ptr(),
        frame_length_8kHz,
        sf_length_8kHz,
    );
    if shift > 0 as libc::c_int {
        i = 0 as libc::c_int;
        while i < frame_length_8kHz {
            signal_8kHz[i
                as usize] = (signal_8kHz[i as usize] as libc::c_int >> shift)
                as libc::c_short;
            i += 1;
            i;
        }
    }
    memset(
        C.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ((4 as libc::c_int
            * ((18 as libc::c_int * 24 as libc::c_int >> 1 as libc::c_int)
                + 5 as libc::c_int)) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as libc::c_ulong),
    );
    target_ptr = &mut *signal_8kHz.as_mut_ptr().offset(frame_length_4kHz as isize)
        as *mut libc::c_short;
    k = 0 as libc::c_int;
    while k < 4 as libc::c_int {
        energy_target = SKP_Silk_inner_prod_aligned(
            target_ptr,
            target_ptr,
            sf_length_8kHz,
        );
        j = 0 as libc::c_int;
        while j < length_d_comp {
            d = d_comp[j as usize] as libc::c_int;
            basis_ptr = target_ptr.offset(-(d as isize));
            cross_corr = SKP_Silk_inner_prod_aligned(
                target_ptr,
                basis_ptr,
                sf_length_8kHz,
            );
            energy_basis = SKP_Silk_inner_prod_aligned(
                basis_ptr,
                basis_ptr,
                sf_length_8kHz,
            );
            if cross_corr > 0 as libc::c_int {
                energy = if energy_target > energy_basis {
                    energy_target
                } else {
                    energy_basis
                };
                lz = SKP_Silk_CLZ32(cross_corr);
                lshift = if 0 as libc::c_int > 15 as libc::c_int {
                    if lz - 1 as libc::c_int > 0 as libc::c_int {
                        0 as libc::c_int
                    } else if (lz - 1 as libc::c_int) < 15 as libc::c_int {
                        15 as libc::c_int
                    } else {
                        lz - 1 as libc::c_int
                    }
                } else if lz - 1 as libc::c_int > 15 as libc::c_int {
                    15 as libc::c_int
                } else if (lz - 1 as libc::c_int) < 0 as libc::c_int {
                    0 as libc::c_int
                } else {
                    lz - 1 as libc::c_int
                };
                temp32 = (cross_corr << lshift)
                    / ((energy >> 15 as libc::c_int - lshift) + 1 as libc::c_int);
                temp32 = (cross_corr >> 16 as libc::c_int)
                    * temp32 as libc::c_short as libc::c_int
                    + ((cross_corr & 0xffff as libc::c_int)
                        * temp32 as libc::c_short as libc::c_int >> 16 as libc::c_int);
                temp32 = if (temp32 + temp32) as libc::c_uint
                    & 0x80000000 as libc::c_uint == 0 as libc::c_int as libc::c_uint
                {
                    if (temp32 & temp32) as libc::c_uint & 0x80000000 as libc::c_uint
                        != 0 as libc::c_int as libc::c_uint
                    {
                        0x80000000 as libc::c_uint as libc::c_int
                    } else {
                        temp32 + temp32
                    }
                } else if (temp32 | temp32) as libc::c_uint & 0x80000000 as libc::c_uint
                    == 0 as libc::c_int as libc::c_uint
                {
                    0x7fffffff as libc::c_int
                } else {
                    temp32 + temp32
                };
                lz = SKP_Silk_CLZ32(temp32);
                lshift = if 0 as libc::c_int > 15 as libc::c_int {
                    if lz - 1 as libc::c_int > 0 as libc::c_int {
                        0 as libc::c_int
                    } else if (lz - 1 as libc::c_int) < 15 as libc::c_int {
                        15 as libc::c_int
                    } else {
                        lz - 1 as libc::c_int
                    }
                } else if lz - 1 as libc::c_int > 15 as libc::c_int {
                    15 as libc::c_int
                } else if (lz - 1 as libc::c_int) < 0 as libc::c_int {
                    0 as libc::c_int
                } else {
                    lz - 1 as libc::c_int
                };
                energy = if energy_target < energy_basis {
                    energy_target
                } else {
                    energy_basis
                };
                C[k
                    as usize][d
                    as usize] = ((temp32 << lshift)
                    / ((energy >> 15 as libc::c_int - lshift) + 1 as libc::c_int))
                    as libc::c_short;
            } else {
                C[k as usize][d as usize] = 0 as libc::c_int as libc::c_short;
            }
            j += 1;
            j;
        }
        target_ptr = target_ptr.offset(sf_length_8kHz as isize);
        k += 1;
        k;
    }
    CCmax = 0x80000000 as libc::c_uint as libc::c_int;
    CCmax_b = 0x80000000 as libc::c_uint as libc::c_int;
    CBimax = 0 as libc::c_int;
    lag = -(1 as libc::c_int);
    if prevLag > 0 as libc::c_int {
        if Fs_kHz == 12 as libc::c_int {
            prevLag = (prevLag << 1 as libc::c_int) / 3 as libc::c_int;
        } else if Fs_kHz == 16 as libc::c_int {
            prevLag = prevLag >> 1 as libc::c_int;
        } else if Fs_kHz == 24 as libc::c_int {
            prevLag = prevLag / 3 as libc::c_int;
        }
        prevLag_log2_Q7 = SKP_Silk_lin2log(prevLag);
    } else {
        prevLag_log2_Q7 = 0 as libc::c_int;
    }
    corr_thres_Q15 = search_thres2_Q15 as libc::c_short as libc::c_int
        * search_thres2_Q15 as libc::c_short as libc::c_int >> 13 as libc::c_int;
    if Fs_kHz == 8 as libc::c_int && complexity > 0 as libc::c_int {
        nb_cbks_stage2 = 11 as libc::c_int;
    } else {
        nb_cbks_stage2 = 3 as libc::c_int;
    }
    k = 0 as libc::c_int;
    while k < length_d_srch {
        d = d_srch[k as usize];
        j = 0 as libc::c_int;
        while j < nb_cbks_stage2 {
            CC[j as usize] = 0 as libc::c_int;
            i = 0 as libc::c_int;
            while i < 4 as libc::c_int {
                CC[j
                    as usize] = CC[j as usize]
                    + C[i
                        as usize][(d
                        + SKP_Silk_CB_lags_stage2[i as usize][j as usize] as libc::c_int)
                        as usize] as libc::c_int;
                i += 1;
                i;
            }
            j += 1;
            j;
        }
        CCmax_new = 0x80000000 as libc::c_uint as libc::c_int;
        CBimax_new = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while i < nb_cbks_stage2 {
            if CC[i as usize] > CCmax_new {
                CCmax_new = CC[i as usize];
                CBimax_new = i;
            }
            i += 1;
            i;
        }
        lag_log2_Q7 = SKP_Silk_lin2log(d);
        if forLJC != 0 {
            CCmax_new_b = CCmax_new;
        } else {
            CCmax_new_b = CCmax_new
                - ((4 as libc::c_int * 6554 as libc::c_int) as libc::c_short
                    as libc::c_int * lag_log2_Q7 as libc::c_short as libc::c_int
                    >> 7 as libc::c_int);
        }
        if prevLag > 0 as libc::c_int {
            delta_lag_log2_sqr_Q7 = lag_log2_Q7 - prevLag_log2_Q7;
            delta_lag_log2_sqr_Q7 = delta_lag_log2_sqr_Q7 as libc::c_short as libc::c_int
                * delta_lag_log2_sqr_Q7 as libc::c_short as libc::c_int
                >> 7 as libc::c_int;
            prev_lag_bias_Q15 = (4 as libc::c_int * 6554 as libc::c_int) as libc::c_short
                as libc::c_int * *LTPCorr_Q15 as libc::c_short as libc::c_int
                >> 15 as libc::c_int;
            prev_lag_bias_Q15 = prev_lag_bias_Q15 * delta_lag_log2_sqr_Q7
                / (delta_lag_log2_sqr_Q7 + ((1 as libc::c_int) << 6 as libc::c_int));
            CCmax_new_b -= prev_lag_bias_Q15;
        }
        if CCmax_new_b > CCmax_b && CCmax_new > corr_thres_Q15
            && SKP_Silk_CB_lags_stage2[0 as libc::c_int as usize][CBimax_new as usize]
                as libc::c_int <= min_lag_8kHz
        {
            CCmax_b = CCmax_new_b;
            CCmax = CCmax_new;
            lag = d;
            CBimax = CBimax_new;
        }
        k += 1;
        k;
    }
    if lag == -(1 as libc::c_int) {
        memset(
            pitch_out as *mut libc::c_void,
            0 as libc::c_int,
            (4 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
        );
        *LTPCorr_Q15 = 0 as libc::c_int;
        *lagIndex = 0 as libc::c_int;
        *contourIndex = 0 as libc::c_int;
        return 1 as libc::c_int;
    }
    if Fs_kHz > 8 as libc::c_int {
        shift = SKP_FIX_P_Ana_find_scaling(signal, frame_length, sf_length);
        if shift > 0 as libc::c_int {
            input_signal_ptr = scratch_mem.as_mut_ptr() as *mut libc::c_short;
            i = 0 as libc::c_int;
            while i < frame_length {
                *input_signal_ptr
                    .offset(
                        i as isize,
                    ) = (*signal.offset(i as isize) as libc::c_int >> shift)
                    as libc::c_short;
                i += 1;
                i;
            }
        } else {
            input_signal_ptr = signal as *mut libc::c_short;
        }
        CBimax_old = CBimax;
        if Fs_kHz == 12 as libc::c_int {
            lag = lag as libc::c_short as libc::c_int
                * 3 as libc::c_int as libc::c_short as libc::c_int >> 1 as libc::c_int;
        } else if Fs_kHz == 16 as libc::c_int {
            lag = lag << 1 as libc::c_int;
        } else {
            lag = lag as libc::c_short as libc::c_int
                * 3 as libc::c_int as libc::c_short as libc::c_int;
        }
        lag = if min_lag > max_lag {
            if lag > min_lag { min_lag } else if lag < max_lag { max_lag } else { lag }
        } else if lag > max_lag {
            max_lag
        } else if lag < min_lag {
            min_lag
        } else {
            lag
        };
        start_lag = SKP_max_int(lag - 2 as libc::c_int, min_lag);
        end_lag = SKP_min_int(lag + 2 as libc::c_int, max_lag);
        lag_new = lag;
        CBimax = 0 as libc::c_int;
        *LTPCorr_Q15 = SKP_Silk_SQRT_APPROX(CCmax << 13 as libc::c_int);
        CCmax = 0x80000000 as libc::c_uint as libc::c_int;
        k = 0 as libc::c_int;
        while k < 4 as libc::c_int {
            *pitch_out
                .offset(
                    k as isize,
                ) = lag
                + 2 as libc::c_int
                    * SKP_Silk_CB_lags_stage2[k as usize][CBimax_old as usize]
                        as libc::c_int;
            k += 1;
            k;
        }
        SKP_FIX_P_Ana_calc_corr_st3(
            crosscorr_st3.as_mut_ptr(),
            input_signal_ptr as *const libc::c_short,
            start_lag,
            sf_length,
            complexity,
        );
        SKP_FIX_P_Ana_calc_energy_st3(
            energies_st3.as_mut_ptr(),
            input_signal_ptr as *const libc::c_short,
            start_lag,
            sf_length,
            complexity,
        );
        lag_counter = 0 as libc::c_int;
        contour_bias = 52429 as libc::c_int / lag;
        cbk_size = SKP_Silk_cbk_sizes_stage3[complexity as usize] as libc::c_int;
        cbk_offset = SKP_Silk_cbk_offsets_stage3[complexity as usize] as libc::c_int;
        d = start_lag;
        while d <= end_lag {
            j = cbk_offset;
            while j < cbk_offset + cbk_size {
                cross_corr = 0 as libc::c_int;
                energy = 0 as libc::c_int;
                k = 0 as libc::c_int;
                while k < 4 as libc::c_int {
                    energy
                        += energies_st3[k as usize][j as usize][lag_counter as usize]
                            >> 2 as libc::c_int;
                    cross_corr
                        += crosscorr_st3[k as usize][j as usize][lag_counter as usize]
                            >> 2 as libc::c_int;
                    k += 1;
                    k;
                }
                if cross_corr > 0 as libc::c_int {
                    lz = SKP_Silk_CLZ32(cross_corr);
                    lshift = if 0 as libc::c_int > 13 as libc::c_int {
                        if lz - 1 as libc::c_int > 0 as libc::c_int {
                            0 as libc::c_int
                        } else if (lz - 1 as libc::c_int) < 13 as libc::c_int {
                            13 as libc::c_int
                        } else {
                            lz - 1 as libc::c_int
                        }
                    } else if lz - 1 as libc::c_int > 13 as libc::c_int {
                        13 as libc::c_int
                    } else if (lz - 1 as libc::c_int) < 0 as libc::c_int {
                        0 as libc::c_int
                    } else {
                        lz - 1 as libc::c_int
                    };
                    CCmax_new = (cross_corr << lshift)
                        / ((energy >> 13 as libc::c_int - lshift) + 1 as libc::c_int);
                    CCmax_new = if CCmax_new > 0x7fff as libc::c_int {
                        0x7fff as libc::c_int
                    } else if CCmax_new
                        < 0x8000 as libc::c_int as libc::c_short as libc::c_int
                    {
                        0x8000 as libc::c_int as libc::c_short as libc::c_int
                    } else {
                        CCmax_new
                    };
                    CCmax_new = (cross_corr >> 16 as libc::c_int)
                        * CCmax_new as libc::c_short as libc::c_int
                        + ((cross_corr & 0xffff as libc::c_int)
                            * CCmax_new as libc::c_short as libc::c_int
                            >> 16 as libc::c_int);
                    if CCmax_new > 0x7fffffff as libc::c_int >> 3 as libc::c_int {
                        CCmax_new = 0x7fffffff as libc::c_int;
                    } else {
                        CCmax_new = CCmax_new << 3 as libc::c_int;
                    }
                    diff = j - (34 as libc::c_int >> 1 as libc::c_int);
                    diff = diff * diff;
                    diff = 0x7fff as libc::c_int
                        - (contour_bias * diff >> 5 as libc::c_int);
                    CCmax_new = (CCmax_new >> 16 as libc::c_int)
                        * diff as libc::c_short as libc::c_int
                        + ((CCmax_new & 0xffff as libc::c_int)
                            * diff as libc::c_short as libc::c_int >> 16 as libc::c_int)
                        << 1 as libc::c_int;
                } else {
                    CCmax_new = 0 as libc::c_int;
                }
                if CCmax_new > CCmax
                    && d
                        + SKP_Silk_CB_lags_stage3[0 as libc::c_int as usize][j as usize]
                            as libc::c_int <= max_lag
                {
                    CCmax = CCmax_new;
                    lag_new = d;
                    CBimax = j;
                }
                j += 1;
                j;
            }
            lag_counter += 1;
            lag_counter;
            d += 1;
            d;
        }
        k = 0 as libc::c_int;
        while k < 4 as libc::c_int {
            *pitch_out
                .offset(
                    k as isize,
                ) = lag_new
                + SKP_Silk_CB_lags_stage3[k as usize][CBimax as usize] as libc::c_int;
            k += 1;
            k;
        }
        *lagIndex = lag_new - min_lag;
        *contourIndex = CBimax;
    } else {
        CCmax = if CCmax > 0 as libc::c_int { CCmax } else { 0 as libc::c_int };
        *LTPCorr_Q15 = SKP_Silk_SQRT_APPROX(CCmax << 13 as libc::c_int);
        k = 0 as libc::c_int;
        while k < 4 as libc::c_int {
            *pitch_out
                .offset(
                    k as isize,
                ) = lag
                + SKP_Silk_CB_lags_stage2[k as usize][CBimax as usize] as libc::c_int;
            k += 1;
            k;
        }
        *lagIndex = lag - min_lag_8kHz;
        *contourIndex = CBimax;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn SKP_FIX_P_Ana_calc_corr_st3(
    mut cross_corr_st3: *mut [[libc::c_int; 5]; 34],
    mut signal: *const libc::c_short,
    mut start_lag: libc::c_int,
    mut sf_length: libc::c_int,
    mut complexity: libc::c_int,
) {
    let mut target_ptr: *const libc::c_short = 0 as *const libc::c_short;
    let mut basis_ptr: *const libc::c_short = 0 as *const libc::c_short;
    let mut cross_corr: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut lag_counter: libc::c_int = 0;
    let mut cbk_offset: libc::c_int = 0;
    let mut cbk_size: libc::c_int = 0;
    let mut delta: libc::c_int = 0;
    let mut idx: libc::c_int = 0;
    let mut scratch_mem: [libc::c_int; 22] = [0; 22];
    cbk_offset = SKP_Silk_cbk_offsets_stage3[complexity as usize] as libc::c_int;
    cbk_size = SKP_Silk_cbk_sizes_stage3[complexity as usize] as libc::c_int;
    target_ptr = &*signal.offset((sf_length << 2 as libc::c_int) as isize)
        as *const libc::c_short;
    k = 0 as libc::c_int;
    while k < 4 as libc::c_int {
        lag_counter = 0 as libc::c_int;
        j = SKP_Silk_Lag_range_stage3[complexity
            as usize][k as usize][0 as libc::c_int as usize] as libc::c_int;
        while j
            <= SKP_Silk_Lag_range_stage3[complexity
                as usize][k as usize][1 as libc::c_int as usize] as libc::c_int
        {
            basis_ptr = target_ptr.offset(-((start_lag + j) as isize));
            cross_corr = SKP_Silk_inner_prod_aligned(
                target_ptr as *mut libc::c_short,
                basis_ptr as *mut libc::c_short,
                sf_length,
            );
            scratch_mem[lag_counter as usize] = cross_corr;
            lag_counter += 1;
            lag_counter;
            j += 1;
            j;
        }
        delta = SKP_Silk_Lag_range_stage3[complexity
            as usize][k as usize][0 as libc::c_int as usize] as libc::c_int;
        i = cbk_offset;
        while i < cbk_offset + cbk_size {
            idx = SKP_Silk_CB_lags_stage3[k as usize][i as usize] as libc::c_int - delta;
            j = 0 as libc::c_int;
            while j < 5 as libc::c_int {
                (*cross_corr_st3
                    .offset(
                        k as isize,
                    ))[i as usize][j as usize] = scratch_mem[(idx + j) as usize];
                j += 1;
                j;
            }
            i += 1;
            i;
        }
        target_ptr = target_ptr.offset(sf_length as isize);
        k += 1;
        k;
    }
}
#[no_mangle]
pub unsafe extern "C" fn SKP_FIX_P_Ana_calc_energy_st3(
    mut energies_st3: *mut [[libc::c_int; 5]; 34],
    mut signal: *const libc::c_short,
    mut start_lag: libc::c_int,
    mut sf_length: libc::c_int,
    mut complexity: libc::c_int,
) {
    let mut target_ptr: *const libc::c_short = 0 as *const libc::c_short;
    let mut basis_ptr: *const libc::c_short = 0 as *const libc::c_short;
    let mut energy: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut lag_counter: libc::c_int = 0;
    let mut cbk_offset: libc::c_int = 0;
    let mut cbk_size: libc::c_int = 0;
    let mut delta: libc::c_int = 0;
    let mut idx: libc::c_int = 0;
    let mut scratch_mem: [libc::c_int; 22] = [0; 22];
    cbk_offset = SKP_Silk_cbk_offsets_stage3[complexity as usize] as libc::c_int;
    cbk_size = SKP_Silk_cbk_sizes_stage3[complexity as usize] as libc::c_int;
    target_ptr = &*signal.offset((sf_length << 2 as libc::c_int) as isize)
        as *const libc::c_short;
    k = 0 as libc::c_int;
    while k < 4 as libc::c_int {
        lag_counter = 0 as libc::c_int;
        basis_ptr = target_ptr
            .offset(
                -((start_lag
                    + SKP_Silk_Lag_range_stage3[complexity
                        as usize][k as usize][0 as libc::c_int as usize] as libc::c_int)
                    as isize),
            );
        energy = SKP_Silk_inner_prod_aligned(basis_ptr, basis_ptr, sf_length);
        scratch_mem[lag_counter as usize] = energy;
        lag_counter += 1;
        lag_counter;
        i = 1 as libc::c_int;
        while i
            < SKP_Silk_Lag_range_stage3[complexity
                as usize][k as usize][1 as libc::c_int as usize] as libc::c_int
                - SKP_Silk_Lag_range_stage3[complexity
                    as usize][k as usize][0 as libc::c_int as usize] as libc::c_int
                + 1 as libc::c_int
        {
            energy
                -= *basis_ptr.offset((sf_length - i) as isize) as libc::c_int
                    * *basis_ptr.offset((sf_length - i) as isize) as libc::c_int;
            energy = if (energy
                + *basis_ptr.offset(-i as isize) as libc::c_int
                    * *basis_ptr.offset(-i as isize) as libc::c_int) as libc::c_uint
                & 0x80000000 as libc::c_uint == 0 as libc::c_int as libc::c_uint
            {
                if (energy
                    & *basis_ptr.offset(-i as isize) as libc::c_int
                        * *basis_ptr.offset(-i as isize) as libc::c_int) as libc::c_uint
                    & 0x80000000 as libc::c_uint != 0 as libc::c_int as libc::c_uint
                {
                    0x80000000 as libc::c_uint as libc::c_int
                } else {
                    energy
                        + *basis_ptr.offset(-i as isize) as libc::c_int
                            * *basis_ptr.offset(-i as isize) as libc::c_int
                }
            } else if (energy
                | *basis_ptr.offset(-i as isize) as libc::c_int
                    * *basis_ptr.offset(-i as isize) as libc::c_int) as libc::c_uint
                & 0x80000000 as libc::c_uint == 0 as libc::c_int as libc::c_uint
            {
                0x7fffffff as libc::c_int
            } else {
                energy
                    + *basis_ptr.offset(-i as isize) as libc::c_int
                        * *basis_ptr.offset(-i as isize) as libc::c_int
            };
            scratch_mem[lag_counter as usize] = energy;
            lag_counter += 1;
            lag_counter;
            i += 1;
            i;
        }
        delta = SKP_Silk_Lag_range_stage3[complexity
            as usize][k as usize][0 as libc::c_int as usize] as libc::c_int;
        i = cbk_offset;
        while i < cbk_offset + cbk_size {
            idx = SKP_Silk_CB_lags_stage3[k as usize][i as usize] as libc::c_int - delta;
            j = 0 as libc::c_int;
            while j < 5 as libc::c_int {
                (*energies_st3
                    .offset(
                        k as isize,
                    ))[i as usize][j as usize] = scratch_mem[(idx + j) as usize];
                j += 1;
                j;
            }
            i += 1;
            i;
        }
        target_ptr = target_ptr.offset(sf_length as isize);
        k += 1;
        k;
    }
}
#[no_mangle]
pub unsafe extern "C" fn SKP_FIX_P_Ana_find_scaling(
    mut signal: *const libc::c_short,
    signal_length: libc::c_int,
    sum_sqr_len: libc::c_int,
) -> libc::c_int {
    let mut nbits: libc::c_int = 0;
    let mut x_max: libc::c_int = 0;
    x_max = SKP_Silk_int16_array_maxabs(signal, signal_length) as libc::c_int;
    if x_max < 0x7fff as libc::c_int {
        nbits = 32 as libc::c_int
            - SKP_Silk_CLZ32(
                x_max as libc::c_short as libc::c_int
                    * x_max as libc::c_short as libc::c_int,
            );
    } else {
        nbits = 30 as libc::c_int;
    }
    nbits += 17 as libc::c_int - SKP_Silk_CLZ16(sum_sqr_len as libc::c_short);
    if nbits < 31 as libc::c_int {
        return 0 as libc::c_int
    } else {
        return nbits - 30 as libc::c_int
    };
}
