#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
pub type __int64_t = libc::c_longlong;
pub type int64_t = __int64_t;
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
pub struct SkpSilkNlsfCbStruct {
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
    pub psNLSF_CB: [*const SkpSilkNlsfCbStruct; 2],
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
pub unsafe extern "C" fn SKP_Silk_warped_LPC_analysis_filter_FIX(
    mut state: *mut libc::c_int,
    mut res: *mut libc::c_short,
    mut coef_Q13: *const libc::c_short,
    mut input: *const libc::c_short,
    lambda_Q16: libc::c_short,
    length: libc::c_int,
    order: libc::c_int,
) {
    let mut n: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut acc_Q11: libc::c_int = 0;
    let mut tmp1: libc::c_int = 0;
    let mut tmp2: libc::c_int = 0;
    n = 0 as libc::c_int;
    while n < length {
        tmp2 = *state.offset(0 as libc::c_int as isize)
            + ((*state.offset(1 as libc::c_int as isize) >> 16 as libc::c_int)
                * lambda_Q16 as libc::c_int
                + ((*state.offset(1 as libc::c_int as isize) & 0xffff as libc::c_int)
                    * lambda_Q16 as libc::c_int >> 16 as libc::c_int));
        *state
            .offset(
                0 as libc::c_int as isize,
            ) = (*input.offset(n as isize) as libc::c_int) << 14 as libc::c_int;
        tmp1 = *state.offset(1 as libc::c_int as isize)
            + ((*state.offset(2 as libc::c_int as isize) - tmp2 >> 16 as libc::c_int)
                * lambda_Q16 as libc::c_int
                + ((*state.offset(2 as libc::c_int as isize) - tmp2
                    & 0xffff as libc::c_int) * lambda_Q16 as libc::c_int
                    >> 16 as libc::c_int));
        *state.offset(1 as libc::c_int as isize) = tmp2;
        acc_Q11 = (tmp2 >> 16 as libc::c_int)
            * *coef_Q13.offset(0 as libc::c_int as isize) as libc::c_int
            + ((tmp2 & 0xffff as libc::c_int)
                * *coef_Q13.offset(0 as libc::c_int as isize) as libc::c_int
                >> 16 as libc::c_int);
        i = 2 as libc::c_int;
        while i < order {
            tmp2 = *state.offset(i as isize)
                + ((*state.offset((i + 1 as libc::c_int) as isize) - tmp1
                    >> 16 as libc::c_int) * lambda_Q16 as libc::c_int
                    + ((*state.offset((i + 1 as libc::c_int) as isize) - tmp1
                        & 0xffff as libc::c_int) * lambda_Q16 as libc::c_int
                        >> 16 as libc::c_int));
            *state.offset(i as isize) = tmp1;
            acc_Q11 = acc_Q11
                + ((tmp1 >> 16 as libc::c_int)
                    * *coef_Q13.offset((i - 1 as libc::c_int) as isize) as libc::c_int
                    + ((tmp1 & 0xffff as libc::c_int)
                        * *coef_Q13.offset((i - 1 as libc::c_int) as isize)
                            as libc::c_int >> 16 as libc::c_int));
            tmp1 = *state.offset((i + 1 as libc::c_int) as isize)
                + ((*state.offset((i + 2 as libc::c_int) as isize) - tmp2
                    >> 16 as libc::c_int) * lambda_Q16 as libc::c_int
                    + ((*state.offset((i + 2 as libc::c_int) as isize) - tmp2
                        & 0xffff as libc::c_int) * lambda_Q16 as libc::c_int
                        >> 16 as libc::c_int));
            *state.offset((i + 1 as libc::c_int) as isize) = tmp2;
            acc_Q11 = acc_Q11
                + ((tmp2 >> 16 as libc::c_int)
                    * *coef_Q13.offset(i as isize) as libc::c_int
                    + ((tmp2 & 0xffff as libc::c_int)
                        * *coef_Q13.offset(i as isize) as libc::c_int
                        >> 16 as libc::c_int));
            i += 2 as libc::c_int;
        }
        *state.offset(order as isize) = tmp1;
        acc_Q11 = acc_Q11
            + ((tmp1 >> 16 as libc::c_int)
                * *coef_Q13.offset((order - 1 as libc::c_int) as isize) as libc::c_int
                + ((tmp1 & 0xffff as libc::c_int)
                    * *coef_Q13.offset((order - 1 as libc::c_int) as isize)
                        as libc::c_int >> 16 as libc::c_int));
        *res
            .offset(
                n as isize,
            ) = (if *input.offset(n as isize) as libc::c_int
            - (if 11 as libc::c_int == 1 as libc::c_int {
                (acc_Q11 >> 1 as libc::c_int) + (acc_Q11 & 1 as libc::c_int)
            } else {
                (acc_Q11 >> 11 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                    >> 1 as libc::c_int
            }) > 0x7fff as libc::c_int
        {
            0x7fff as libc::c_int
        } else if *input.offset(n as isize) as libc::c_int
            - (if 11 as libc::c_int == 1 as libc::c_int {
                (acc_Q11 >> 1 as libc::c_int) + (acc_Q11 & 1 as libc::c_int)
            } else {
                (acc_Q11 >> 11 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                    >> 1 as libc::c_int
            }) < 0x8000 as libc::c_int as libc::c_short as libc::c_int
        {
            0x8000 as libc::c_int as libc::c_short as libc::c_int
        } else {
            *input.offset(n as isize) as libc::c_int
                - (if 11 as libc::c_int == 1 as libc::c_int {
                    (acc_Q11 >> 1 as libc::c_int) + (acc_Q11 & 1 as libc::c_int)
                } else {
                    (acc_Q11 >> 11 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                        >> 1 as libc::c_int
                })
        }) as libc::c_short;
        n += 1;
        n;
    }
}
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_prefilter_FIX(
    mut psEnc: *mut SKP_Silk_encoder_state_FIX,
    mut psEncCtrl: *const SKP_Silk_encoder_control_FIX,
    mut xw: *mut libc::c_short,
    mut x: *const libc::c_short,
) {
    let mut P: *mut SKP_Silk_prefilter_state_FIX = &mut (*psEnc).sPrefilt;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut lag: libc::c_int = 0;
    let mut tmp_32: libc::c_int = 0;
    let mut AR1_shp_Q13: *const libc::c_short = 0 as *const libc::c_short;
    let mut px: *const libc::c_short = 0 as *const libc::c_short;
    let mut pxw: *mut libc::c_short = 0 as *mut libc::c_short;
    let mut HarmShapeGain_Q12: libc::c_int = 0;
    let mut Tilt_Q14: libc::c_int = 0;
    let mut HarmShapeFIRPacked_Q12: libc::c_int = 0;
    let mut LF_shp_Q14: libc::c_int = 0;
    let mut x_filt_Q12: [libc::c_int; 120] = [0; 120];
    let mut st_res: [libc::c_short; 136] = [0; 136];
    let mut B_Q12: libc::c_int = 0;
    px = x;
    pxw = xw;
    lag = (*P).lagPrev;
    k = 0 as libc::c_int;
    while k < 4 as libc::c_int {
        if (*psEncCtrl).sCmn.sigtype == 0 as libc::c_int {
            lag = (*psEncCtrl).sCmn.pitchL[k as usize];
        }
        HarmShapeGain_Q12 = ((*psEncCtrl).HarmShapeGain_Q14[k as usize]
            >> 16 as libc::c_int)
            * (16384 as libc::c_int - (*psEncCtrl).HarmBoost_Q14[k as usize])
                as libc::c_short as libc::c_int
            + (((*psEncCtrl).HarmShapeGain_Q14[k as usize] & 0xffff as libc::c_int)
                * (16384 as libc::c_int - (*psEncCtrl).HarmBoost_Q14[k as usize])
                    as libc::c_short as libc::c_int >> 16 as libc::c_int);
        HarmShapeFIRPacked_Q12 = HarmShapeGain_Q12 >> 2 as libc::c_int;
        HarmShapeFIRPacked_Q12
            |= (HarmShapeGain_Q12 >> 1 as libc::c_int) << 16 as libc::c_int;
        Tilt_Q14 = (*psEncCtrl).Tilt_Q14[k as usize];
        LF_shp_Q14 = (*psEncCtrl).LF_shp_Q14[k as usize];
        AR1_shp_Q13 = &*((*psEncCtrl).AR1_Q13)
            .as_ptr()
            .offset((k * 16 as libc::c_int) as isize) as *const libc::c_short;
        SKP_Silk_warped_LPC_analysis_filter_FIX(
            ((*P).sAR_shp).as_mut_ptr(),
            st_res.as_mut_ptr(),
            AR1_shp_Q13,
            px,
            (*psEnc).sCmn.warping_Q16 as libc::c_short,
            (*psEnc).sCmn.subfr_length,
            (*psEnc).sCmn.shapingLPCOrder,
        );
        B_Q12 = if 2 as libc::c_int == 1 as libc::c_int {
            ((*psEncCtrl).GainsPre_Q14[k as usize] >> 1 as libc::c_int)
                + ((*psEncCtrl).GainsPre_Q14[k as usize] & 1 as libc::c_int)
        } else {
            ((*psEncCtrl).GainsPre_Q14[k as usize]
                >> 2 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                >> 1 as libc::c_int
        };
        tmp_32 = ((0.05f32
            * ((1 as libc::c_int as int64_t) << 26 as libc::c_int) as libc::c_float)
            as libc::c_double + 0.5f64) as libc::c_int
            + (*psEncCtrl).HarmBoost_Q14[k as usize] as libc::c_short as libc::c_int
                * HarmShapeGain_Q12 as libc::c_short as libc::c_int;
        tmp_32 = tmp_32
            + (*psEncCtrl).coding_quality_Q14 as libc::c_short as libc::c_int
                * ((0.1f32
                    * ((1 as libc::c_int as int64_t) << 12 as libc::c_int)
                        as libc::c_float) as libc::c_double + 0.5f64) as libc::c_int
                    as libc::c_short as libc::c_int;
        tmp_32 = (tmp_32 >> 16 as libc::c_int)
            * -(*psEncCtrl).GainsPre_Q14[k as usize] as libc::c_short as libc::c_int
            + ((tmp_32 & 0xffff as libc::c_int)
                * -(*psEncCtrl).GainsPre_Q14[k as usize] as libc::c_short as libc::c_int
                >> 16 as libc::c_int);
        tmp_32 = if 12 as libc::c_int == 1 as libc::c_int {
            (tmp_32 >> 1 as libc::c_int) + (tmp_32 & 1 as libc::c_int)
        } else {
            (tmp_32 >> 12 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                >> 1 as libc::c_int
        };
        B_Q12
            |= (if tmp_32 > 0x7fff as libc::c_int {
                0x7fff as libc::c_int
            } else {
                (if tmp_32 < 0x8000 as libc::c_int as libc::c_short as libc::c_int {
                    0x8000 as libc::c_int as libc::c_short as libc::c_int
                } else {
                    tmp_32
                })
            }) << 16 as libc::c_int;
        x_filt_Q12[0 as libc::c_int
            as usize] = st_res[0 as libc::c_int as usize] as libc::c_int
            * B_Q12 as libc::c_short as libc::c_int
            + (*P).sHarmHP as libc::c_short as libc::c_int
                * (B_Q12 >> 16 as libc::c_int);
        j = 1 as libc::c_int;
        while j < (*psEnc).sCmn.subfr_length {
            x_filt_Q12[j
                as usize] = st_res[j as usize] as libc::c_int
                * B_Q12 as libc::c_short as libc::c_int
                + st_res[(j - 1 as libc::c_int) as usize] as libc::c_int
                    * (B_Q12 >> 16 as libc::c_int);
            j += 1;
            j;
        }
        (*P)
            .sHarmHP = st_res[((*psEnc).sCmn.subfr_length - 1 as libc::c_int) as usize]
            as libc::c_int;
        SKP_Silk_prefilt_FIX(
            P,
            x_filt_Q12.as_mut_ptr(),
            pxw,
            HarmShapeFIRPacked_Q12,
            Tilt_Q14,
            LF_shp_Q14,
            lag,
            (*psEnc).sCmn.subfr_length,
        );
        px = px.offset((*psEnc).sCmn.subfr_length as isize);
        pxw = pxw.offset((*psEnc).sCmn.subfr_length as isize);
        k += 1;
        k;
    }
    (*P)
        .lagPrev = (*psEncCtrl)
        .sCmn
        .pitchL[(4 as libc::c_int - 1 as libc::c_int) as usize];
}
#[inline]
unsafe extern "C" fn SKP_Silk_prefilt_FIX(
    mut P: *mut SKP_Silk_prefilter_state_FIX,
    mut st_res_Q12: *mut libc::c_int,
    mut xw: *mut libc::c_short,
    mut HarmShapeFIRPacked_Q12: libc::c_int,
    mut Tilt_Q14: libc::c_int,
    mut LF_shp_Q14: libc::c_int,
    mut lag: libc::c_int,
    mut length: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut idx: libc::c_int = 0;
    let mut LTP_shp_buf_idx: libc::c_int = 0;
    let mut n_LTP_Q12: libc::c_int = 0;
    let mut n_Tilt_Q10: libc::c_int = 0;
    let mut n_LF_Q10: libc::c_int = 0;
    let mut sLF_MA_shp_Q12: libc::c_int = 0;
    let mut sLF_AR_shp_Q12: libc::c_int = 0;
    let mut LTP_shp_buf: *mut libc::c_short = 0 as *mut libc::c_short;
    LTP_shp_buf = ((*P).sLTP_shp).as_mut_ptr();
    LTP_shp_buf_idx = (*P).sLTP_shp_buf_idx;
    sLF_AR_shp_Q12 = (*P).sLF_AR_shp_Q12;
    sLF_MA_shp_Q12 = (*P).sLF_MA_shp_Q12;
    i = 0 as libc::c_int;
    while i < length {
        if lag > 0 as libc::c_int {
            idx = lag + LTP_shp_buf_idx;
            n_LTP_Q12 = *LTP_shp_buf
                .offset(
                    (idx - 3 as libc::c_int / 2 as libc::c_int - 1 as libc::c_int
                        & 512 as libc::c_int - 1 as libc::c_int) as isize,
                ) as libc::c_int
                * HarmShapeFIRPacked_Q12 as libc::c_short as libc::c_int;
            n_LTP_Q12 = n_LTP_Q12
                + *LTP_shp_buf
                    .offset(
                        (idx - 3 as libc::c_int / 2 as libc::c_int
                            & 512 as libc::c_int - 1 as libc::c_int) as isize,
                    ) as libc::c_int * (HarmShapeFIRPacked_Q12 >> 16 as libc::c_int);
            n_LTP_Q12 = n_LTP_Q12
                + *LTP_shp_buf
                    .offset(
                        (idx - 3 as libc::c_int / 2 as libc::c_int + 1 as libc::c_int
                            & 512 as libc::c_int - 1 as libc::c_int) as isize,
                    ) as libc::c_int
                    * HarmShapeFIRPacked_Q12 as libc::c_short as libc::c_int;
        } else {
            n_LTP_Q12 = 0 as libc::c_int;
        }
        n_Tilt_Q10 = (sLF_AR_shp_Q12 >> 16 as libc::c_int)
            * Tilt_Q14 as libc::c_short as libc::c_int
            + ((sLF_AR_shp_Q12 & 0xffff as libc::c_int)
                * Tilt_Q14 as libc::c_short as libc::c_int >> 16 as libc::c_int);
        n_LF_Q10 = (sLF_AR_shp_Q12 >> 16 as libc::c_int)
            * (LF_shp_Q14 >> 16 as libc::c_int)
            + ((sLF_AR_shp_Q12 & 0xffff as libc::c_int)
                * (LF_shp_Q14 >> 16 as libc::c_int) >> 16 as libc::c_int)
            + ((sLF_MA_shp_Q12 >> 16 as libc::c_int)
                * LF_shp_Q14 as libc::c_short as libc::c_int
                + ((sLF_MA_shp_Q12 & 0xffff as libc::c_int)
                    * LF_shp_Q14 as libc::c_short as libc::c_int >> 16 as libc::c_int));
        sLF_AR_shp_Q12 = *st_res_Q12.offset(i as isize)
            - (n_Tilt_Q10 << 2 as libc::c_int);
        sLF_MA_shp_Q12 = sLF_AR_shp_Q12 - (n_LF_Q10 << 2 as libc::c_int);
        LTP_shp_buf_idx = LTP_shp_buf_idx - 1 as libc::c_int
            & 512 as libc::c_int - 1 as libc::c_int;
        *LTP_shp_buf
            .offset(
                LTP_shp_buf_idx as isize,
            ) = (if (if 12 as libc::c_int == 1 as libc::c_int {
            (sLF_MA_shp_Q12 >> 1 as libc::c_int) + (sLF_MA_shp_Q12 & 1 as libc::c_int)
        } else {
            (sLF_MA_shp_Q12 >> 12 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                >> 1 as libc::c_int
        }) > 0x7fff as libc::c_int
        {
            0x7fff as libc::c_int
        } else if (if 12 as libc::c_int == 1 as libc::c_int {
            (sLF_MA_shp_Q12 >> 1 as libc::c_int) + (sLF_MA_shp_Q12 & 1 as libc::c_int)
        } else {
            (sLF_MA_shp_Q12 >> 12 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                >> 1 as libc::c_int
        }) < 0x8000 as libc::c_int as libc::c_short as libc::c_int
        {
            0x8000 as libc::c_int as libc::c_short as libc::c_int
        } else if 12 as libc::c_int == 1 as libc::c_int {
            (sLF_MA_shp_Q12 >> 1 as libc::c_int) + (sLF_MA_shp_Q12 & 1 as libc::c_int)
        } else {
            (sLF_MA_shp_Q12 >> 12 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                >> 1 as libc::c_int
        }) as libc::c_short;
        *xw
            .offset(
                i as isize,
            ) = (if (if 12 as libc::c_int == 1 as libc::c_int {
            (sLF_MA_shp_Q12 - n_LTP_Q12 >> 1 as libc::c_int)
                + (sLF_MA_shp_Q12 - n_LTP_Q12 & 1 as libc::c_int)
        } else {
            (sLF_MA_shp_Q12 - n_LTP_Q12 >> 12 as libc::c_int - 1 as libc::c_int)
                + 1 as libc::c_int >> 1 as libc::c_int
        }) > 0x7fff as libc::c_int
        {
            0x7fff as libc::c_int
        } else if (if 12 as libc::c_int == 1 as libc::c_int {
            (sLF_MA_shp_Q12 - n_LTP_Q12 >> 1 as libc::c_int)
                + (sLF_MA_shp_Q12 - n_LTP_Q12 & 1 as libc::c_int)
        } else {
            (sLF_MA_shp_Q12 - n_LTP_Q12 >> 12 as libc::c_int - 1 as libc::c_int)
                + 1 as libc::c_int >> 1 as libc::c_int
        }) < 0x8000 as libc::c_int as libc::c_short as libc::c_int
        {
            0x8000 as libc::c_int as libc::c_short as libc::c_int
        } else if 12 as libc::c_int == 1 as libc::c_int {
            (sLF_MA_shp_Q12 - n_LTP_Q12 >> 1 as libc::c_int)
                + (sLF_MA_shp_Q12 - n_LTP_Q12 & 1 as libc::c_int)
        } else {
            (sLF_MA_shp_Q12 - n_LTP_Q12 >> 12 as libc::c_int - 1 as libc::c_int)
                + 1 as libc::c_int >> 1 as libc::c_int
        }) as libc::c_short;
        i += 1;
        i;
    }
    (*P).sLF_AR_shp_Q12 = sLF_AR_shp_Q12;
    (*P).sLF_MA_shp_Q12 = sLF_MA_shp_Q12;
    (*P).sLTP_shp_buf_idx = LTP_shp_buf_idx;
}
