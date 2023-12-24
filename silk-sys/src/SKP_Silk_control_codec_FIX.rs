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
    fn SKP_Silk_resampler_init(
        S: *mut SKP_Silk_resampler_state_struct,
        Fs_Hz_in: libc::c_int,
        Fs_Hz_out: libc::c_int,
    ) -> libc::c_int;
    fn SKP_Silk_resampler(
        S: *mut SKP_Silk_resampler_state_struct,
        out: *mut libc::c_short,
        in_0: *const libc::c_short,
        inLen: libc::c_int,
    ) -> libc::c_int;
    static SKP_Silk_NLSF_CB0_16: SKP_Silk_NLSF_CB_struct;
    static SKP_Silk_NLSF_CB1_16: SKP_Silk_NLSF_CB_struct;
    static SKP_Silk_NLSF_CB0_10: SKP_Silk_NLSF_CB_struct;
    static SKP_Silk_NLSF_CB1_10: SKP_Silk_NLSF_CB_struct;
    static TargetRate_table_NB: [libc::c_int; 8];
    static TargetRate_table_MB: [libc::c_int; 8];
    static TargetRate_table_WB: [libc::c_int; 8];
    static TargetRate_table_SWB: [libc::c_int; 8];
    static SNR_table_Q1: [libc::c_int; 8];
    fn SKP_Silk_control_audio_bandwidth(
        psEncC: *mut SKP_Silk_encoder_state,
        TargetRate_bps: libc::c_int,
    ) -> libc::c_int;
    fn SKP_Silk_LBRR_reset(psEncC: *mut SKP_Silk_encoder_state);
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
#[inline]
unsafe extern "C" fn SKP_min_int(mut a: libc::c_int, mut b: libc::c_int) -> libc::c_int {
    return if a < b { a } else { b };
}
#[inline]
unsafe extern "C" fn SKP_max_int(mut a: libc::c_int, mut b: libc::c_int) -> libc::c_int {
    return if a > b { a } else { b };
}
#[inline]
unsafe extern "C" fn SKP_Silk_setup_complexity(
    mut psEncC: *mut SKP_Silk_encoder_state,
    mut Complexity: libc::c_int,
) -> libc::c_int {
    let mut ret: libc::c_int = 0 as libc::c_int;
    if 0 as libc::c_int != 0 && Complexity != 0 as libc::c_int {
        ret = -(6 as libc::c_int);
    }
    if Complexity == 0 as libc::c_int || 0 as libc::c_int != 0 {
        (*psEncC).Complexity = 0 as libc::c_int;
        (*psEncC).pitchEstimationComplexity = 0 as libc::c_int;
        (*psEncC)
            .pitchEstimationThreshold_Q16 = ((0.8f32
            * ((1 as libc::c_int as int64_t) << 16 as libc::c_int) as libc::c_float)
            as libc::c_double + 0.5f64) as libc::c_int;
        (*psEncC).pitchEstimationLPCOrder = 6 as libc::c_int;
        (*psEncC).shapingLPCOrder = 8 as libc::c_int;
        (*psEncC).la_shape = 3 as libc::c_int * (*psEncC).fs_kHz;
        (*psEncC).nStatesDelayedDecision = 1 as libc::c_int;
        (*psEncC).useInterpolatedNLSFs = 0 as libc::c_int;
        (*psEncC).LTPQuantLowComplexity = 1 as libc::c_int;
        (*psEncC).NLSF_MSVQ_Survivors = 2 as libc::c_int;
        (*psEncC).warping_Q16 = 0 as libc::c_int;
    } else if Complexity == 1 as libc::c_int {
        (*psEncC).Complexity = 1 as libc::c_int;
        (*psEncC).pitchEstimationComplexity = 1 as libc::c_int;
        (*psEncC)
            .pitchEstimationThreshold_Q16 = ((0.75f32
            * ((1 as libc::c_int as int64_t) << 16 as libc::c_int) as libc::c_float)
            as libc::c_double + 0.5f64) as libc::c_int;
        (*psEncC).pitchEstimationLPCOrder = 12 as libc::c_int;
        (*psEncC).shapingLPCOrder = 12 as libc::c_int;
        (*psEncC).la_shape = 5 as libc::c_int * (*psEncC).fs_kHz;
        (*psEncC).nStatesDelayedDecision = 2 as libc::c_int;
        (*psEncC).useInterpolatedNLSFs = 0 as libc::c_int;
        (*psEncC).LTPQuantLowComplexity = 0 as libc::c_int;
        (*psEncC).NLSF_MSVQ_Survivors = 4 as libc::c_int;
        (*psEncC)
            .warping_Q16 = (*psEncC).fs_kHz
            * ((0.015f32
                * ((1 as libc::c_int as int64_t) << 16 as libc::c_int) as libc::c_float)
                as libc::c_double + 0.5f64) as libc::c_int;
    } else if Complexity == 2 as libc::c_int {
        (*psEncC).Complexity = 2 as libc::c_int;
        (*psEncC).pitchEstimationComplexity = 2 as libc::c_int;
        (*psEncC)
            .pitchEstimationThreshold_Q16 = ((0.7f32
            * ((1 as libc::c_int as int64_t) << 16 as libc::c_int) as libc::c_float)
            as libc::c_double + 0.5f64) as libc::c_int;
        (*psEncC).pitchEstimationLPCOrder = 16 as libc::c_int;
        (*psEncC).shapingLPCOrder = 16 as libc::c_int;
        (*psEncC).la_shape = 5 as libc::c_int * (*psEncC).fs_kHz;
        (*psEncC).nStatesDelayedDecision = 4 as libc::c_int;
        (*psEncC).useInterpolatedNLSFs = 1 as libc::c_int;
        (*psEncC).LTPQuantLowComplexity = 0 as libc::c_int;
        (*psEncC).NLSF_MSVQ_Survivors = 16 as libc::c_int;
        (*psEncC)
            .warping_Q16 = (*psEncC).fs_kHz
            * ((0.015f32
                * ((1 as libc::c_int as int64_t) << 16 as libc::c_int) as libc::c_float)
                as libc::c_double + 0.5f64) as libc::c_int;
    } else {
        ret = -(6 as libc::c_int);
    }
    (*psEncC)
        .pitchEstimationLPCOrder = SKP_min_int(
        (*psEncC).pitchEstimationLPCOrder,
        (*psEncC).predictLPCOrder,
    );
    (*psEncC)
        .shapeWinLength = 5 as libc::c_int * (*psEncC).fs_kHz
        + 2 as libc::c_int * (*psEncC).la_shape;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_control_encoder_FIX(
    mut psEnc: *mut SKP_Silk_encoder_state_FIX,
    PacketSize_ms: libc::c_int,
    TargetRate_bps: libc::c_int,
    PacketLoss_perc: libc::c_int,
    DTX_enabled: libc::c_int,
    Complexity: libc::c_int,
) -> libc::c_int {
    let mut fs_kHz: libc::c_int = 0;
    let mut ret: libc::c_int = 0 as libc::c_int;
    if (*psEnc).sCmn.controlled_since_last_payload != 0 as libc::c_int {
        if (*psEnc).sCmn.API_fs_Hz != (*psEnc).sCmn.prev_API_fs_Hz
            && (*psEnc).sCmn.fs_kHz > 0 as libc::c_int
        {
            ret += SKP_Silk_setup_resamplers_FIX(psEnc, (*psEnc).sCmn.fs_kHz);
        }
        return ret;
    }
    fs_kHz = SKP_Silk_control_audio_bandwidth(&mut (*psEnc).sCmn, TargetRate_bps);
    ret += SKP_Silk_setup_resamplers_FIX(psEnc, fs_kHz);
    ret += SKP_Silk_setup_packetsize_FIX(psEnc, PacketSize_ms);
    ret += SKP_Silk_setup_fs_FIX(psEnc, fs_kHz);
    ret += SKP_Silk_setup_complexity(&mut (*psEnc).sCmn, Complexity);
    ret += SKP_Silk_setup_rate_FIX(psEnc, TargetRate_bps);
    if PacketLoss_perc < 0 as libc::c_int || PacketLoss_perc > 100 as libc::c_int {
        ret = -(5 as libc::c_int);
    }
    (*psEnc).sCmn.PacketLoss_perc = PacketLoss_perc;
    ret += SKP_Silk_setup_LBRR_FIX(psEnc);
    if DTX_enabled < 0 as libc::c_int || DTX_enabled > 1 as libc::c_int {
        ret = -(8 as libc::c_int);
    }
    (*psEnc).sCmn.useDTX = DTX_enabled;
    (*psEnc).sCmn.controlled_since_last_payload = 1 as libc::c_int;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_LBRR_ctrl_FIX(
    mut psEnc: *mut SKP_Silk_encoder_state_FIX,
    mut psEncCtrlC: *mut SKP_Silk_encoder_control,
) {
    let mut LBRR_usage: libc::c_int = 0;
    if (*psEnc).sCmn.LBRR_enabled != 0 {
        LBRR_usage = 0 as libc::c_int;
        if (*psEnc).speech_activity_Q8
            > ((0.5f32
                * ((1 as libc::c_int as int64_t) << 8 as libc::c_int) as libc::c_float)
                as libc::c_double + 0.5f64) as libc::c_int
            && (*psEnc).sCmn.PacketLoss_perc > 1 as libc::c_int
        {
            LBRR_usage = 1 as libc::c_int;
        }
        (*psEncCtrlC).LBRR_usage = LBRR_usage;
    } else {
        (*psEncCtrlC).LBRR_usage = 0 as libc::c_int;
    };
}
#[inline]
unsafe extern "C" fn SKP_Silk_setup_resamplers_FIX(
    mut psEnc: *mut SKP_Silk_encoder_state_FIX,
    mut fs_kHz: libc::c_int,
) -> libc::c_int {
    let mut ret: libc::c_int = 0 as libc::c_int;
    if (*psEnc).sCmn.fs_kHz != fs_kHz
        || (*psEnc).sCmn.prev_API_fs_Hz != (*psEnc).sCmn.API_fs_Hz
    {
        if (*psEnc).sCmn.fs_kHz == 0 as libc::c_int {
            ret
                += SKP_Silk_resampler_init(
                    &mut (*psEnc).sCmn.resampler_state,
                    (*psEnc).sCmn.API_fs_Hz,
                    fs_kHz * 1000 as libc::c_int,
                );
        } else {
            let mut x_buf_API_fs_Hz: [libc::c_short; 6480] = [0; 6480];
            let mut nSamples_temp: libc::c_int = ((*psEnc).sCmn.frame_length
                << 1 as libc::c_int) + 5 as libc::c_int * (*psEnc).sCmn.fs_kHz;
            if (fs_kHz as libc::c_short as libc::c_int
                * 1000 as libc::c_int as libc::c_short as libc::c_int)
                < (*psEnc).sCmn.API_fs_Hz && (*psEnc).sCmn.fs_kHz != 0 as libc::c_int
            {
                let mut temp_resampler_state: SKP_Silk_resampler_state_struct = _SKP_Silk_resampler_state_struct {
                    sIIR: [0; 6],
                    sFIR: [0; 16],
                    sDown2: [0; 2],
                    resampler_function: None,
                    up2_function: None,
                    batchSize: 0,
                    invRatio_Q16: 0,
                    FIR_Fracs: 0,
                    input2x: 0,
                    Coefs: 0 as *const libc::c_short,
                    sDownPre: [0; 2],
                    sUpPost: [0; 2],
                    down_pre_function: None,
                    up_post_function: None,
                    batchSizePrePost: 0,
                    ratio_Q16: 0,
                    nPreDownsamplers: 0,
                    nPostUpsamplers: 0,
                    magic_number: 0,
                };
                ret
                    += SKP_Silk_resampler_init(
                        &mut temp_resampler_state,
                        (*psEnc).sCmn.fs_kHz as libc::c_short as libc::c_int
                            * 1000 as libc::c_int as libc::c_short as libc::c_int,
                        (*psEnc).sCmn.API_fs_Hz,
                    );
                ret
                    += SKP_Silk_resampler(
                        &mut temp_resampler_state,
                        x_buf_API_fs_Hz.as_mut_ptr(),
                        ((*psEnc).x_buf).as_mut_ptr() as *const libc::c_short,
                        nSamples_temp,
                    );
                nSamples_temp = nSamples_temp * (*psEnc).sCmn.API_fs_Hz
                    / ((*psEnc).sCmn.fs_kHz as libc::c_short as libc::c_int
                        * 1000 as libc::c_int as libc::c_short as libc::c_int);
                ret
                    += SKP_Silk_resampler_init(
                        &mut (*psEnc).sCmn.resampler_state,
                        (*psEnc).sCmn.API_fs_Hz,
                        fs_kHz as libc::c_short as libc::c_int
                            * 1000 as libc::c_int as libc::c_short as libc::c_int,
                    );
            } else {
                memcpy(
                    x_buf_API_fs_Hz.as_mut_ptr() as *mut libc::c_void,
                    ((*psEnc).x_buf).as_mut_ptr() as *const libc::c_void,
                    (nSamples_temp as libc::c_ulong)
                        .wrapping_mul(
                            ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                        ),
                );
            }
            if 1000 as libc::c_int * fs_kHz != (*psEnc).sCmn.API_fs_Hz {
                ret
                    += SKP_Silk_resampler(
                        &mut (*psEnc).sCmn.resampler_state,
                        ((*psEnc).x_buf).as_mut_ptr(),
                        x_buf_API_fs_Hz.as_mut_ptr() as *const libc::c_short,
                        nSamples_temp,
                    );
            }
        }
    }
    (*psEnc).sCmn.prev_API_fs_Hz = (*psEnc).sCmn.API_fs_Hz;
    return ret;
}
#[inline]
unsafe extern "C" fn SKP_Silk_setup_packetsize_FIX(
    mut psEnc: *mut SKP_Silk_encoder_state_FIX,
    mut PacketSize_ms: libc::c_int,
) -> libc::c_int {
    let mut ret: libc::c_int = 0 as libc::c_int;
    if PacketSize_ms != 20 as libc::c_int && PacketSize_ms != 40 as libc::c_int
        && PacketSize_ms != 60 as libc::c_int && PacketSize_ms != 80 as libc::c_int
        && PacketSize_ms != 100 as libc::c_int
    {
        ret = -(3 as libc::c_int);
    } else if PacketSize_ms != (*psEnc).sCmn.PacketSize_ms {
        (*psEnc).sCmn.PacketSize_ms = PacketSize_ms;
        SKP_Silk_LBRR_reset(&mut (*psEnc).sCmn);
    }
    return ret;
}
#[inline]
unsafe extern "C" fn SKP_Silk_setup_fs_FIX(
    mut psEnc: *mut SKP_Silk_encoder_state_FIX,
    mut fs_kHz: libc::c_int,
) -> libc::c_int {
    let mut ret: libc::c_int = 0 as libc::c_int;
    if (*psEnc).sCmn.fs_kHz != fs_kHz {
        memset(
            &mut (*psEnc).sShape as *mut SKP_Silk_shape_state_FIX as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<SKP_Silk_shape_state_FIX>() as libc::c_ulong,
        );
        memset(
            &mut (*psEnc).sPrefilt as *mut SKP_Silk_prefilter_state_FIX
                as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<SKP_Silk_prefilter_state_FIX>() as libc::c_ulong,
        );
        memset(
            &mut (*psEnc).sPred as *mut SKP_Silk_predict_state_FIX as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<SKP_Silk_predict_state_FIX>() as libc::c_ulong,
        );
        memset(
            &mut (*psEnc).sCmn.sNSQ as *mut SKP_Silk_nsq_state as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<SKP_Silk_nsq_state>() as libc::c_ulong,
        );
        memset(
            ((*psEnc).sCmn.sNSQ_LBRR.xq).as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            ((2 as libc::c_int * (20 as libc::c_int * 24 as libc::c_int))
                as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_short>() as libc::c_ulong),
        );
        memset(
            ((*psEnc).sCmn.LBRR_buffer).as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(
                    ::core::mem::size_of::<SKP_SILK_LBRR_struct>() as libc::c_ulong,
                ),
        );
        memset(
            ((*psEnc).sCmn.sLP.In_LP_State).as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
        );
        if (*psEnc).sCmn.sLP.mode == 1 as libc::c_int {
            (*psEnc).sCmn.sLP.transition_frame_no = 1 as libc::c_int;
        } else {
            (*psEnc).sCmn.sLP.transition_frame_no = 0 as libc::c_int;
        }
        (*psEnc).sCmn.inputBufIx = 0 as libc::c_int;
        (*psEnc).sCmn.nFramesInPayloadBuf = 0 as libc::c_int;
        (*psEnc).sCmn.nBytesInPayloadBuf = 0 as libc::c_int;
        (*psEnc).sCmn.oldest_LBRR_idx = 0 as libc::c_int;
        (*psEnc).sCmn.TargetRate_bps = 0 as libc::c_int;
        memset(
            ((*psEnc).sPred.prev_NLSFq_Q15).as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            (16 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
        );
        (*psEnc).sCmn.prevLag = 100 as libc::c_int;
        (*psEnc).sCmn.prev_sigtype = 1 as libc::c_int;
        (*psEnc).sCmn.first_frame_after_reset = 1 as libc::c_int;
        (*psEnc).sPrefilt.lagPrev = 100 as libc::c_int;
        (*psEnc).sShape.LastGainIndex = 1 as libc::c_int;
        (*psEnc).sCmn.sNSQ.lagPrev = 100 as libc::c_int;
        (*psEnc).sCmn.sNSQ.prev_inv_gain_Q16 = 65536 as libc::c_int;
        (*psEnc).sCmn.sNSQ_LBRR.prev_inv_gain_Q16 = 65536 as libc::c_int;
        (*psEnc).sCmn.fs_kHz = fs_kHz;
        if (*psEnc).sCmn.fs_kHz == 8 as libc::c_int {
            (*psEnc).sCmn.predictLPCOrder = 10 as libc::c_int;
            (*psEnc).sCmn.psNLSF_CB[0 as libc::c_int as usize] = &SKP_Silk_NLSF_CB0_10;
            (*psEnc).sCmn.psNLSF_CB[1 as libc::c_int as usize] = &SKP_Silk_NLSF_CB1_10;
        } else {
            (*psEnc).sCmn.predictLPCOrder = 16 as libc::c_int;
            (*psEnc).sCmn.psNLSF_CB[0 as libc::c_int as usize] = &SKP_Silk_NLSF_CB0_16;
            (*psEnc).sCmn.psNLSF_CB[1 as libc::c_int as usize] = &SKP_Silk_NLSF_CB1_16;
        }
        (*psEnc)
            .sCmn
            .frame_length = 20 as libc::c_int as libc::c_short as libc::c_int
            * fs_kHz as libc::c_short as libc::c_int;
        (*psEnc).sCmn.subfr_length = (*psEnc).sCmn.frame_length / 4 as libc::c_int;
        (*psEnc)
            .sCmn
            .la_pitch = 2 as libc::c_int as libc::c_short as libc::c_int
            * fs_kHz as libc::c_short as libc::c_int;
        (*psEnc)
            .sPred
            .min_pitch_lag = 3 as libc::c_int as libc::c_short as libc::c_int
            * fs_kHz as libc::c_short as libc::c_int;
        (*psEnc)
            .sPred
            .max_pitch_lag = 18 as libc::c_int as libc::c_short as libc::c_int
            * fs_kHz as libc::c_short as libc::c_int;
        (*psEnc)
            .sPred
            .pitch_LPC_win_length = (20 as libc::c_int
            + ((2 as libc::c_int) << 1 as libc::c_int)) as libc::c_short as libc::c_int
            * fs_kHz as libc::c_short as libc::c_int;
        if (*psEnc).sCmn.fs_kHz == 24 as libc::c_int {
            (*psEnc)
                .mu_LTP_Q8 = ((0.016f32
                * ((1 as libc::c_int as int64_t) << 8 as libc::c_int) as libc::c_float)
                as libc::c_double + 0.5f64) as libc::c_int;
            (*psEnc).sCmn.bitrate_threshold_up = 0x7fffffff as libc::c_int;
            (*psEnc).sCmn.bitrate_threshold_down = 25000 as libc::c_int;
        } else if (*psEnc).sCmn.fs_kHz == 16 as libc::c_int {
            (*psEnc)
                .mu_LTP_Q8 = ((0.02f32
                * ((1 as libc::c_int as int64_t) << 8 as libc::c_int) as libc::c_float)
                as libc::c_double + 0.5f64) as libc::c_int;
            (*psEnc).sCmn.bitrate_threshold_up = 30000 as libc::c_int;
            (*psEnc).sCmn.bitrate_threshold_down = 14000 as libc::c_int;
        } else if (*psEnc).sCmn.fs_kHz == 12 as libc::c_int {
            (*psEnc)
                .mu_LTP_Q8 = ((0.025f32
                * ((1 as libc::c_int as int64_t) << 8 as libc::c_int) as libc::c_float)
                as libc::c_double + 0.5f64) as libc::c_int;
            (*psEnc).sCmn.bitrate_threshold_up = 18000 as libc::c_int;
            (*psEnc).sCmn.bitrate_threshold_down = 10000 as libc::c_int;
        } else {
            (*psEnc)
                .mu_LTP_Q8 = ((0.03f32
                * ((1 as libc::c_int as int64_t) << 8 as libc::c_int) as libc::c_float)
                as libc::c_double + 0.5f64) as libc::c_int;
            (*psEnc).sCmn.bitrate_threshold_up = 14000 as libc::c_int;
            (*psEnc).sCmn.bitrate_threshold_down = 0 as libc::c_int;
        }
        (*psEnc).sCmn.fs_kHz_changed = 1 as libc::c_int;
    }
    return ret;
}
#[inline]
unsafe extern "C" fn SKP_Silk_setup_rate_FIX(
    mut psEnc: *mut SKP_Silk_encoder_state_FIX,
    mut TargetRate_bps: libc::c_int,
) -> libc::c_int {
    let mut k: libc::c_int = 0;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut frac_Q6: libc::c_int = 0;
    let mut rateTable: *const libc::c_int = 0 as *const libc::c_int;
    if TargetRate_bps != (*psEnc).sCmn.TargetRate_bps {
        (*psEnc).sCmn.TargetRate_bps = TargetRate_bps;
        if (*psEnc).sCmn.fs_kHz == 8 as libc::c_int {
            rateTable = TargetRate_table_NB.as_ptr();
        } else if (*psEnc).sCmn.fs_kHz == 12 as libc::c_int {
            rateTable = TargetRate_table_MB.as_ptr();
        } else if (*psEnc).sCmn.fs_kHz == 16 as libc::c_int {
            rateTable = TargetRate_table_WB.as_ptr();
        } else {
            rateTable = TargetRate_table_SWB.as_ptr();
        }
        k = 1 as libc::c_int;
        while k < 8 as libc::c_int {
            if TargetRate_bps <= *rateTable.offset(k as isize) {
                frac_Q6 = (TargetRate_bps
                    - *rateTable.offset((k - 1 as libc::c_int) as isize)
                    << 6 as libc::c_int)
                    / (*rateTable.offset(k as isize)
                        - *rateTable.offset((k - 1 as libc::c_int) as isize));
                (*psEnc)
                    .SNR_dB_Q7 = (SNR_table_Q1[(k - 1 as libc::c_int) as usize]
                    << 6 as libc::c_int)
                    + frac_Q6
                        * (SNR_table_Q1[k as usize]
                            - SNR_table_Q1[(k - 1 as libc::c_int) as usize]);
                break;
            } else {
                k += 1;
                k;
            }
        }
    }
    return ret;
}
#[inline]
unsafe extern "C" fn SKP_Silk_setup_LBRR_FIX(
    mut psEnc: *mut SKP_Silk_encoder_state_FIX,
) -> libc::c_int {
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut LBRRRate_thres_bps: libc::c_int = 0;
    if (*psEnc).sCmn.useInBandFEC < 0 as libc::c_int
        || (*psEnc).sCmn.useInBandFEC > 1 as libc::c_int
    {
        ret = -(7 as libc::c_int);
    }
    (*psEnc).sCmn.LBRR_enabled = (*psEnc).sCmn.useInBandFEC;
    if (*psEnc).sCmn.fs_kHz == 8 as libc::c_int {
        LBRRRate_thres_bps = 18000 as libc::c_int - 9000 as libc::c_int;
    } else if (*psEnc).sCmn.fs_kHz == 12 as libc::c_int {
        LBRRRate_thres_bps = 18000 as libc::c_int - 6000 as libc::c_int;
    } else if (*psEnc).sCmn.fs_kHz == 16 as libc::c_int {
        LBRRRate_thres_bps = 18000 as libc::c_int - 3000 as libc::c_int;
    } else {
        LBRRRate_thres_bps = 18000 as libc::c_int;
    }
    if (*psEnc).sCmn.TargetRate_bps >= LBRRRate_thres_bps {
        (*psEnc)
            .sCmn
            .LBRR_GainIncreases = SKP_max_int(
            8 as libc::c_int - ((*psEnc).sCmn.PacketLoss_perc >> 1 as libc::c_int),
            0 as libc::c_int,
        );
        if (*psEnc).sCmn.LBRR_enabled != 0
            && (*psEnc).sCmn.PacketLoss_perc > 1 as libc::c_int
        {
            (*psEnc)
                .inBandFEC_SNR_comp_Q8 = ((6.0f32
                * ((1 as libc::c_int as int64_t) << 8 as libc::c_int) as libc::c_float)
                as libc::c_double + 0.5f64) as libc::c_int
                - ((*psEnc).sCmn.LBRR_GainIncreases << 7 as libc::c_int);
        } else {
            (*psEnc).inBandFEC_SNR_comp_Q8 = 0 as libc::c_int;
            (*psEnc).sCmn.LBRR_enabled = 0 as libc::c_int;
        }
    } else {
        (*psEnc).inBandFEC_SNR_comp_Q8 = 0 as libc::c_int;
        (*psEnc).sCmn.LBRR_enabled = 0 as libc::c_int;
    }
    return ret;
}
