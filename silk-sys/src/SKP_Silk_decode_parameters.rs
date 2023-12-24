#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]

use crate::{SKP_Silk_dec_API::{SKP_Silk_decoder_state, SKP_Silk_decoder_control}, SKP_Silk_NLSF_MSVQ_decode::{SKP_Silk_NLSF_MSVQ_decode, SKP_Silk_NLSF_CB_struct}, SKP_Silk_bwexpander::SKP_Silk_bwexpander, SKP_Silk_decode_pitch::SKP_Silk_decode_pitch, SKP_Silk_range_coder::{SKP_Silk_range_decoder, SKP_Silk_range_decoder_multi, SKP_Silk_range_coder_get_length, SKP_Silk_range_coder_check_after_decoding}, SKP_Silk_decoder_set_fs::SKP_Silk_decoder_set_fs, SKP_Silk_gain_quant::SKP_Silk_gains_dequant, SKP_Silk_NLSF2A_stable::SKP_Silk_NLSF2A_stable, SKP_Silk_decode_pulses::SKP_Silk_decode_pulses, SKP_Silk_tables_other::{SKP_Silk_SamplingRates_CDF, SKP_Silk_SamplingRates_offset, SKP_Silk_SamplingRates_table, SKP_Silk_NLSF_interpolation_factor_CDF, SKP_Silk_NLSF_interpolation_factor_offset, SKP_Silk_LTPscale_CDF, SKP_Silk_LTPscale_offset, SKP_Silk_LTPScales_table_Q14, SKP_Silk_Seed_CDF, SKP_Silk_Seed_offset, SKP_Silk_vadflag_CDF, SKP_Silk_vadflag_offset, SKP_Silk_FrameTermination_CDF, SKP_Silk_FrameTermination_offset}, SKP_Silk_tables_type_offset::{SKP_Silk_type_offset_CDF, SKP_Silk_type_offset_CDF_offset, SKP_Silk_type_offset_joint_CDF}, SKP_Silk_tables_gain::{SKP_Silk_gain_CDF, SKP_Silk_gain_CDF_offset, SKP_Silk_delta_gain_CDF, SKP_Silk_delta_gain_CDF_offset}, SKP_Silk_tables_pitch_lag::{SKP_Silk_pitch_lag_NB_CDF, SKP_Silk_pitch_lag_NB_CDF_offset, SKP_Silk_pitch_lag_MB_CDF, SKP_Silk_pitch_lag_MB_CDF_offset, SKP_Silk_pitch_lag_WB_CDF, SKP_Silk_pitch_lag_WB_CDF_offset, SKP_Silk_pitch_lag_SWB_CDF, SKP_Silk_pitch_lag_SWB_CDF_offset, SKP_Silk_pitch_contour_NB_CDF, SKP_Silk_pitch_contour_NB_CDF_offset, SKP_Silk_pitch_contour_CDF, SKP_Silk_pitch_contour_CDF_offset}, SKP_Silk_tables_LTP::{SKP_Silk_LTP_per_index_CDF, SKP_Silk_LTP_per_index_CDF_offset, SKP_Silk_LTP_vq_ptrs_Q14, SKP_Silk_LTP_gain_CDF_ptrs, SKP_Silk_LTP_gain_CDF_offsets}};
use super::SKP_Silk_range_coder::SKP_Silk_range_coder_state;
extern "C" {
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _SKP_Silk_resampler_state_struct {
    pub sIIR: [libc::c_int; 6],
    pub sFIR: [libc::c_int; 16],
    pub sDown2: [libc::c_int; 2],
    pub resampler_function: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut libc::c_short,
            *const libc::c_short,
            libc::c_int,
        ) -> (),
    >,
    pub up2_function: Option::<
        unsafe extern "C" fn(
            *mut libc::c_int,
            *mut libc::c_short,
            *const libc::c_short,
            libc::c_int,
        ) -> (),
    >,
    pub batchSize: libc::c_int,
    pub invRatio_Q16: libc::c_int,
    pub FIR_Fracs: libc::c_int,
    pub input2x: libc::c_int,
    pub Coefs: *const libc::c_short,
    pub sDownPre: [libc::c_int; 2],
    pub sUpPost: [libc::c_int; 2],
    pub down_pre_function: Option::<
        unsafe extern "C" fn(
            *mut libc::c_int,
            *mut libc::c_short,
            *const libc::c_short,
            libc::c_int,
        ) -> (),
    >,
    pub up_post_function: Option::<
        unsafe extern "C" fn(
            *mut libc::c_int,
            *mut libc::c_short,
            *const libc::c_short,
            libc::c_int,
        ) -> (),
    >,
    pub batchSizePrePost: libc::c_int,
    pub ratio_Q16: libc::c_int,
    pub nPreDownsamplers: libc::c_int,
    pub nPostUpsamplers: libc::c_int,
    pub magic_number: libc::c_int,
}
pub type SKP_Silk_resampler_state_struct = _SKP_Silk_resampler_state_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SKP_Silk_NLSF_CBS {
    pub nVectors: libc::c_int,
    pub CB_NLSF_Q15: *const libc::c_short,
    pub Rates_Q5: *const libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SKP_Silk_PLC_struct {
    pub pitchL_Q8: libc::c_int,
    pub LTPCoef_Q14: [libc::c_short; 5],
    pub prevLPC_Q12: [libc::c_short; 16],
    pub last_frame_lost: libc::c_int,
    pub rand_seed: libc::c_int,
    pub randScale_Q14: libc::c_short,
    pub conc_energy: libc::c_int,
    pub conc_energy_shift: libc::c_int,
    pub prevLTP_scale_Q14: libc::c_short,
    pub prevGain_Q16: [libc::c_int; 4],
    pub fs_kHz: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SKP_Silk_CNG_struct {
    pub CNG_exc_buf_Q10: [libc::c_int; 480],
    pub CNG_smth_NLSF_Q15: [libc::c_int; 16],
    pub CNG_synth_state: [libc::c_int; 16],
    pub CNG_smth_Gain_Q16: libc::c_int,
    pub rand_seed: libc::c_int,
    pub fs_kHz: libc::c_int,
}
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_decode_parameters(
    mut psDec: *mut SKP_Silk_decoder_state,
    mut psDecCtrl: *mut SKP_Silk_decoder_control,
    mut q: *mut libc::c_int,
    fullDecoding: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut Ix: libc::c_int = 0;
    let mut fs_kHz_dec: libc::c_int = 0;
    let mut nBytesUsed: libc::c_int = 0;
    let mut Ixs: [libc::c_int; 4] = [0; 4];
    let mut GainsIndices: [libc::c_int; 4] = [0; 4];
    let mut NLSFIndices: [libc::c_int; 10] = [0; 10];
    let mut pNLSF_Q15: [libc::c_int; 16] = [0; 16];
    let mut pNLSF0_Q15: [libc::c_int; 16] = [0; 16];
    let mut cbk_ptr_Q14: *const libc::c_short = 0 as *const libc::c_short;
    let mut psNLSF_CB: *const SKP_Silk_NLSF_CB_struct = 0
        as *const SKP_Silk_NLSF_CB_struct;
    let mut psRC: *mut SKP_Silk_range_coder_state = &mut (*psDec).sRC;
    if (*psDec).nFramesDecoded == 0 as libc::c_int {
        SKP_Silk_range_decoder(
            &mut Ix,
            psRC,
            SKP_Silk_SamplingRates_CDF.as_ptr(),
            SKP_Silk_SamplingRates_offset,
        );
        if Ix < 0 as libc::c_int || Ix > 3 as libc::c_int {
            (*psRC).error = -(7 as libc::c_int);
            return;
        }
        fs_kHz_dec = SKP_Silk_SamplingRates_table[Ix as usize];
        SKP_Silk_decoder_set_fs(psDec, fs_kHz_dec);
    }
    if (*psDec).nFramesDecoded == 0 as libc::c_int {
        SKP_Silk_range_decoder(
            &mut Ix,
            psRC,
            SKP_Silk_type_offset_CDF.as_ptr(),
            SKP_Silk_type_offset_CDF_offset,
        );
    } else {
        SKP_Silk_range_decoder(
            &mut Ix,
            psRC,
            (SKP_Silk_type_offset_joint_CDF[(*psDec).typeOffsetPrev as usize]).as_ptr(),
            SKP_Silk_type_offset_CDF_offset,
        );
    }
    (*psDecCtrl).sigtype = Ix >> 1 as libc::c_int;
    (*psDecCtrl).QuantOffsetType = Ix & 1 as libc::c_int;
    (*psDec).typeOffsetPrev = Ix;
    if (*psDec).nFramesDecoded == 0 as libc::c_int {
        SKP_Silk_range_decoder(
            &mut *GainsIndices.as_mut_ptr().offset(0 as libc::c_int as isize),
            psRC,
            (SKP_Silk_gain_CDF[(*psDecCtrl).sigtype as usize]).as_ptr(),
            SKP_Silk_gain_CDF_offset,
        );
    } else {
        SKP_Silk_range_decoder(
            &mut *GainsIndices.as_mut_ptr().offset(0 as libc::c_int as isize),
            psRC,
            SKP_Silk_delta_gain_CDF.as_ptr(),
            SKP_Silk_delta_gain_CDF_offset,
        );
    }
    i = 1 as libc::c_int;
    while i < 4 as libc::c_int {
        SKP_Silk_range_decoder(
            &mut *GainsIndices.as_mut_ptr().offset(i as isize),
            psRC,
            SKP_Silk_delta_gain_CDF.as_ptr(),
            SKP_Silk_delta_gain_CDF_offset,
        );
        i += 1;
    }
    SKP_Silk_gains_dequant(
        ((*psDecCtrl).Gains_Q16).as_mut_ptr(),
        GainsIndices.as_mut_ptr() as *const libc::c_int,
        &mut (*psDec).LastGainIndex,
        (*psDec).nFramesDecoded,
    );
    psNLSF_CB = (*psDec).psNLSF_CB[(*psDecCtrl).sigtype as usize];
    SKP_Silk_range_decoder_multi(
        NLSFIndices.as_mut_ptr(),
        psRC,
        (*psNLSF_CB).StartPtr,
        (*psNLSF_CB).MiddleIx,
        (*psNLSF_CB).nStages,
    );
    SKP_Silk_NLSF_MSVQ_decode(
        pNLSF_Q15.as_mut_ptr(),
        psNLSF_CB,
        NLSFIndices.as_mut_ptr(),
        (*psDec).LPC_order,
    );
    SKP_Silk_range_decoder(
        &mut (*psDecCtrl).NLSFInterpCoef_Q2,
        psRC,
        SKP_Silk_NLSF_interpolation_factor_CDF.as_ptr(),
        SKP_Silk_NLSF_interpolation_factor_offset,
    );
    if (*psDec).first_frame_after_reset == 1 as libc::c_int {
        (*psDecCtrl).NLSFInterpCoef_Q2 = 4 as libc::c_int;
    }
    if fullDecoding != 0 {
        SKP_Silk_NLSF2A_stable(
            ((*psDecCtrl).PredCoef_Q12[1 as libc::c_int as usize]).as_mut_ptr(),
            pNLSF_Q15.as_mut_ptr() as *const libc::c_int,
            (*psDec).LPC_order,
        );
        if (*psDecCtrl).NLSFInterpCoef_Q2 < 4 as libc::c_int {
            i = 0 as libc::c_int;
            while i < (*psDec).LPC_order {
                pNLSF0_Q15[i
                    as usize] = (*psDec).prevNLSF_Q15[i as usize]
                    + ((*psDecCtrl).NLSFInterpCoef_Q2
                        * (pNLSF_Q15[i as usize] - (*psDec).prevNLSF_Q15[i as usize])
                        >> 2 as libc::c_int);
                i += 1;
            }
            SKP_Silk_NLSF2A_stable(
                ((*psDecCtrl).PredCoef_Q12[0 as libc::c_int as usize]).as_mut_ptr(),
                pNLSF0_Q15.as_mut_ptr() as *const libc::c_int,
                (*psDec).LPC_order,
            );
        } else {
            memcpy(
                ((*psDecCtrl).PredCoef_Q12[0 as libc::c_int as usize]).as_mut_ptr()
                    as *mut libc::c_void,
                ((*psDecCtrl).PredCoef_Q12[1 as libc::c_int as usize]).as_mut_ptr()
                    as *const libc::c_void,
                ((*psDec).LPC_order as libc::c_ulong)
                    .wrapping_mul(
                        ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                    ),
            );
        }
    }
    memcpy(
        ((*psDec).prevNLSF_Q15).as_mut_ptr() as *mut libc::c_void,
        pNLSF_Q15.as_mut_ptr() as *const libc::c_void,
        ((*psDec).LPC_order as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    if (*psDec).lossCnt != 0 {
        SKP_Silk_bwexpander(
            ((*psDecCtrl).PredCoef_Q12[0 as libc::c_int as usize]).as_mut_ptr(),
            (*psDec).LPC_order,
            63570 as libc::c_int,
        );
        SKP_Silk_bwexpander(
            ((*psDecCtrl).PredCoef_Q12[1 as libc::c_int as usize]).as_mut_ptr(),
            (*psDec).LPC_order,
            63570 as libc::c_int,
        );
    }
    if (*psDecCtrl).sigtype == 0 as libc::c_int {
        if (*psDec).fs_kHz == 8 as libc::c_int {
            SKP_Silk_range_decoder(
                &mut *Ixs.as_mut_ptr().offset(0 as libc::c_int as isize),
                psRC,
                SKP_Silk_pitch_lag_NB_CDF.as_ptr(),
                SKP_Silk_pitch_lag_NB_CDF_offset,
            );
        } else if (*psDec).fs_kHz == 12 as libc::c_int {
            SKP_Silk_range_decoder(
                &mut *Ixs.as_mut_ptr().offset(0 as libc::c_int as isize),
                psRC,
                SKP_Silk_pitch_lag_MB_CDF.as_ptr(),
                SKP_Silk_pitch_lag_MB_CDF_offset,
            );
        } else if (*psDec).fs_kHz == 16 as libc::c_int {
            SKP_Silk_range_decoder(
                &mut *Ixs.as_mut_ptr().offset(0 as libc::c_int as isize),
                psRC,
                SKP_Silk_pitch_lag_WB_CDF.as_ptr(),
                SKP_Silk_pitch_lag_WB_CDF_offset,
            );
        } else {
            SKP_Silk_range_decoder(
                &mut *Ixs.as_mut_ptr().offset(0 as libc::c_int as isize),
                psRC,
                SKP_Silk_pitch_lag_SWB_CDF.as_ptr(),
                SKP_Silk_pitch_lag_SWB_CDF_offset,
            );
        }
        if (*psDec).fs_kHz == 8 as libc::c_int {
            SKP_Silk_range_decoder(
                &mut *Ixs.as_mut_ptr().offset(1 as libc::c_int as isize),
                psRC,
                SKP_Silk_pitch_contour_NB_CDF.as_ptr(),
                SKP_Silk_pitch_contour_NB_CDF_offset,
            );
        } else {
            SKP_Silk_range_decoder(
                &mut *Ixs.as_mut_ptr().offset(1 as libc::c_int as isize),
                psRC,
                SKP_Silk_pitch_contour_CDF.as_ptr(),
                SKP_Silk_pitch_contour_CDF_offset,
            );
        }
        SKP_Silk_decode_pitch(
            Ixs[0 as libc::c_int as usize],
            Ixs[1 as libc::c_int as usize],
            ((*psDecCtrl).pitchL).as_mut_ptr(),
            (*psDec).fs_kHz,
        );
        SKP_Silk_range_decoder(
            &mut (*psDecCtrl).PERIndex,
            psRC,
            SKP_Silk_LTP_per_index_CDF.as_ptr(),
            SKP_Silk_LTP_per_index_CDF_offset,
        );
        cbk_ptr_Q14 = SKP_Silk_LTP_vq_ptrs_Q14[(*psDecCtrl).PERIndex as usize];
        k = 0 as libc::c_int;
        while k < 4 as libc::c_int {
            SKP_Silk_range_decoder(
                &mut Ix,
                psRC,
                SKP_Silk_LTP_gain_CDF_ptrs[(*psDecCtrl).PERIndex as usize],
                SKP_Silk_LTP_gain_CDF_offsets[(*psDecCtrl).PERIndex as usize],
            );
            i = 0 as libc::c_int;
            while i < 5 as libc::c_int {
                (*psDecCtrl)
                    .LTPCoef_Q14[(k * 5 as libc::c_int + i)
                    as usize] = *cbk_ptr_Q14
                    .offset((Ix * 5 as libc::c_int + i) as isize);
                i += 1;
            }
            k += 1;
        }
        SKP_Silk_range_decoder(
            &mut Ix,
            psRC,
            SKP_Silk_LTPscale_CDF.as_ptr(),
            SKP_Silk_LTPscale_offset,
        );
        (*psDecCtrl)
            .LTP_scale_Q14 = SKP_Silk_LTPScales_table_Q14[Ix as usize] as libc::c_int;
    } else {
        memset(
            ((*psDecCtrl).pitchL).as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            (4 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
        );
        memset(
            ((*psDecCtrl).LTPCoef_Q14).as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            ((5 as libc::c_int * 4 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_short>() as libc::c_ulong),
        );
        (*psDecCtrl).PERIndex = 0 as libc::c_int;
        (*psDecCtrl).LTP_scale_Q14 = 0 as libc::c_int;
    }
    SKP_Silk_range_decoder(
        &mut Ix,
        psRC,
        SKP_Silk_Seed_CDF.as_ptr(),
        SKP_Silk_Seed_offset,
    );
    (*psDecCtrl).Seed = Ix;
    SKP_Silk_decode_pulses(psRC, psDecCtrl, q, (*psDec).frame_length);
    SKP_Silk_range_decoder(
        &mut (*psDec).vadFlag,
        psRC,
        SKP_Silk_vadflag_CDF.as_ptr(),
        SKP_Silk_vadflag_offset,
    );
    SKP_Silk_range_decoder(
        &mut (*psDec).FrameTermination,
        psRC,
        SKP_Silk_FrameTermination_CDF.as_ptr(),
        SKP_Silk_FrameTermination_offset,
    );
    SKP_Silk_range_coder_get_length(psRC, &mut nBytesUsed);
    (*psDec).nBytesLeft = (*psRC).bufferLength - nBytesUsed;
    if (*psDec).nBytesLeft < 0 as libc::c_int {
        (*psRC).error = -(6 as libc::c_int);
    }
    if (*psDec).nBytesLeft == 0 as libc::c_int {
        SKP_Silk_range_coder_check_after_decoding(psRC);
    }
}
