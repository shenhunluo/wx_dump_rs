#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn SKP_Silk_ana_filt_bank_1(
        in_0: *const libc::c_short,
        S: *mut libc::c_int,
        outL: *mut libc::c_short,
        outH: *mut libc::c_short,
        scratch: *mut libc::c_int,
        N: libc::c_int,
    );
    fn SKP_Silk_lin2log(inLin: libc::c_int) -> libc::c_int;
    fn SKP_Silk_sigm_Q15(in_Q5: libc::c_int) -> libc::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SKP_Silk_VAD_state {
    pub AnaState: [libc::c_int; 2],
    pub AnaState1: [libc::c_int; 2],
    pub AnaState2: [libc::c_int; 2],
    pub XnrgSubfr: [libc::c_int; 4],
    pub NrgRatioSmth_Q8: [libc::c_int; 4],
    pub HPstate: libc::c_short,
    pub NL: [libc::c_int; 4],
    pub inv_NL: [libc::c_int; 4],
    pub NoiseLevelBias: [libc::c_int; 4],
    pub counter: libc::c_int,
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
unsafe extern "C" fn SKP_min_int(mut a: libc::c_int, mut b: libc::c_int) -> libc::c_int {
    return if a < b { a } else { b };
}
#[inline]
unsafe extern "C" fn SKP_max_int(mut a: libc::c_int, mut b: libc::c_int) -> libc::c_int {
    return if a > b { a } else { b };
}
#[inline]
unsafe extern "C" fn SKP_max_32(mut a: libc::c_int, mut b: libc::c_int) -> libc::c_int {
    return if a > b { a } else { b };
}
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_VAD_Init(
    mut psSilk_VAD: *mut SKP_Silk_VAD_state,
) -> libc::c_int {
    let mut b: libc::c_int = 0;
    let mut ret: libc::c_int = 0 as libc::c_int;
    memset(
        psSilk_VAD as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<SKP_Silk_VAD_state>() as libc::c_ulong,
    );
    b = 0 as libc::c_int;
    while b < 4 as libc::c_int {
        (*psSilk_VAD)
            .NoiseLevelBias[b
            as usize] = SKP_max_32(
            50 as libc::c_int / (b + 1 as libc::c_int),
            1 as libc::c_int,
        );
        b += 1;
        b;
    }
    b = 0 as libc::c_int;
    while b < 4 as libc::c_int {
        (*psSilk_VAD)
            .NL[b
            as usize] = 100 as libc::c_int * (*psSilk_VAD).NoiseLevelBias[b as usize];
        (*psSilk_VAD)
            .inv_NL[b
            as usize] = 0x7fffffff as libc::c_int / (*psSilk_VAD).NL[b as usize];
        b += 1;
        b;
    }
    (*psSilk_VAD).counter = 15 as libc::c_int;
    b = 0 as libc::c_int;
    while b < 4 as libc::c_int {
        (*psSilk_VAD)
            .NrgRatioSmth_Q8[b as usize] = 100 as libc::c_int * 256 as libc::c_int;
        b += 1;
        b;
    }
    return ret;
}
static mut tiltWeights: [libc::c_int; 4] = [
    30000 as libc::c_int,
    6000 as libc::c_int,
    -(12000 as libc::c_int),
    -(12000 as libc::c_int),
];
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_VAD_GetSA_Q8(
    mut psSilk_VAD: *mut SKP_Silk_VAD_state,
    mut pSA_Q8: *mut libc::c_int,
    mut pSNR_dB_Q7: *mut libc::c_int,
    mut pQuality_Q15: *mut libc::c_int,
    mut pTilt_Q15: *mut libc::c_int,
    mut pIn: *const libc::c_short,
    framelength: libc::c_int,
) -> libc::c_int {
    let mut SA_Q15: libc::c_int = 0;
    let mut input_tilt: libc::c_int = 0;
    let mut scratch: [libc::c_int; 720] = [0; 720];
    let mut decimated_framelength: libc::c_int = 0;
    let mut dec_subframe_length: libc::c_int = 0;
    let mut dec_subframe_offset: libc::c_int = 0;
    let mut SNR_Q7: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    let mut s: libc::c_int = 0;
    let mut sumSquared: libc::c_int = 0;
    let mut smooth_coef_Q16: libc::c_int = 0;
    let mut HPstateTmp: libc::c_short = 0;
    let mut X: [[libc::c_short; 240]; 4] = [[0; 240]; 4];
    let mut Xnrg: [libc::c_int; 4] = [0; 4];
    let mut NrgToNoiseRatio_Q8: [libc::c_int; 4] = [0; 4];
    let mut speech_nrg: libc::c_int = 0;
    let mut x_tmp: libc::c_int = 0;
    let mut ret: libc::c_int = 0 as libc::c_int;
    SKP_Silk_ana_filt_bank_1(
        pIn,
        &mut *((*psSilk_VAD).AnaState).as_mut_ptr().offset(0 as libc::c_int as isize),
        &mut *(*X.as_mut_ptr().offset(0 as libc::c_int as isize))
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize),
        &mut *(*X.as_mut_ptr().offset(3 as libc::c_int as isize))
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize),
        &mut *scratch.as_mut_ptr().offset(0 as libc::c_int as isize),
        framelength,
    );
    SKP_Silk_ana_filt_bank_1(
        &mut *(*X.as_mut_ptr().offset(0 as libc::c_int as isize))
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize),
        &mut *((*psSilk_VAD).AnaState1).as_mut_ptr().offset(0 as libc::c_int as isize),
        &mut *(*X.as_mut_ptr().offset(0 as libc::c_int as isize))
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize),
        &mut *(*X.as_mut_ptr().offset(2 as libc::c_int as isize))
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize),
        &mut *scratch.as_mut_ptr().offset(0 as libc::c_int as isize),
        framelength >> 1 as libc::c_int,
    );
    SKP_Silk_ana_filt_bank_1(
        &mut *(*X.as_mut_ptr().offset(0 as libc::c_int as isize))
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize),
        &mut *((*psSilk_VAD).AnaState2).as_mut_ptr().offset(0 as libc::c_int as isize),
        &mut *(*X.as_mut_ptr().offset(0 as libc::c_int as isize))
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize),
        &mut *(*X.as_mut_ptr().offset(1 as libc::c_int as isize))
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize),
        &mut *scratch.as_mut_ptr().offset(0 as libc::c_int as isize),
        framelength >> 2 as libc::c_int,
    );
    decimated_framelength = framelength >> 3 as libc::c_int;
    X[0 as libc::c_int
        as usize][(decimated_framelength - 1 as libc::c_int)
        as usize] = (X[0 as libc::c_int
        as usize][(decimated_framelength - 1 as libc::c_int) as usize] as libc::c_int
        >> 1 as libc::c_int) as libc::c_short;
    HPstateTmp = X[0 as libc::c_int
        as usize][(decimated_framelength - 1 as libc::c_int) as usize];
    i = decimated_framelength - 1 as libc::c_int;
    while i > 0 as libc::c_int {
        X[0 as libc::c_int
            as usize][(i - 1 as libc::c_int)
            as usize] = (X[0 as libc::c_int as usize][(i - 1 as libc::c_int) as usize]
            as libc::c_int >> 1 as libc::c_int) as libc::c_short;
        X[0 as libc::c_int
            as usize][i
            as usize] = (X[0 as libc::c_int as usize][i as usize] as libc::c_int
            - X[0 as libc::c_int as usize][(i - 1 as libc::c_int) as usize]
                as libc::c_int) as libc::c_short;
        i -= 1;
        i;
    }
    X[0 as libc::c_int
        as usize][0 as libc::c_int
        as usize] = (X[0 as libc::c_int as usize][0 as libc::c_int as usize]
        as libc::c_int - (*psSilk_VAD).HPstate as libc::c_int) as libc::c_short;
    (*psSilk_VAD).HPstate = HPstateTmp;
    b = 0 as libc::c_int;
    while b < 4 as libc::c_int {
        decimated_framelength = framelength
            >> SKP_min_int(4 as libc::c_int - b, 4 as libc::c_int - 1 as libc::c_int);
        dec_subframe_length = decimated_framelength >> 2 as libc::c_int;
        dec_subframe_offset = 0 as libc::c_int;
        Xnrg[b as usize] = (*psSilk_VAD).XnrgSubfr[b as usize];
        s = 0 as libc::c_int;
        while s < (1 as libc::c_int) << 2 as libc::c_int {
            sumSquared = 0 as libc::c_int;
            i = 0 as libc::c_int;
            while i < dec_subframe_length {
                x_tmp = X[b as usize][(i + dec_subframe_offset) as usize] as libc::c_int
                    >> 3 as libc::c_int;
                sumSquared = sumSquared
                    + x_tmp as libc::c_short as libc::c_int
                        * x_tmp as libc::c_short as libc::c_int;
                i += 1;
                i;
            }
            if s < ((1 as libc::c_int) << 2 as libc::c_int) - 1 as libc::c_int {
                Xnrg[b
                    as usize] = if (Xnrg[b as usize] + sumSquared) as libc::c_uint
                    & 0x80000000 as libc::c_uint != 0
                {
                    0x7fffffff as libc::c_int
                } else {
                    Xnrg[b as usize] + sumSquared
                };
            } else {
                Xnrg[b
                    as usize] = if (Xnrg[b as usize] + (sumSquared >> 1 as libc::c_int))
                    as libc::c_uint & 0x80000000 as libc::c_uint != 0
                {
                    0x7fffffff as libc::c_int
                } else {
                    Xnrg[b as usize] + (sumSquared >> 1 as libc::c_int)
                };
            }
            dec_subframe_offset += dec_subframe_length;
            s += 1;
            s;
        }
        (*psSilk_VAD).XnrgSubfr[b as usize] = sumSquared;
        b += 1;
        b;
    }
    SKP_Silk_VAD_GetNoiseLevels(
        &mut *Xnrg.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut libc::c_int
            as *const libc::c_int,
        psSilk_VAD,
    );
    sumSquared = 0 as libc::c_int;
    input_tilt = 0 as libc::c_int;
    b = 0 as libc::c_int;
    while b < 4 as libc::c_int {
        speech_nrg = Xnrg[b as usize] - (*psSilk_VAD).NL[b as usize];
        if speech_nrg > 0 as libc::c_int {
            if Xnrg[b as usize] as libc::c_uint & 0xff800000 as libc::c_uint
                == 0 as libc::c_int as libc::c_uint
            {
                NrgToNoiseRatio_Q8[b
                    as usize] = (Xnrg[b as usize] << 8 as libc::c_int)
                    / ((*psSilk_VAD).NL[b as usize] + 1 as libc::c_int);
            } else {
                NrgToNoiseRatio_Q8[b
                    as usize] = Xnrg[b as usize]
                    / (((*psSilk_VAD).NL[b as usize] >> 8 as libc::c_int)
                        + 1 as libc::c_int);
            }
            SNR_Q7 = SKP_Silk_lin2log(NrgToNoiseRatio_Q8[b as usize])
                - 8 as libc::c_int * 128 as libc::c_int;
            sumSquared = sumSquared
                + SNR_Q7 as libc::c_short as libc::c_int
                    * SNR_Q7 as libc::c_short as libc::c_int;
            if speech_nrg < (1 as libc::c_int) << 20 as libc::c_int {
                SNR_Q7 = (SKP_Silk_SQRT_APPROX(speech_nrg) << 6 as libc::c_int
                    >> 16 as libc::c_int) * SNR_Q7 as libc::c_short as libc::c_int
                    + ((SKP_Silk_SQRT_APPROX(speech_nrg) << 6 as libc::c_int
                        & 0xffff as libc::c_int) * SNR_Q7 as libc::c_short as libc::c_int
                        >> 16 as libc::c_int);
            }
            input_tilt = input_tilt
                + ((tiltWeights[b as usize] >> 16 as libc::c_int)
                    * SNR_Q7 as libc::c_short as libc::c_int
                    + ((tiltWeights[b as usize] & 0xffff as libc::c_int)
                        * SNR_Q7 as libc::c_short as libc::c_int >> 16 as libc::c_int));
        } else {
            NrgToNoiseRatio_Q8[b as usize] = 256 as libc::c_int;
        }
        b += 1;
        b;
    }
    sumSquared = sumSquared / 4 as libc::c_int;
    *pSNR_dB_Q7 = (3 as libc::c_int * SKP_Silk_SQRT_APPROX(sumSquared)) as libc::c_short
        as libc::c_int;
    SA_Q15 = SKP_Silk_sigm_Q15(
        (45000 as libc::c_int >> 16 as libc::c_int)
            * *pSNR_dB_Q7 as libc::c_short as libc::c_int
            + ((45000 as libc::c_int & 0xffff as libc::c_int)
                * *pSNR_dB_Q7 as libc::c_short as libc::c_int >> 16 as libc::c_int)
            - 128 as libc::c_int,
    );
    *pTilt_Q15 = (SKP_Silk_sigm_Q15(input_tilt) - 16384 as libc::c_int)
        << 1 as libc::c_int;
    speech_nrg = 0 as libc::c_int;
    b = 0 as libc::c_int;
    while b < 4 as libc::c_int {
        speech_nrg
            += (b + 1 as libc::c_int)
                * (Xnrg[b as usize] - (*psSilk_VAD).NL[b as usize] >> 4 as libc::c_int);
        b += 1;
        b;
    }
    if speech_nrg <= 0 as libc::c_int {
        SA_Q15 = SA_Q15 >> 1 as libc::c_int;
    } else if speech_nrg < 32768 as libc::c_int {
        speech_nrg = SKP_Silk_SQRT_APPROX(speech_nrg << 15 as libc::c_int);
        SA_Q15 = (32768 as libc::c_int + speech_nrg >> 16 as libc::c_int)
            * SA_Q15 as libc::c_short as libc::c_int
            + ((32768 as libc::c_int + speech_nrg & 0xffff as libc::c_int)
                * SA_Q15 as libc::c_short as libc::c_int >> 16 as libc::c_int);
    }
    *pSA_Q8 = SKP_min_int(SA_Q15 >> 7 as libc::c_int, 0xff as libc::c_int);
    smooth_coef_Q16 = (4096 as libc::c_int >> 16 as libc::c_int)
        * ((SA_Q15 >> 16 as libc::c_int) * SA_Q15 as libc::c_short as libc::c_int
            + ((SA_Q15 & 0xffff as libc::c_int) * SA_Q15 as libc::c_short as libc::c_int
                >> 16 as libc::c_int)) as libc::c_short as libc::c_int
        + ((4096 as libc::c_int & 0xffff as libc::c_int)
            * ((SA_Q15 >> 16 as libc::c_int) * SA_Q15 as libc::c_short as libc::c_int
                + ((SA_Q15 & 0xffff as libc::c_int)
                    * SA_Q15 as libc::c_short as libc::c_int >> 16 as libc::c_int))
                as libc::c_short as libc::c_int >> 16 as libc::c_int);
    b = 0 as libc::c_int;
    while b < 4 as libc::c_int {
        (*psSilk_VAD)
            .NrgRatioSmth_Q8[b
            as usize] = (*psSilk_VAD).NrgRatioSmth_Q8[b as usize]
            + ((NrgToNoiseRatio_Q8[b as usize]
                - (*psSilk_VAD).NrgRatioSmth_Q8[b as usize] >> 16 as libc::c_int)
                * smooth_coef_Q16 as libc::c_short as libc::c_int
                + ((NrgToNoiseRatio_Q8[b as usize]
                    - (*psSilk_VAD).NrgRatioSmth_Q8[b as usize] & 0xffff as libc::c_int)
                    * smooth_coef_Q16 as libc::c_short as libc::c_int
                    >> 16 as libc::c_int));
        SNR_Q7 = 3 as libc::c_int
            * (SKP_Silk_lin2log((*psSilk_VAD).NrgRatioSmth_Q8[b as usize])
                - 8 as libc::c_int * 128 as libc::c_int);
        *pQuality_Q15
            .offset(
                b as isize,
            ) = SKP_Silk_sigm_Q15(
            SNR_Q7 - 16 as libc::c_int * 128 as libc::c_int >> 4 as libc::c_int,
        );
        b += 1;
        b;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_VAD_GetNoiseLevels(
    mut pX: *const libc::c_int,
    mut psSilk_VAD: *mut SKP_Silk_VAD_state,
) {
    let mut k: libc::c_int = 0;
    let mut nl: libc::c_int = 0;
    let mut nrg: libc::c_int = 0;
    let mut inv_nrg: libc::c_int = 0;
    let mut coef: libc::c_int = 0;
    let mut min_coef: libc::c_int = 0;
    if (*psSilk_VAD).counter < 1000 as libc::c_int {
        min_coef = 0x7fff as libc::c_int
            / (((*psSilk_VAD).counter >> 4 as libc::c_int) + 1 as libc::c_int);
    } else {
        min_coef = 0 as libc::c_int;
    }
    k = 0 as libc::c_int;
    while k < 4 as libc::c_int {
        nl = (*psSilk_VAD).NL[k as usize];
        nrg = if (*pX.offset(k as isize) + (*psSilk_VAD).NoiseLevelBias[k as usize])
            as libc::c_uint & 0x80000000 as libc::c_uint != 0
        {
            0x7fffffff as libc::c_int
        } else {
            *pX.offset(k as isize) + (*psSilk_VAD).NoiseLevelBias[k as usize]
        };
        inv_nrg = 0x7fffffff as libc::c_int / nrg;
        if nrg > nl << 3 as libc::c_int {
            coef = 1024 as libc::c_int >> 3 as libc::c_int;
        } else if nrg < nl {
            coef = 1024 as libc::c_int;
        } else {
            coef = ((inv_nrg >> 16 as libc::c_int) * nl as libc::c_short as libc::c_int
                + ((inv_nrg & 0xffff as libc::c_int) * nl as libc::c_short as libc::c_int
                    >> 16 as libc::c_int)
                + inv_nrg
                    * (if 16 as libc::c_int == 1 as libc::c_int {
                        (nl >> 1 as libc::c_int) + (nl & 1 as libc::c_int)
                    } else {
                        (nl >> 16 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                            >> 1 as libc::c_int
                    }) >> 16 as libc::c_int)
                * ((1024 as libc::c_int) << 1 as libc::c_int) as libc::c_short
                    as libc::c_int
                + (((inv_nrg >> 16 as libc::c_int) * nl as libc::c_short as libc::c_int
                    + ((inv_nrg & 0xffff as libc::c_int)
                        * nl as libc::c_short as libc::c_int >> 16 as libc::c_int)
                    + inv_nrg
                        * (if 16 as libc::c_int == 1 as libc::c_int {
                            (nl >> 1 as libc::c_int) + (nl & 1 as libc::c_int)
                        } else {
                            (nl >> 16 as libc::c_int - 1 as libc::c_int)
                                + 1 as libc::c_int >> 1 as libc::c_int
                        }) & 0xffff as libc::c_int)
                    * ((1024 as libc::c_int) << 1 as libc::c_int) as libc::c_short
                        as libc::c_int >> 16 as libc::c_int);
        }
        coef = SKP_max_int(coef, min_coef);
        (*psSilk_VAD)
            .inv_NL[k
            as usize] = (*psSilk_VAD).inv_NL[k as usize]
            + ((inv_nrg - (*psSilk_VAD).inv_NL[k as usize] >> 16 as libc::c_int)
                * coef as libc::c_short as libc::c_int
                + ((inv_nrg - (*psSilk_VAD).inv_NL[k as usize] & 0xffff as libc::c_int)
                    * coef as libc::c_short as libc::c_int >> 16 as libc::c_int));
        nl = 0x7fffffff as libc::c_int / (*psSilk_VAD).inv_NL[k as usize];
        nl = if nl < 0xffffff as libc::c_int { nl } else { 0xffffff as libc::c_int };
        (*psSilk_VAD).NL[k as usize] = nl;
        k += 1;
        k;
    }
    (*psSilk_VAD).counter += 1;
    (*psSilk_VAD).counter;
}
