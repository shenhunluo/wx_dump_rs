#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    static SKP_Silk_type_offset_CDF: [libc::c_ushort; 5];
    static SKP_Silk_type_offset_joint_CDF: [[libc::c_ushort; 5]; 4];
    static SKP_Silk_gain_CDF: [[libc::c_ushort; 65]; 2];
    static SKP_Silk_delta_gain_CDF: [libc::c_ushort; 46];
    static SKP_Silk_pitch_lag_NB_CDF: [libc::c_ushort; 130];
    static SKP_Silk_pitch_lag_MB_CDF: [libc::c_ushort; 194];
    static SKP_Silk_pitch_lag_WB_CDF: [libc::c_ushort; 258];
    static SKP_Silk_pitch_lag_SWB_CDF: [libc::c_ushort; 386];
    static SKP_Silk_pitch_contour_CDF: [libc::c_ushort; 35];
    static SKP_Silk_pitch_contour_NB_CDF: [libc::c_ushort; 12];
    static SKP_Silk_LTP_per_index_CDF: [libc::c_ushort; 4];
    static SKP_Silk_LTP_gain_CDF_ptrs: [*const libc::c_ushort; 3];
    static SKP_Silk_LTPscale_CDF: [libc::c_ushort; 4];
    static SKP_Silk_vadflag_CDF: [libc::c_ushort; 3];
    static SKP_Silk_SamplingRates_table: [libc::c_int; 4];
    static SKP_Silk_SamplingRates_CDF: [libc::c_ushort; 5];
    static SKP_Silk_NLSF_interpolation_factor_CDF: [libc::c_ushort; 6];
    static SKP_Silk_Seed_CDF: [libc::c_ushort; 5];
    fn SKP_Silk_encode_pulses(
        psRC: *mut SKP_Silk_range_coder_state,
        sigtype: libc::c_int,
        QuantOffsetType: libc::c_int,
        q: *const libc::c_schar,
        frame_length: libc::c_int,
    );
    fn SKP_Silk_range_encoder(
        psRC: *mut SKP_Silk_range_coder_state,
        data: libc::c_int,
        prob: *const libc::c_ushort,
    );
    fn SKP_Silk_range_encoder_multi(
        psRC: *mut SKP_Silk_range_coder_state,
        data: *const libc::c_int,
        prob: *const *const libc::c_ushort,
        nSymbols: libc::c_int,
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
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_encode_parameters(
    mut psEncC: *mut SKP_Silk_encoder_state,
    mut psEncCtrlC: *mut SKP_Silk_encoder_control,
    mut psRC: *mut SKP_Silk_range_coder_state,
    mut q: *const libc::c_schar,
) {
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut typeOffset: libc::c_int = 0;
    let mut psNLSF_CB: *const SKP_Silk_NLSF_CB_struct = 0
        as *const SKP_Silk_NLSF_CB_struct;
    if (*psEncC).nFramesInPayloadBuf == 0 as libc::c_int {
        i = 0 as libc::c_int;
        while i < 3 as libc::c_int {
            if SKP_Silk_SamplingRates_table[i as usize] == (*psEncC).fs_kHz {
                break;
            }
            i += 1;
            i;
        }
        SKP_Silk_range_encoder(psRC, i, SKP_Silk_SamplingRates_CDF.as_ptr());
    }
    typeOffset = 2 as libc::c_int * (*psEncCtrlC).sigtype
        + (*psEncCtrlC).QuantOffsetType;
    if (*psEncC).nFramesInPayloadBuf == 0 as libc::c_int {
        SKP_Silk_range_encoder(psRC, typeOffset, SKP_Silk_type_offset_CDF.as_ptr());
    } else {
        SKP_Silk_range_encoder(
            psRC,
            typeOffset,
            (SKP_Silk_type_offset_joint_CDF[(*psEncC).typeOffsetPrev as usize]).as_ptr(),
        );
    }
    (*psEncC).typeOffsetPrev = typeOffset;
    if (*psEncC).nFramesInPayloadBuf == 0 as libc::c_int {
        SKP_Silk_range_encoder(
            psRC,
            (*psEncCtrlC).GainsIndices[0 as libc::c_int as usize],
            (SKP_Silk_gain_CDF[(*psEncCtrlC).sigtype as usize]).as_ptr(),
        );
    } else {
        SKP_Silk_range_encoder(
            psRC,
            (*psEncCtrlC).GainsIndices[0 as libc::c_int as usize],
            SKP_Silk_delta_gain_CDF.as_ptr(),
        );
    }
    i = 1 as libc::c_int;
    while i < 4 as libc::c_int {
        SKP_Silk_range_encoder(
            psRC,
            (*psEncCtrlC).GainsIndices[i as usize],
            SKP_Silk_delta_gain_CDF.as_ptr(),
        );
        i += 1;
        i;
    }
    psNLSF_CB = (*psEncC).psNLSF_CB[(*psEncCtrlC).sigtype as usize];
    SKP_Silk_range_encoder_multi(
        psRC,
        ((*psEncCtrlC).NLSFIndices).as_mut_ptr() as *const libc::c_int,
        (*psNLSF_CB).StartPtr,
        (*psNLSF_CB).nStages,
    );
    SKP_Silk_range_encoder(
        psRC,
        (*psEncCtrlC).NLSFInterpCoef_Q2,
        SKP_Silk_NLSF_interpolation_factor_CDF.as_ptr(),
    );
    if (*psEncCtrlC).sigtype == 0 as libc::c_int {
        if (*psEncC).fs_kHz == 8 as libc::c_int {
            SKP_Silk_range_encoder(
                psRC,
                (*psEncCtrlC).lagIndex,
                SKP_Silk_pitch_lag_NB_CDF.as_ptr(),
            );
        } else if (*psEncC).fs_kHz == 12 as libc::c_int {
            SKP_Silk_range_encoder(
                psRC,
                (*psEncCtrlC).lagIndex,
                SKP_Silk_pitch_lag_MB_CDF.as_ptr(),
            );
        } else if (*psEncC).fs_kHz == 16 as libc::c_int {
            SKP_Silk_range_encoder(
                psRC,
                (*psEncCtrlC).lagIndex,
                SKP_Silk_pitch_lag_WB_CDF.as_ptr(),
            );
        } else {
            SKP_Silk_range_encoder(
                psRC,
                (*psEncCtrlC).lagIndex,
                SKP_Silk_pitch_lag_SWB_CDF.as_ptr(),
            );
        }
        if (*psEncC).fs_kHz == 8 as libc::c_int {
            SKP_Silk_range_encoder(
                psRC,
                (*psEncCtrlC).contourIndex,
                SKP_Silk_pitch_contour_NB_CDF.as_ptr(),
            );
        } else {
            SKP_Silk_range_encoder(
                psRC,
                (*psEncCtrlC).contourIndex,
                SKP_Silk_pitch_contour_CDF.as_ptr(),
            );
        }
        SKP_Silk_range_encoder(
            psRC,
            (*psEncCtrlC).PERIndex,
            SKP_Silk_LTP_per_index_CDF.as_ptr(),
        );
        k = 0 as libc::c_int;
        while k < 4 as libc::c_int {
            SKP_Silk_range_encoder(
                psRC,
                (*psEncCtrlC).LTPIndex[k as usize],
                SKP_Silk_LTP_gain_CDF_ptrs[(*psEncCtrlC).PERIndex as usize],
            );
            k += 1;
            k;
        }
        SKP_Silk_range_encoder(
            psRC,
            (*psEncCtrlC).LTP_scaleIndex,
            SKP_Silk_LTPscale_CDF.as_ptr(),
        );
    }
    SKP_Silk_range_encoder(psRC, (*psEncCtrlC).Seed, SKP_Silk_Seed_CDF.as_ptr());
    SKP_Silk_encode_pulses(
        psRC,
        (*psEncCtrlC).sigtype,
        (*psEncCtrlC).QuantOffsetType,
        q,
        (*psEncC).frame_length,
    );
    SKP_Silk_range_encoder(psRC, (*psEncC).vadFlag, SKP_Silk_vadflag_CDF.as_ptr());
}
