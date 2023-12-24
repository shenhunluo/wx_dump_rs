#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
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
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_control_audio_bandwidth(
    mut psEncC: *mut SKP_Silk_encoder_state,
    TargetRate_bps: libc::c_int,
) -> libc::c_int {
    let mut fs_kHz: libc::c_int = 0;
    fs_kHz = (*psEncC).fs_kHz;
    if fs_kHz == 0 as libc::c_int {
        if TargetRate_bps >= 25000 as libc::c_int {
            fs_kHz = 24 as libc::c_int;
        } else if TargetRate_bps >= 14000 as libc::c_int {
            fs_kHz = 16 as libc::c_int;
        } else if TargetRate_bps >= 10000 as libc::c_int {
            fs_kHz = 12 as libc::c_int;
        } else {
            fs_kHz = 8 as libc::c_int;
        }
        fs_kHz = if fs_kHz < (*psEncC).API_fs_Hz / 1000 as libc::c_int {
            fs_kHz
        } else {
            (*psEncC).API_fs_Hz / 1000 as libc::c_int
        };
        fs_kHz = if fs_kHz < (*psEncC).maxInternal_fs_kHz {
            fs_kHz
        } else {
            (*psEncC).maxInternal_fs_kHz
        };
    } else if fs_kHz as libc::c_short as libc::c_int
        * 1000 as libc::c_int as libc::c_short as libc::c_int > (*psEncC).API_fs_Hz
        || fs_kHz > (*psEncC).maxInternal_fs_kHz
    {
        fs_kHz = (*psEncC).API_fs_Hz / 1000 as libc::c_int;
        fs_kHz = if fs_kHz < (*psEncC).maxInternal_fs_kHz {
            fs_kHz
        } else {
            (*psEncC).maxInternal_fs_kHz
        };
    } else {
        if (*psEncC).API_fs_Hz > 8000 as libc::c_int {
            (*psEncC).bitrateDiff
                += (*psEncC).PacketSize_ms
                    * (TargetRate_bps - (*psEncC).bitrate_threshold_down);
            (*psEncC)
                .bitrateDiff = if (*psEncC).bitrateDiff < 0 as libc::c_int {
                (*psEncC).bitrateDiff
            } else {
                0 as libc::c_int
            };
            if (*psEncC).vadFlag == 0 as libc::c_int {
                if (*psEncC).sLP.transition_frame_no == 0 as libc::c_int
                    && ((*psEncC).bitrateDiff <= -(30000000 as libc::c_int)
                        || (*psEncC).sSWBdetect.WB_detected * (*psEncC).fs_kHz
                            == 24 as libc::c_int)
                {
                    (*psEncC).sLP.transition_frame_no = 1 as libc::c_int;
                    (*psEncC).sLP.mode = 0 as libc::c_int;
                } else if (*psEncC).sLP.transition_frame_no
                    >= 2560 as libc::c_int / 20 as libc::c_int
                    && (*psEncC).sLP.mode == 0 as libc::c_int
                {
                    (*psEncC).sLP.transition_frame_no = 0 as libc::c_int;
                    (*psEncC).bitrateDiff = 0 as libc::c_int;
                    if (*psEncC).fs_kHz == 24 as libc::c_int {
                        fs_kHz = 16 as libc::c_int;
                    } else if (*psEncC).fs_kHz == 16 as libc::c_int {
                        fs_kHz = 12 as libc::c_int;
                    } else {
                        fs_kHz = 8 as libc::c_int;
                    }
                }
                if ((*psEncC).fs_kHz * 1000 as libc::c_int) < (*psEncC).API_fs_Hz
                    && TargetRate_bps >= (*psEncC).bitrate_threshold_up
                    && (*psEncC).sSWBdetect.WB_detected * (*psEncC).fs_kHz
                        < 16 as libc::c_int
                    && ((*psEncC).fs_kHz == 16 as libc::c_int
                        && (*psEncC).maxInternal_fs_kHz >= 24 as libc::c_int
                        || (*psEncC).fs_kHz == 12 as libc::c_int
                            && (*psEncC).maxInternal_fs_kHz >= 16 as libc::c_int
                        || (*psEncC).fs_kHz == 8 as libc::c_int
                            && (*psEncC).maxInternal_fs_kHz >= 12 as libc::c_int)
                    && (*psEncC).sLP.transition_frame_no == 0 as libc::c_int
                {
                    (*psEncC).sLP.mode = 1 as libc::c_int;
                    (*psEncC).bitrateDiff = 0 as libc::c_int;
                    if (*psEncC).fs_kHz == 8 as libc::c_int {
                        fs_kHz = 12 as libc::c_int;
                    } else if (*psEncC).fs_kHz == 12 as libc::c_int {
                        fs_kHz = 16 as libc::c_int;
                    } else {
                        fs_kHz = 24 as libc::c_int;
                    }
                }
            }
        }
        if (*psEncC).sLP.mode == 1 as libc::c_int
            && (*psEncC).sLP.transition_frame_no
                >= 5120 as libc::c_int / 20 as libc::c_int
            && (*psEncC).vadFlag == 0 as libc::c_int
        {
            (*psEncC).sLP.transition_frame_no = 0 as libc::c_int;
            memset(
                ((*psEncC).sLP.In_LP_State).as_mut_ptr() as *mut libc::c_void,
                0 as libc::c_int,
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
            );
        }
    }
    return fs_kHz;
}
