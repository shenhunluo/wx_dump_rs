#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn SKP_Silk_NLSF_VQ_weights_laroia(
        pNLSFW_Q6: *mut libc::c_int,
        pNLSF_Q15: *const libc::c_int,
        D: libc::c_int,
    );
    fn SKP_Silk_NLSF2A_stable(
        pAR_Q12: *mut libc::c_short,
        pNLSF: *const libc::c_int,
        LPC_order: libc::c_int,
    );
    fn SKP_Silk_interpolate(
        xi: *mut libc::c_int,
        x0: *const libc::c_int,
        x1: *const libc::c_int,
        ifact_Q2: libc::c_int,
        d: libc::c_int,
    );
    fn SKP_Silk_NLSF_MSVQ_encode_FIX(
        NLSFIndices: *mut libc::c_int,
        pNLSF_Q15: *mut libc::c_int,
        psNLSF_CB: *const SKP_Silk_NLSF_CB_struct,
        pNLSF_q_Q15_prev: *const libc::c_int,
        pW_Q6: *const libc::c_int,
        NLSF_mu_Q15: libc::c_int,
        NLSF_mu_fluc_red_Q16: libc::c_int,
        NLSF_MSVQ_Survivors: libc::c_int,
        LPC_order: libc::c_int,
        deactivate_fluc_red: libc::c_int,
    );
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
pub struct SKP_Silk_nsq_state {
    pub xq: [libc::c_short; 960],
    pub sLTP_shp_Q10: [libc::c_int; 960],
    pub sLPC_Q14: [libc::c_int; 152],
    pub sAR2_Q14: [libc::c_int; 16],
    pub sLF_AR_shp_Q12: libc::c_int,
    pub lagPrev: libc::c_int,
    pub sLTP_buf_idx: libc::c_int,
    pub sLTP_shp_buf_idx: libc::c_int,
    pub rand_seed: libc::c_int,
    pub prev_inv_gain_Q16: libc::c_int,
    pub rewhite_flag: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SKP_SILK_LBRR_struct {
    pub payload: [libc::c_uchar; 1024],
    pub nBytes: libc::c_int,
    pub usage: libc::c_int,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SKP_Silk_detect_SWB_state {
    pub S_HP_8_kHz: [[libc::c_int; 2]; 3],
    pub ConsecSmplsAboveThres: libc::c_int,
    pub ActiveSpeech_ms: libc::c_int,
    pub SWB_detected: libc::c_int,
    pub WB_detected: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SKP_Silk_LP_state {
    pub In_LP_State: [libc::c_int; 2],
    pub transition_frame_no: libc::c_int,
    pub mode: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SKP_Silk_NLSF_CBS {
    pub nVectors: libc::c_int,
    pub CB_NLSF_Q15: *const libc::c_short,
    pub Rates_Q5: *const libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SKP_Silk_NLSF_CB_struct {
    pub nStages: libc::c_int,
    pub CBStages: *const SKP_Silk_NLSF_CBS,
    pub NDeltaMin_Q15: *const libc::c_int,
    pub CDF: *const libc::c_ushort,
    pub StartPtr: *const *const libc::c_ushort,
    pub MiddleIx: *const libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SKP_Silk_encoder_state {
    pub sRC: SKP_Silk_range_coder_state,
    pub sRC_LBRR: SKP_Silk_range_coder_state,
    pub sNSQ: SKP_Silk_nsq_state,
    pub sNSQ_LBRR: SKP_Silk_nsq_state,
    pub In_HP_State: [libc::c_int; 2],
    pub sLP: SKP_Silk_LP_state,
    pub sVAD: SKP_Silk_VAD_state,
    pub LBRRprevLastGainIndex: libc::c_int,
    pub prev_sigtype: libc::c_int,
    pub typeOffsetPrev: libc::c_int,
    pub prevLag: libc::c_int,
    pub prev_lagIndex: libc::c_int,
    pub API_fs_Hz: libc::c_int,
    pub prev_API_fs_Hz: libc::c_int,
    pub maxInternal_fs_kHz: libc::c_int,
    pub fs_kHz: libc::c_int,
    pub fs_kHz_changed: libc::c_int,
    pub frame_length: libc::c_int,
    pub subfr_length: libc::c_int,
    pub la_pitch: libc::c_int,
    pub la_shape: libc::c_int,
    pub shapeWinLength: libc::c_int,
    pub TargetRate_bps: libc::c_int,
    pub PacketSize_ms: libc::c_int,
    pub PacketLoss_perc: libc::c_int,
    pub frameCounter: libc::c_int,
    pub Complexity: libc::c_int,
    pub nStatesDelayedDecision: libc::c_int,
    pub useInterpolatedNLSFs: libc::c_int,
    pub shapingLPCOrder: libc::c_int,
    pub predictLPCOrder: libc::c_int,
    pub pitchEstimationComplexity: libc::c_int,
    pub pitchEstimationLPCOrder: libc::c_int,
    pub pitchEstimationThreshold_Q16: libc::c_int,
    pub LTPQuantLowComplexity: libc::c_int,
    pub NLSF_MSVQ_Survivors: libc::c_int,
    pub first_frame_after_reset: libc::c_int,
    pub controlled_since_last_payload: libc::c_int,
    pub warping_Q16: libc::c_int,
    pub inputBuf: [libc::c_short; 480],
    pub inputBufIx: libc::c_int,
    pub nFramesInPayloadBuf: libc::c_int,
    pub nBytesInPayloadBuf: libc::c_int,
    pub frames_since_onset: libc::c_int,
    pub psNLSF_CB: [*const SKP_Silk_NLSF_CB_struct; 2],
    pub LBRR_buffer: [SKP_SILK_LBRR_struct; 2],
    pub oldest_LBRR_idx: libc::c_int,
    pub useInBandFEC: libc::c_int,
    pub LBRR_enabled: libc::c_int,
    pub LBRR_GainIncreases: libc::c_int,
    pub bitrateDiff: libc::c_int,
    pub bitrate_threshold_up: libc::c_int,
    pub bitrate_threshold_down: libc::c_int,
    pub resampler_state: SKP_Silk_resampler_state_struct,
    pub noSpeechCounter: libc::c_int,
    pub useDTX: libc::c_int,
    pub inDTX: libc::c_int,
    pub vadFlag: libc::c_int,
    pub sSWBdetect: SKP_Silk_detect_SWB_state,
    pub q: [libc::c_schar; 480],
    pub q_LBRR: [libc::c_schar; 480],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SKP_Silk_encoder_control {
    pub lagIndex: libc::c_int,
    pub contourIndex: libc::c_int,
    pub PERIndex: libc::c_int,
    pub LTPIndex: [libc::c_int; 4],
    pub NLSFIndices: [libc::c_int; 10],
    pub NLSFInterpCoef_Q2: libc::c_int,
    pub GainsIndices: [libc::c_int; 4],
    pub Seed: libc::c_int,
    pub LTP_scaleIndex: libc::c_int,
    pub RateLevelIndex: libc::c_int,
    pub QuantOffsetType: libc::c_int,
    pub sigtype: libc::c_int,
    pub pitchL: [libc::c_int; 4],
    pub LBRR_usage: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SKP_Silk_shape_state_FIX {
    pub LastGainIndex: libc::c_int,
    pub HarmBoost_smth_Q16: libc::c_int,
    pub HarmShapeGain_smth_Q16: libc::c_int,
    pub Tilt_smth_Q16: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SKP_Silk_prefilter_state_FIX {
    pub sLTP_shp: [libc::c_short; 512],
    pub sAR_shp: [libc::c_int; 17],
    pub sLTP_shp_buf_idx: libc::c_int,
    pub sLF_AR_shp_Q12: libc::c_int,
    pub sLF_MA_shp_Q12: libc::c_int,
    pub sHarmHP: libc::c_int,
    pub rand_seed: libc::c_int,
    pub lagPrev: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SKP_Silk_predict_state_FIX {
    pub pitch_LPC_win_length: libc::c_int,
    pub min_pitch_lag: libc::c_int,
    pub max_pitch_lag: libc::c_int,
    pub prev_NLSFq_Q15: [libc::c_int; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SKP_Silk_encoder_state_FIX {
    pub sCmn: SKP_Silk_encoder_state,
    pub variable_HP_smth1_Q15: libc::c_int,
    pub variable_HP_smth2_Q15: libc::c_int,
    pub sShape: SKP_Silk_shape_state_FIX,
    pub sPrefilt: SKP_Silk_prefilter_state_FIX,
    pub sPred: SKP_Silk_predict_state_FIX,
    pub x_buf: [libc::c_short; 1080],
    pub LTPCorr_Q15: libc::c_int,
    pub mu_LTP_Q8: libc::c_int,
    pub SNR_dB_Q7: libc::c_int,
    pub avgGain_Q16: libc::c_int,
    pub avgGain_Q16_one_bit_per_sample: libc::c_int,
    pub BufferedInChannel_ms: libc::c_int,
    pub speech_activity_Q8: libc::c_int,
    pub prevLTPredCodGain_Q7: libc::c_int,
    pub HPLTPredCodGain_Q7: libc::c_int,
    pub inBandFEC_SNR_comp_Q8: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SKP_Silk_encoder_control_FIX {
    pub sCmn: SKP_Silk_encoder_control,
    pub Gains_Q16: [libc::c_int; 4],
    pub PredCoef_Q12: [[libc::c_short; 16]; 2],
    pub LTPCoef_Q14: [libc::c_short; 20],
    pub LTP_scale_Q14: libc::c_int,
    pub AR1_Q13: [libc::c_short; 64],
    pub AR2_Q13: [libc::c_short; 64],
    pub LF_shp_Q14: [libc::c_int; 4],
    pub GainsPre_Q14: [libc::c_int; 4],
    pub HarmBoost_Q14: [libc::c_int; 4],
    pub Tilt_Q14: [libc::c_int; 4],
    pub HarmShapeGain_Q14: [libc::c_int; 4],
    pub Lambda_Q10: libc::c_int,
    pub input_quality_Q14: libc::c_int,
    pub coding_quality_Q14: libc::c_int,
    pub pitch_freq_low_Hz: libc::c_int,
    pub current_SNR_dB_Q7: libc::c_int,
    pub sparseness_Q8: libc::c_int,
    pub predGain_Q16: libc::c_int,
    pub LTPredCodGain_Q7: libc::c_int,
    pub input_quality_bands_Q15: [libc::c_int; 4],
    pub input_tilt_Q15: libc::c_int,
    pub ResNrg: [libc::c_int; 4],
    pub ResNrgQ: [libc::c_int; 4],
}
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_process_NLSFs_FIX(
    mut psEnc: *mut SKP_Silk_encoder_state_FIX,
    mut psEncCtrl: *mut SKP_Silk_encoder_control_FIX,
    mut pNLSF_Q15: *mut libc::c_int,
) {
    let mut doInterpolate: libc::c_int = 0;
    let mut pNLSFW_Q6: [libc::c_int; 16] = [0; 16];
    let mut NLSF_mu_Q15: libc::c_int = 0;
    let mut NLSF_mu_fluc_red_Q16: libc::c_int = 0;
    let mut i_sqr_Q15: libc::c_int = 0;
    let mut psNLSF_CB: *const SKP_Silk_NLSF_CB_struct = 0
        as *const SKP_Silk_NLSF_CB_struct;
    let mut pNLSF0_temp_Q15: [libc::c_int; 16] = [0; 16];
    let mut pNLSFW0_temp_Q6: [libc::c_int; 16] = [0; 16];
    let mut i: libc::c_int = 0;
    if (*psEncCtrl).sCmn.sigtype == 0 as libc::c_int {
        NLSF_mu_Q15 = 66 as libc::c_int
            + ((-(8388 as libc::c_int) >> 16 as libc::c_int)
                * (*psEnc).speech_activity_Q8 as libc::c_short as libc::c_int
                + ((-(8388 as libc::c_int) & 0xffff as libc::c_int)
                    * (*psEnc).speech_activity_Q8 as libc::c_short as libc::c_int
                    >> 16 as libc::c_int));
        NLSF_mu_fluc_red_Q16 = 6554 as libc::c_int
            + ((-(838848 as libc::c_int) >> 16 as libc::c_int)
                * (*psEnc).speech_activity_Q8 as libc::c_short as libc::c_int
                + ((-(838848 as libc::c_int) & 0xffff as libc::c_int)
                    * (*psEnc).speech_activity_Q8 as libc::c_short as libc::c_int
                    >> 16 as libc::c_int));
    } else {
        NLSF_mu_Q15 = 164 as libc::c_int
            + ((-(33554 as libc::c_int) >> 16 as libc::c_int)
                * (*psEnc).speech_activity_Q8 as libc::c_short as libc::c_int
                + ((-(33554 as libc::c_int) & 0xffff as libc::c_int)
                    * (*psEnc).speech_activity_Q8 as libc::c_short as libc::c_int
                    >> 16 as libc::c_int));
        NLSF_mu_fluc_red_Q16 = 13107 as libc::c_int
            + ((-(1677696 as libc::c_int) >> 16 as libc::c_int)
                * ((*psEnc).speech_activity_Q8 + (*psEncCtrl).sparseness_Q8)
                    as libc::c_short as libc::c_int
                + ((-(1677696 as libc::c_int) & 0xffff as libc::c_int)
                    * ((*psEnc).speech_activity_Q8 + (*psEncCtrl).sparseness_Q8)
                        as libc::c_short as libc::c_int >> 16 as libc::c_int));
    }
    NLSF_mu_Q15 = if NLSF_mu_Q15 > 1 as libc::c_int {
        NLSF_mu_Q15
    } else {
        1 as libc::c_int
    };
    SKP_Silk_NLSF_VQ_weights_laroia(
        pNLSFW_Q6.as_mut_ptr(),
        pNLSF_Q15,
        (*psEnc).sCmn.predictLPCOrder,
    );
    doInterpolate = ((*psEnc).sCmn.useInterpolatedNLSFs == 1 as libc::c_int
        && (*psEncCtrl).sCmn.NLSFInterpCoef_Q2 < (1 as libc::c_int) << 2 as libc::c_int)
        as libc::c_int;
    if doInterpolate != 0 {
        SKP_Silk_interpolate(
            pNLSF0_temp_Q15.as_mut_ptr(),
            ((*psEnc).sPred.prev_NLSFq_Q15).as_mut_ptr() as *const libc::c_int,
            pNLSF_Q15 as *const libc::c_int,
            (*psEncCtrl).sCmn.NLSFInterpCoef_Q2,
            (*psEnc).sCmn.predictLPCOrder,
        );
        SKP_Silk_NLSF_VQ_weights_laroia(
            pNLSFW0_temp_Q6.as_mut_ptr(),
            pNLSF0_temp_Q15.as_mut_ptr(),
            (*psEnc).sCmn.predictLPCOrder,
        );
        i_sqr_Q15 = ((*psEncCtrl).sCmn.NLSFInterpCoef_Q2 as libc::c_short as libc::c_int
            * (*psEncCtrl).sCmn.NLSFInterpCoef_Q2 as libc::c_short as libc::c_int)
            << 11 as libc::c_int;
        i = 0 as libc::c_int;
        while i < (*psEnc).sCmn.predictLPCOrder {
            pNLSFW_Q6[i
                as usize] = (pNLSFW_Q6[i as usize] >> 1 as libc::c_int)
                + ((pNLSFW0_temp_Q6[i as usize] >> 16 as libc::c_int)
                    * i_sqr_Q15 as libc::c_short as libc::c_int
                    + ((pNLSFW0_temp_Q6[i as usize] & 0xffff as libc::c_int)
                        * i_sqr_Q15 as libc::c_short as libc::c_int
                        >> 16 as libc::c_int));
            i += 1;
            i;
        }
    }
    psNLSF_CB = (*psEnc).sCmn.psNLSF_CB[(*psEncCtrl).sCmn.sigtype as usize];
    SKP_Silk_NLSF_MSVQ_encode_FIX(
        ((*psEncCtrl).sCmn.NLSFIndices).as_mut_ptr(),
        pNLSF_Q15,
        psNLSF_CB,
        ((*psEnc).sPred.prev_NLSFq_Q15).as_mut_ptr(),
        pNLSFW_Q6.as_mut_ptr(),
        NLSF_mu_Q15,
        NLSF_mu_fluc_red_Q16,
        (*psEnc).sCmn.NLSF_MSVQ_Survivors,
        (*psEnc).sCmn.predictLPCOrder,
        (*psEnc).sCmn.first_frame_after_reset,
    );
    SKP_Silk_NLSF2A_stable(
        ((*psEncCtrl).PredCoef_Q12[1 as libc::c_int as usize]).as_mut_ptr(),
        pNLSF_Q15 as *const libc::c_int,
        (*psEnc).sCmn.predictLPCOrder,
    );
    if doInterpolate != 0 {
        SKP_Silk_interpolate(
            pNLSF0_temp_Q15.as_mut_ptr(),
            ((*psEnc).sPred.prev_NLSFq_Q15).as_mut_ptr() as *const libc::c_int,
            pNLSF_Q15 as *const libc::c_int,
            (*psEncCtrl).sCmn.NLSFInterpCoef_Q2,
            (*psEnc).sCmn.predictLPCOrder,
        );
        SKP_Silk_NLSF2A_stable(
            ((*psEncCtrl).PredCoef_Q12[0 as libc::c_int as usize]).as_mut_ptr(),
            pNLSF0_temp_Q15.as_mut_ptr() as *const libc::c_int,
            (*psEnc).sCmn.predictLPCOrder,
        );
    } else {
        memcpy(
            ((*psEncCtrl).PredCoef_Q12[0 as libc::c_int as usize]).as_mut_ptr()
                as *mut libc::c_void,
            ((*psEncCtrl).PredCoef_Q12[1 as libc::c_int as usize]).as_mut_ptr()
                as *const libc::c_void,
            ((*psEnc).sCmn.predictLPCOrder as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_short>() as libc::c_ulong),
        );
    };
}
