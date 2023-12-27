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
    fn SKP_Silk_insertion_sort_increasing(
        a: *mut libc::c_int,
        index: *mut libc::c_int,
        L: libc::c_int,
        K: libc::c_int,
    );
    fn SKP_Silk_NLSF_MSVQ_decode(
        pNLSF_Q15: *mut libc::c_int,
        psNLSF_CB: *const SkpSilkNlsfCbStruct,
        NLSFIndices: *const libc::c_int,
        LPC_order: libc::c_int,
    );
    fn SKP_Silk_NLSF_VQ_rate_distortion_FIX(
        pRD_Q20: *mut libc::c_int,
        psNLSF_CBS: *const SKP_Silk_NLSF_CBS,
        in_Q15: *const libc::c_int,
        w_Q6: *const libc::c_int,
        rate_acc_Q5: *const libc::c_int,
        mu_Q15: libc::c_int,
        N: libc::c_int,
        LPC_order: libc::c_int,
    );
}
pub type __int64_t = libc::c_longlong;
pub type int64_t = __int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SKP_Silk_NLSF_CBS {
    pub nVectors: libc::c_int,
    pub CB_NLSF_Q15: *const libc::c_short,
    pub Rates_Q5: *const libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SkpSilkNlsfCbStruct {
    pub nStages: libc::c_int,
    pub CBStages: *const SKP_Silk_NLSF_CBS,
    pub NDeltaMin_Q15: *const libc::c_int,
    pub CDF: *const libc::c_ushort,
    pub StartPtr: *const *const libc::c_ushort,
    pub MiddleIx: *const libc::c_int,
}
#[inline]
unsafe extern "C" fn SKP_min_32(mut a: libc::c_int, mut b: libc::c_int) -> libc::c_int {
    return if a < b { a } else { b };
}
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_NLSF_MSVQ_encode_FIX(
    mut NLSFIndices: *mut libc::c_int,
    mut pNLSF_Q15: *mut libc::c_int,
    mut psNLSF_CB: *const SkpSilkNlsfCbStruct,
    mut pNLSF_q_Q15_prev: *const libc::c_int,
    mut pW_Q6: *const libc::c_int,
    NLSF_mu_Q15: libc::c_int,
    NLSF_mu_fluc_red_Q16: libc::c_int,
    NLSF_MSVQ_Survivors: libc::c_int,
    LPC_order: libc::c_int,
    deactivate_fluc_red: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut s: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut cur_survivors: libc::c_int = 0 as libc::c_int;
    let mut prev_survivors: libc::c_int = 0;
    let mut min_survivors: libc::c_int = 0;
    let mut input_index: libc::c_int = 0;
    let mut cb_index: libc::c_int = 0;
    let mut bestIndex: libc::c_int = 0;
    let mut rateDistThreshold_Q18: libc::c_int = 0;
    let mut se_Q15: libc::c_int = 0;
    let mut wsse_Q20: libc::c_int = 0;
    let mut bestRateDist_Q20: libc::c_int = 0;
    let mut pRateDist_Q18: [libc::c_int; 256] = [0; 256];
    let mut pRate_Q5: [libc::c_int; 16] = [0; 16];
    let mut pRate_new_Q5: [libc::c_int; 16] = [0; 16];
    let mut pTempIndices: [libc::c_int; 16] = [0; 16];
    let mut pPath: [libc::c_int; 160] = [0; 160];
    let mut pPath_new: [libc::c_int; 160] = [0; 160];
    let mut pRes_Q15: [libc::c_int; 256] = [0; 256];
    let mut pRes_new_Q15: [libc::c_int; 256] = [0; 256];
    let mut pConstInt: *const libc::c_int = 0 as *const libc::c_int;
    let mut pInt: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut pCB_element: *const libc::c_short = 0 as *const libc::c_short;
    let mut pCurrentCBStage: *const SKP_Silk_NLSF_CBS = 0 as *const SKP_Silk_NLSF_CBS;
    memset(
        pRate_Q5.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        (NLSF_MSVQ_Survivors as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    i = 0 as libc::c_int;
    while i < LPC_order {
        pRes_Q15[i as usize] = *pNLSF_Q15.offset(i as isize);
        i += 1;
        i;
    }
    prev_survivors = 1 as libc::c_int;
    min_survivors = NLSF_MSVQ_Survivors / 2 as libc::c_int;
    s = 0 as libc::c_int;
    while s < (*psNLSF_CB).nStages {
        pCurrentCBStage = &*((*psNLSF_CB).CBStages).offset(s as isize)
            as *const SKP_Silk_NLSF_CBS;
        cur_survivors = SKP_min_32(
            NLSF_MSVQ_Survivors,
            prev_survivors as libc::c_short as libc::c_int
                * (*pCurrentCBStage).nVectors as libc::c_short as libc::c_int,
        );
        SKP_Silk_NLSF_VQ_rate_distortion_FIX(
            pRateDist_Q18.as_mut_ptr(),
            pCurrentCBStage,
            pRes_Q15.as_mut_ptr(),
            pW_Q6,
            pRate_Q5.as_mut_ptr(),
            NLSF_mu_Q15,
            prev_survivors,
            LPC_order,
        );
        SKP_Silk_insertion_sort_increasing(
            pRateDist_Q18.as_mut_ptr(),
            pTempIndices.as_mut_ptr(),
            prev_survivors * (*pCurrentCBStage).nVectors,
            cur_survivors,
        );
        if pRateDist_Q18[0 as libc::c_int as usize]
            < 0x7fffffff as libc::c_int / 16 as libc::c_int
        {
            rateDistThreshold_Q18 = pRateDist_Q18[0 as libc::c_int as usize]
                + ((NLSF_MSVQ_Survivors * pRateDist_Q18[0 as libc::c_int as usize]
                    >> 16 as libc::c_int)
                    * ((0.1f32
                        * ((1 as libc::c_int as int64_t) << 16 as libc::c_int)
                            as libc::c_float) as libc::c_double + 0.5f64) as libc::c_int
                        as libc::c_short as libc::c_int
                    + ((NLSF_MSVQ_Survivors * pRateDist_Q18[0 as libc::c_int as usize]
                        & 0xffff as libc::c_int)
                        * ((0.1f32
                            * ((1 as libc::c_int as int64_t) << 16 as libc::c_int)
                                as libc::c_float) as libc::c_double + 0.5f64) as libc::c_int
                            as libc::c_short as libc::c_int >> 16 as libc::c_int));
            while pRateDist_Q18[(cur_survivors - 1 as libc::c_int) as usize]
                > rateDistThreshold_Q18 && cur_survivors > min_survivors
            {
                cur_survivors -= 1;
                cur_survivors;
            }
        }
        k = 0 as libc::c_int;
        while k < cur_survivors {
            if s > 0 as libc::c_int {
                if (*pCurrentCBStage).nVectors == 8 as libc::c_int {
                    input_index = pTempIndices[k as usize] >> 3 as libc::c_int;
                    cb_index = pTempIndices[k as usize] & 7 as libc::c_int;
                } else {
                    input_index = pTempIndices[k as usize] / (*pCurrentCBStage).nVectors;
                    cb_index = pTempIndices[k as usize]
                        - input_index as libc::c_short as libc::c_int
                            * (*pCurrentCBStage).nVectors as libc::c_short
                                as libc::c_int;
                }
            } else {
                input_index = 0 as libc::c_int;
                cb_index = pTempIndices[k as usize];
            }
            pConstInt = &mut *pRes_Q15
                .as_mut_ptr()
                .offset(
                    (input_index as libc::c_short as libc::c_int
                        * LPC_order as libc::c_short as libc::c_int) as isize,
                ) as *mut libc::c_int;
            pCB_element = &*((*pCurrentCBStage).CB_NLSF_Q15)
                .offset(
                    (cb_index as libc::c_short as libc::c_int
                        * LPC_order as libc::c_short as libc::c_int) as isize,
                ) as *const libc::c_short;
            pInt = &mut *pRes_new_Q15
                .as_mut_ptr()
                .offset(
                    (k as libc::c_short as libc::c_int
                        * LPC_order as libc::c_short as libc::c_int) as isize,
                ) as *mut libc::c_int;
            i = 0 as libc::c_int;
            while i < LPC_order {
                *pInt
                    .offset(
                        i as isize,
                    ) = *pConstInt.offset(i as isize)
                    - *pCB_element.offset(i as isize) as libc::c_int;
                i += 1;
                i;
            }
            pRate_new_Q5[k
                as usize] = pRate_Q5[input_index as usize]
                + *((*pCurrentCBStage).Rates_Q5).offset(cb_index as isize)
                    as libc::c_int;
            pConstInt = &mut *pPath
                .as_mut_ptr()
                .offset(
                    (input_index as libc::c_short as libc::c_int
                        * (*psNLSF_CB).nStages as libc::c_short as libc::c_int) as isize,
                ) as *mut libc::c_int;
            pInt = &mut *pPath_new
                .as_mut_ptr()
                .offset(
                    (k as libc::c_short as libc::c_int
                        * (*psNLSF_CB).nStages as libc::c_short as libc::c_int) as isize,
                ) as *mut libc::c_int;
            i = 0 as libc::c_int;
            while i < s {
                *pInt.offset(i as isize) = *pConstInt.offset(i as isize);
                i += 1;
                i;
            }
            *pInt.offset(s as isize) = cb_index;
            k += 1;
            k;
        }
        if s < (*psNLSF_CB).nStages - 1 as libc::c_int {
            memcpy(
                pRes_Q15.as_mut_ptr() as *mut libc::c_void,
                pRes_new_Q15.as_mut_ptr() as *const libc::c_void,
                ((cur_survivors as libc::c_short as libc::c_int
                    * LPC_order as libc::c_short as libc::c_int) as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
            );
            memcpy(
                pRate_Q5.as_mut_ptr() as *mut libc::c_void,
                pRate_new_Q5.as_mut_ptr() as *const libc::c_void,
                (cur_survivors as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
            );
            memcpy(
                pPath.as_mut_ptr() as *mut libc::c_void,
                pPath_new.as_mut_ptr() as *const libc::c_void,
                ((cur_survivors as libc::c_short as libc::c_int
                    * (*psNLSF_CB).nStages as libc::c_short as libc::c_int)
                    as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
            );
        }
        prev_survivors = cur_survivors;
        s += 1;
        s;
    }
    bestIndex = 0 as libc::c_int;
    if deactivate_fluc_red != 1 as libc::c_int {
        bestRateDist_Q20 = 0x7fffffff as libc::c_int;
        s = 0 as libc::c_int;
        while s < cur_survivors {
            SKP_Silk_NLSF_MSVQ_decode(
                pNLSF_Q15,
                psNLSF_CB,
                &mut *pPath_new
                    .as_mut_ptr()
                    .offset(
                        (s as libc::c_short as libc::c_int
                            * (*psNLSF_CB).nStages as libc::c_short as libc::c_int)
                            as isize,
                    ),
                LPC_order,
            );
            wsse_Q20 = 0 as libc::c_int;
            i = 0 as libc::c_int;
            while i < LPC_order {
                se_Q15 = *pNLSF_Q15.offset(i as isize)
                    - *pNLSF_q_Q15_prev.offset(i as isize);
                wsse_Q20 = wsse_Q20
                    + ((se_Q15 as libc::c_short as libc::c_int
                        * se_Q15 as libc::c_short as libc::c_int >> 16 as libc::c_int)
                        * *pW_Q6.offset(i as isize) as libc::c_short as libc::c_int
                        + ((se_Q15 as libc::c_short as libc::c_int
                            * se_Q15 as libc::c_short as libc::c_int
                            & 0xffff as libc::c_int)
                            * *pW_Q6.offset(i as isize) as libc::c_short as libc::c_int
                            >> 16 as libc::c_int));
                se_Q15 = *pNLSF_Q15.offset((i + 1 as libc::c_int) as isize)
                    - *pNLSF_q_Q15_prev.offset((i + 1 as libc::c_int) as isize);
                wsse_Q20 = wsse_Q20
                    + ((se_Q15 as libc::c_short as libc::c_int
                        * se_Q15 as libc::c_short as libc::c_int >> 16 as libc::c_int)
                        * *pW_Q6.offset((i + 1 as libc::c_int) as isize) as libc::c_short
                            as libc::c_int
                        + ((se_Q15 as libc::c_short as libc::c_int
                            * se_Q15 as libc::c_short as libc::c_int
                            & 0xffff as libc::c_int)
                            * *pW_Q6.offset((i + 1 as libc::c_int) as isize)
                                as libc::c_short as libc::c_int >> 16 as libc::c_int));
                i += 2 as libc::c_int;
            }
            wsse_Q20 = if (pRateDist_Q18[s as usize]
                + ((wsse_Q20 >> 16 as libc::c_int)
                    * NLSF_mu_fluc_red_Q16 as libc::c_short as libc::c_int
                    + ((wsse_Q20 & 0xffff as libc::c_int)
                        * NLSF_mu_fluc_red_Q16 as libc::c_short as libc::c_int
                        >> 16 as libc::c_int))) as libc::c_uint
                & 0x80000000 as libc::c_uint != 0
            {
                0x7fffffff as libc::c_int
            } else {
                pRateDist_Q18[s as usize]
                    + ((wsse_Q20 >> 16 as libc::c_int)
                        * NLSF_mu_fluc_red_Q16 as libc::c_short as libc::c_int
                        + ((wsse_Q20 & 0xffff as libc::c_int)
                            * NLSF_mu_fluc_red_Q16 as libc::c_short as libc::c_int
                            >> 16 as libc::c_int))
            };
            if wsse_Q20 < bestRateDist_Q20 {
                bestRateDist_Q20 = wsse_Q20;
                bestIndex = s;
            }
            s += 1;
            s;
        }
    }
    memcpy(
        NLSFIndices as *mut libc::c_void,
        &mut *pPath_new
            .as_mut_ptr()
            .offset(
                (bestIndex as libc::c_short as libc::c_int
                    * (*psNLSF_CB).nStages as libc::c_short as libc::c_int) as isize,
            ) as *mut libc::c_int as *const libc::c_void,
        ((*psNLSF_CB).nStages as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    SKP_Silk_NLSF_MSVQ_decode(pNLSF_Q15, psNLSF_CB, NLSFIndices, LPC_order);
}
