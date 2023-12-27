#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn SKP_Silk_biquad_alt(
        in_0: *const libc::c_short,
        B_Q28: *const libc::c_int,
        A_Q28: *const libc::c_int,
        S: *mut libc::c_int,
        out: *mut libc::c_short,
        len: libc::c_int,
    );
    fn SKP_Silk_lin2log(inLin: libc::c_int) -> libc::c_int;
    fn SKP_Silk_log2lin(inLog_Q7: libc::c_int) -> libc::c_int;
}
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
pub unsafe extern "C" fn SKP_Silk_HP_variable_cutoff_FIX(
    mut psEnc: *mut SKP_Silk_encoder_state_FIX,
    mut psEncCtrl: *mut SKP_Silk_encoder_control_FIX,
    mut out: *mut libc::c_short,
    mut in_0: *const libc::c_short,
) {
    let mut quality_Q15: libc::c_int = 0;
    let mut B_Q28: [libc::c_int; 3] = [0; 3];
    let mut A_Q28: [libc::c_int; 2] = [0; 2];
    let mut Fc_Q19: libc::c_int = 0;
    let mut r_Q28: libc::c_int = 0;
    let mut r_Q22: libc::c_int = 0;
    let mut pitch_freq_Hz_Q16: libc::c_int = 0;
    let mut pitch_freq_log_Q7: libc::c_int = 0;
    let mut delta_freq_Q7: libc::c_int = 0;
    if (*psEnc).sCmn.prev_sigtype == 0 as libc::c_int {
        pitch_freq_Hz_Q16 = (((*psEnc).sCmn.fs_kHz * 1000 as libc::c_int)
            << 16 as libc::c_int) / (*psEnc).sCmn.prevLag;
        pitch_freq_log_Q7 = SKP_Silk_lin2log(pitch_freq_Hz_Q16)
            - ((16 as libc::c_int) << 7 as libc::c_int);
        quality_Q15 = (*psEncCtrl).input_quality_bands_Q15[0 as libc::c_int as usize];
        pitch_freq_log_Q7 = pitch_freq_log_Q7
            - (((quality_Q15 << 2 as libc::c_int >> 16 as libc::c_int)
                * quality_Q15 as libc::c_short as libc::c_int
                + ((quality_Q15 << 2 as libc::c_int & 0xffff as libc::c_int)
                    * quality_Q15 as libc::c_short as libc::c_int >> 16 as libc::c_int)
                >> 16 as libc::c_int)
                * (pitch_freq_log_Q7 - 809 as libc::c_int) as libc::c_short
                    as libc::c_int
                + (((quality_Q15 << 2 as libc::c_int >> 16 as libc::c_int)
                    * quality_Q15 as libc::c_short as libc::c_int
                    + ((quality_Q15 << 2 as libc::c_int & 0xffff as libc::c_int)
                        * quality_Q15 as libc::c_short as libc::c_int
                        >> 16 as libc::c_int) & 0xffff as libc::c_int)
                    * (pitch_freq_log_Q7 - 809 as libc::c_int) as libc::c_short
                        as libc::c_int >> 16 as libc::c_int));
        pitch_freq_log_Q7 = pitch_freq_log_Q7
            + ((0.6f64
                * ((1 as libc::c_int as int64_t) << 15 as libc::c_int) as libc::c_double
                + 0.5f64) as libc::c_int - quality_Q15 >> 9 as libc::c_int);
        delta_freq_Q7 = pitch_freq_log_Q7
            - ((*psEnc).variable_HP_smth1_Q15 >> 8 as libc::c_int);
        if delta_freq_Q7 < 0 as libc::c_int {
            delta_freq_Q7 = delta_freq_Q7 * 3 as libc::c_int;
        }
        delta_freq_Q7 = if -(((0.4f32
            * ((1 as libc::c_int as int64_t) << 7 as libc::c_int) as libc::c_float)
            as libc::c_double + 0.5f64) as libc::c_int)
            > ((0.4f32
                * ((1 as libc::c_int as int64_t) << 7 as libc::c_int) as libc::c_float)
                as libc::c_double + 0.5f64) as libc::c_int
        {
            if delta_freq_Q7
                > -(((0.4f32
                    * ((1 as libc::c_int as int64_t) << 7 as libc::c_int)
                        as libc::c_float) as libc::c_double + 0.5f64) as libc::c_int)
            {
                -(((0.4f32
                    * ((1 as libc::c_int as int64_t) << 7 as libc::c_int)
                        as libc::c_float) as libc::c_double + 0.5f64) as libc::c_int)
            } else if delta_freq_Q7
                < ((0.4f32
                    * ((1 as libc::c_int as int64_t) << 7 as libc::c_int)
                        as libc::c_float) as libc::c_double + 0.5f64) as libc::c_int
            {
                ((0.4f32
                    * ((1 as libc::c_int as int64_t) << 7 as libc::c_int)
                        as libc::c_float) as libc::c_double + 0.5f64) as libc::c_int
            } else {
                delta_freq_Q7
            }
        } else if delta_freq_Q7
            > ((0.4f32
                * ((1 as libc::c_int as int64_t) << 7 as libc::c_int) as libc::c_float)
                as libc::c_double + 0.5f64) as libc::c_int
        {
            ((0.4f32
                * ((1 as libc::c_int as int64_t) << 7 as libc::c_int) as libc::c_float)
                as libc::c_double + 0.5f64) as libc::c_int
        } else if delta_freq_Q7
            < -(((0.4f32
                * ((1 as libc::c_int as int64_t) << 7 as libc::c_int) as libc::c_float)
                as libc::c_double + 0.5f64) as libc::c_int)
        {
            -(((0.4f32
                * ((1 as libc::c_int as int64_t) << 7 as libc::c_int) as libc::c_float)
                as libc::c_double + 0.5f64) as libc::c_int)
        } else {
            delta_freq_Q7
        };
        (*psEnc)
            .variable_HP_smth1_Q15 = (*psEnc).variable_HP_smth1_Q15
            + ((((*psEnc).speech_activity_Q8 << 1 as libc::c_int) * delta_freq_Q7
                >> 16 as libc::c_int)
                * ((0.1f32
                    * ((1 as libc::c_int as int64_t) << 16 as libc::c_int)
                        as libc::c_float) as libc::c_double + 0.5f64) as libc::c_int
                    as libc::c_short as libc::c_int
                + ((((*psEnc).speech_activity_Q8 << 1 as libc::c_int) * delta_freq_Q7
                    & 0xffff as libc::c_int)
                    * ((0.1f32
                        * ((1 as libc::c_int as int64_t) << 16 as libc::c_int)
                            as libc::c_float) as libc::c_double + 0.5f64) as libc::c_int
                        as libc::c_short as libc::c_int >> 16 as libc::c_int));
    }
    (*psEnc)
        .variable_HP_smth2_Q15 = (*psEnc).variable_HP_smth2_Q15
        + (((*psEnc).variable_HP_smth1_Q15 - (*psEnc).variable_HP_smth2_Q15
            >> 16 as libc::c_int)
            * ((0.015f32
                * ((1 as libc::c_int as int64_t) << 16 as libc::c_int) as libc::c_float)
                as libc::c_double + 0.5f64) as libc::c_int as libc::c_short
                as libc::c_int
            + (((*psEnc).variable_HP_smth1_Q15 - (*psEnc).variable_HP_smth2_Q15
                & 0xffff as libc::c_int)
                * ((0.015f32
                    * ((1 as libc::c_int as int64_t) << 16 as libc::c_int)
                        as libc::c_float) as libc::c_double + 0.5f64) as libc::c_int
                    as libc::c_short as libc::c_int >> 16 as libc::c_int));
    (*psEncCtrl)
        .pitch_freq_low_Hz = SKP_Silk_log2lin(
        (*psEnc).variable_HP_smth2_Q15 >> 8 as libc::c_int,
    );
    (*psEncCtrl)
        .pitch_freq_low_Hz = if ((80.0f32
        * ((1 as libc::c_int as int64_t) << 0 as libc::c_int) as libc::c_float)
        as libc::c_double + 0.5f64) as libc::c_int
        > ((150.0f32
            * ((1 as libc::c_int as int64_t) << 0 as libc::c_int) as libc::c_float)
            as libc::c_double + 0.5f64) as libc::c_int
    {
        if (*psEncCtrl).pitch_freq_low_Hz
            > ((80.0f32
                * ((1 as libc::c_int as int64_t) << 0 as libc::c_int) as libc::c_float)
                as libc::c_double + 0.5f64) as libc::c_int
        {
            ((80.0f32
                * ((1 as libc::c_int as int64_t) << 0 as libc::c_int) as libc::c_float)
                as libc::c_double + 0.5f64) as libc::c_int
        } else if (*psEncCtrl).pitch_freq_low_Hz
            < ((150.0f32
                * ((1 as libc::c_int as int64_t) << 0 as libc::c_int) as libc::c_float)
                as libc::c_double + 0.5f64) as libc::c_int
        {
            ((150.0f32
                * ((1 as libc::c_int as int64_t) << 0 as libc::c_int) as libc::c_float)
                as libc::c_double + 0.5f64) as libc::c_int
        } else {
            (*psEncCtrl).pitch_freq_low_Hz
        }
    } else if (*psEncCtrl).pitch_freq_low_Hz
        > ((150.0f32
            * ((1 as libc::c_int as int64_t) << 0 as libc::c_int) as libc::c_float)
            as libc::c_double + 0.5f64) as libc::c_int
    {
        ((150.0f32
            * ((1 as libc::c_int as int64_t) << 0 as libc::c_int) as libc::c_float)
            as libc::c_double + 0.5f64) as libc::c_int
    } else if (*psEncCtrl).pitch_freq_low_Hz
        < ((80.0f32
            * ((1 as libc::c_int as int64_t) << 0 as libc::c_int) as libc::c_float)
            as libc::c_double + 0.5f64) as libc::c_int
    {
        ((80.0f32 * ((1 as libc::c_int as int64_t) << 0 as libc::c_int) as libc::c_float)
            as libc::c_double + 0.5f64) as libc::c_int
    } else {
        (*psEncCtrl).pitch_freq_low_Hz
    };
    Fc_Q19 = 1482 as libc::c_int as libc::c_short as libc::c_int
        * (*psEncCtrl).pitch_freq_low_Hz as libc::c_short as libc::c_int
        / (*psEnc).sCmn.fs_kHz;
    r_Q28 = (1.0f64
        * ((1 as libc::c_int as int64_t) << 28 as libc::c_int) as libc::c_double
        + 0.5f64) as libc::c_int
        - (0.92f64
            * ((1 as libc::c_int as int64_t) << 9 as libc::c_int) as libc::c_double
            + 0.5f64) as libc::c_int * Fc_Q19;
    B_Q28[0 as libc::c_int as usize] = r_Q28;
    B_Q28[1 as libc::c_int as usize] = -r_Q28 << 1 as libc::c_int;
    B_Q28[2 as libc::c_int as usize] = r_Q28;
    r_Q22 = r_Q28 >> 6 as libc::c_int;
    A_Q28[0 as libc::c_int
        as usize] = (r_Q22 >> 16 as libc::c_int)
        * ((Fc_Q19 >> 16 as libc::c_int) * Fc_Q19 as libc::c_short as libc::c_int
            + ((Fc_Q19 & 0xffff as libc::c_int) * Fc_Q19 as libc::c_short as libc::c_int
                >> 16 as libc::c_int)
            + Fc_Q19
                * (if 16 as libc::c_int == 1 as libc::c_int {
                    (Fc_Q19 >> 1 as libc::c_int) + (Fc_Q19 & 1 as libc::c_int)
                } else {
                    (Fc_Q19 >> 16 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                        >> 1 as libc::c_int
                })
            - (2.0f64
                * ((1 as libc::c_int as int64_t) << 22 as libc::c_int) as libc::c_double
                + 0.5f64) as libc::c_int) as libc::c_short as libc::c_int
        + ((r_Q22 & 0xffff as libc::c_int)
            * ((Fc_Q19 >> 16 as libc::c_int) * Fc_Q19 as libc::c_short as libc::c_int
                + ((Fc_Q19 & 0xffff as libc::c_int)
                    * Fc_Q19 as libc::c_short as libc::c_int >> 16 as libc::c_int)
                + Fc_Q19
                    * (if 16 as libc::c_int == 1 as libc::c_int {
                        (Fc_Q19 >> 1 as libc::c_int) + (Fc_Q19 & 1 as libc::c_int)
                    } else {
                        (Fc_Q19 >> 16 as libc::c_int - 1 as libc::c_int)
                            + 1 as libc::c_int >> 1 as libc::c_int
                    })
                - (2.0f64
                    * ((1 as libc::c_int as int64_t) << 22 as libc::c_int)
                        as libc::c_double + 0.5f64) as libc::c_int) as libc::c_short
                as libc::c_int >> 16 as libc::c_int)
        + r_Q22
            * (if 16 as libc::c_int == 1 as libc::c_int {
                ((Fc_Q19 >> 16 as libc::c_int) * Fc_Q19 as libc::c_short as libc::c_int
                    + ((Fc_Q19 & 0xffff as libc::c_int)
                        * Fc_Q19 as libc::c_short as libc::c_int >> 16 as libc::c_int)
                    + Fc_Q19
                        * (if 16 as libc::c_int == 1 as libc::c_int {
                            (Fc_Q19 >> 1 as libc::c_int) + (Fc_Q19 & 1 as libc::c_int)
                        } else {
                            (Fc_Q19 >> 16 as libc::c_int - 1 as libc::c_int)
                                + 1 as libc::c_int >> 1 as libc::c_int
                        })
                    - (2.0f64
                        * ((1 as libc::c_int as int64_t) << 22 as libc::c_int)
                            as libc::c_double + 0.5f64) as libc::c_int
                    >> 1 as libc::c_int)
                    + ((Fc_Q19 >> 16 as libc::c_int)
                        * Fc_Q19 as libc::c_short as libc::c_int
                        + ((Fc_Q19 & 0xffff as libc::c_int)
                            * Fc_Q19 as libc::c_short as libc::c_int
                            >> 16 as libc::c_int)
                        + Fc_Q19
                            * (if 16 as libc::c_int == 1 as libc::c_int {
                                (Fc_Q19 >> 1 as libc::c_int) + (Fc_Q19 & 1 as libc::c_int)
                            } else {
                                (Fc_Q19 >> 16 as libc::c_int - 1 as libc::c_int)
                                    + 1 as libc::c_int >> 1 as libc::c_int
                            })
                        - (2.0f64
                            * ((1 as libc::c_int as int64_t) << 22 as libc::c_int)
                                as libc::c_double + 0.5f64) as libc::c_int
                        & 1 as libc::c_int)
            } else {
                ((Fc_Q19 >> 16 as libc::c_int) * Fc_Q19 as libc::c_short as libc::c_int
                    + ((Fc_Q19 & 0xffff as libc::c_int)
                        * Fc_Q19 as libc::c_short as libc::c_int >> 16 as libc::c_int)
                    + Fc_Q19
                        * (if 16 as libc::c_int == 1 as libc::c_int {
                            (Fc_Q19 >> 1 as libc::c_int) + (Fc_Q19 & 1 as libc::c_int)
                        } else {
                            (Fc_Q19 >> 16 as libc::c_int - 1 as libc::c_int)
                                + 1 as libc::c_int >> 1 as libc::c_int
                        })
                    - (2.0f64
                        * ((1 as libc::c_int as int64_t) << 22 as libc::c_int)
                            as libc::c_double + 0.5f64) as libc::c_int
                    >> 16 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                    >> 1 as libc::c_int
            });
    A_Q28[1 as libc::c_int
        as usize] = (r_Q22 >> 16 as libc::c_int) * r_Q22 as libc::c_short as libc::c_int
        + ((r_Q22 & 0xffff as libc::c_int) * r_Q22 as libc::c_short as libc::c_int
            >> 16 as libc::c_int)
        + r_Q22
            * (if 16 as libc::c_int == 1 as libc::c_int {
                (r_Q22 >> 1 as libc::c_int) + (r_Q22 & 1 as libc::c_int)
            } else {
                (r_Q22 >> 16 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                    >> 1 as libc::c_int
            });
    SKP_Silk_biquad_alt(
        in_0,
        B_Q28.as_mut_ptr(),
        A_Q28.as_mut_ptr(),
        ((*psEnc).sCmn.In_HP_State).as_mut_ptr(),
        out,
        (*psEnc).sCmn.frame_length,
    );
}
