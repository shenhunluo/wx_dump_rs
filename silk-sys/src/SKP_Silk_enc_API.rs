#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn SKP_Silk_resampler(
        S: *mut SKP_Silk_resampler_state_struct,
        out: *mut libc::c_short,
        in_0: *const libc::c_short,
        inLen: libc::c_int,
    ) -> libc::c_int;
    fn SKP_Silk_detect_SWB_input(
        psSWBdetect: *mut SKP_Silk_detect_SWB_state,
        samplesIn: *const libc::c_short,
        nSamplesIn: libc::c_int,
    );
    fn SKP_Silk_init_encoder_FIX(psEnc: *mut SKP_Silk_encoder_state_FIX) -> libc::c_int;
    fn SKP_Silk_control_encoder_FIX(
        psEnc: *mut SKP_Silk_encoder_state_FIX,
        PacketSize_ms: libc::c_int,
        TargetRate_bps: libc::c_int,
        PacketLoss_perc: libc::c_int,
        DTX_enabled: libc::c_int,
        Complexity: libc::c_int,
    ) -> libc::c_int;
    fn SKP_Silk_encode_frame_FIX(
        psEnc: *mut SKP_Silk_encoder_state_FIX,
        pCode: *mut libc::c_uchar,
        pnBytesOut: *mut libc::c_short,
        pIn: *const libc::c_short,
    ) -> libc::c_int;
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
pub struct SKP_SILK_SDK_EncControlStruct {
    pub API_sampleRate: libc::c_int,
    pub maxInternalSampleRate: libc::c_int,
    pub packetSize: libc::c_int,
    pub bitRate: libc::c_int,
    pub packetLossPercentage: libc::c_int,
    pub complexity: libc::c_int,
    pub useInBandFEC: libc::c_int,
    pub useDTX: libc::c_int,
}
#[inline]
unsafe extern "C" fn SKP_min_int(mut a: libc::c_int, mut b: libc::c_int) -> libc::c_int {
    return if a < b { a } else { b };
}
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_SDK_Get_Encoder_Size(
    mut encSizeBytes: *mut libc::c_int,
) -> libc::c_int {
    let mut ret: libc::c_int = 0 as libc::c_int;
    *encSizeBytes = ::core::mem::size_of::<SKP_Silk_encoder_state_FIX>() as libc::c_ulong
        as libc::c_int;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_SDK_QueryEncoder(
    mut encState: *const libc::c_void,
    mut encStatus: *mut SKP_SILK_SDK_EncControlStruct,
) -> libc::c_int {
    let mut psEnc: *mut SKP_Silk_encoder_state_FIX = 0
        as *mut SKP_Silk_encoder_state_FIX;
    let mut ret: libc::c_int = 0 as libc::c_int;
    psEnc = encState as *mut SKP_Silk_encoder_state_FIX;
    (*encStatus).API_sampleRate = (*psEnc).sCmn.API_fs_Hz;
    (*encStatus)
        .maxInternalSampleRate = (*psEnc).sCmn.maxInternal_fs_kHz as libc::c_short
        as libc::c_int * 1000 as libc::c_int as libc::c_short as libc::c_int;
    (*encStatus)
        .packetSize = (*psEnc).sCmn.API_fs_Hz * (*psEnc).sCmn.PacketSize_ms
        / 1000 as libc::c_int;
    (*encStatus).bitRate = (*psEnc).sCmn.TargetRate_bps;
    (*encStatus).packetLossPercentage = (*psEnc).sCmn.PacketLoss_perc;
    (*encStatus).complexity = (*psEnc).sCmn.Complexity;
    (*encStatus).useInBandFEC = (*psEnc).sCmn.useInBandFEC;
    (*encStatus).useDTX = (*psEnc).sCmn.useDTX;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_SDK_InitEncoder(
    mut encState: *mut libc::c_void,
    mut encStatus: *mut SKP_SILK_SDK_EncControlStruct,
) -> libc::c_int {
    let mut psEnc: *mut SKP_Silk_encoder_state_FIX = 0
        as *mut SKP_Silk_encoder_state_FIX;
    let mut ret: libc::c_int = 0 as libc::c_int;
    psEnc = encState as *mut SKP_Silk_encoder_state_FIX;
    ret += SKP_Silk_init_encoder_FIX(psEnc);
    ret != 0;
    ret += SKP_Silk_SDK_QueryEncoder(encState, encStatus);
    ret != 0;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_SDK_Encode(
    mut encState: *mut libc::c_void,
    mut encControl: *const SKP_SILK_SDK_EncControlStruct,
    mut samplesIn: *const libc::c_short,
    mut nSamplesIn: libc::c_int,
    mut outData: *mut libc::c_uchar,
    mut nBytesOut: *mut libc::c_short,
) -> libc::c_int {
    let mut max_internal_fs_kHz: libc::c_int = 0;
    let mut PacketSize_ms: libc::c_int = 0;
    let mut PacketLoss_perc: libc::c_int = 0;
    let mut UseInBandFEC: libc::c_int = 0;
    let mut UseDTX: libc::c_int = 0;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut nSamplesToBuffer: libc::c_int = 0;
    let mut Complexity: libc::c_int = 0;
    let mut input_10ms: libc::c_int = 0;
    let mut nSamplesFromInput: libc::c_int = 0 as libc::c_int;
    let mut TargetRate_bps: libc::c_int = 0;
    let mut API_fs_Hz: libc::c_int = 0;
    let mut MaxBytesOut: libc::c_short = 0;
    let mut psEnc: *mut SKP_Silk_encoder_state_FIX = encState
        as *mut SKP_Silk_encoder_state_FIX;
    if (*encControl).API_sampleRate != 8000 as libc::c_int
        && (*encControl).API_sampleRate != 12000 as libc::c_int
        && (*encControl).API_sampleRate != 16000 as libc::c_int
        && (*encControl).API_sampleRate != 24000 as libc::c_int
        && (*encControl).API_sampleRate != 32000 as libc::c_int
        && (*encControl).API_sampleRate != 44100 as libc::c_int
        && (*encControl).API_sampleRate != 48000 as libc::c_int
        || (*encControl).maxInternalSampleRate != 8000 as libc::c_int
            && (*encControl).maxInternalSampleRate != 12000 as libc::c_int
            && (*encControl).maxInternalSampleRate != 16000 as libc::c_int
            && (*encControl).maxInternalSampleRate != 24000 as libc::c_int
    {
        ret = -(2 as libc::c_int);
        return ret;
    }
    API_fs_Hz = (*encControl).API_sampleRate;
    max_internal_fs_kHz = ((*encControl).maxInternalSampleRate >> 10 as libc::c_int)
        + 1 as libc::c_int;
    PacketSize_ms = 1000 as libc::c_int * (*encControl).packetSize / API_fs_Hz;
    TargetRate_bps = (*encControl).bitRate;
    PacketLoss_perc = (*encControl).packetLossPercentage;
    UseInBandFEC = (*encControl).useInBandFEC;
    Complexity = (*encControl).complexity;
    UseDTX = (*encControl).useDTX;
    (*psEnc).sCmn.API_fs_Hz = API_fs_Hz;
    (*psEnc).sCmn.maxInternal_fs_kHz = max_internal_fs_kHz;
    (*psEnc).sCmn.useInBandFEC = UseInBandFEC;
    input_10ms = 100 as libc::c_int * nSamplesIn / API_fs_Hz;
    if input_10ms * API_fs_Hz != 100 as libc::c_int * nSamplesIn
        || nSamplesIn < 0 as libc::c_int
    {
        ret = -(1 as libc::c_int);
        return ret;
    }
    TargetRate_bps = if 5000 as libc::c_int > 100000 as libc::c_int {
        if TargetRate_bps > 5000 as libc::c_int {
            5000 as libc::c_int
        } else if TargetRate_bps < 100000 as libc::c_int {
            100000 as libc::c_int
        } else {
            TargetRate_bps
        }
    } else if TargetRate_bps > 100000 as libc::c_int {
        100000 as libc::c_int
    } else if TargetRate_bps < 5000 as libc::c_int {
        5000 as libc::c_int
    } else {
        TargetRate_bps
    };
    ret = SKP_Silk_control_encoder_FIX(
        psEnc,
        PacketSize_ms,
        TargetRate_bps,
        PacketLoss_perc,
        UseDTX,
        Complexity,
    );
    if ret != 0 as libc::c_int {
        return ret;
    }
    if 1000 as libc::c_int * nSamplesIn > (*psEnc).sCmn.PacketSize_ms * API_fs_Hz {
        ret = -(1 as libc::c_int);
        return ret;
    }
    if (if API_fs_Hz < 1000 as libc::c_int * max_internal_fs_kHz {
        API_fs_Hz
    } else {
        1000 as libc::c_int * max_internal_fs_kHz
    }) == 24000 as libc::c_int
        && (*psEnc).sCmn.sSWBdetect.SWB_detected == 0 as libc::c_int
        && (*psEnc).sCmn.sSWBdetect.WB_detected == 0 as libc::c_int
    {
        SKP_Silk_detect_SWB_input(&mut (*psEnc).sCmn.sSWBdetect, samplesIn, nSamplesIn);
    }
    MaxBytesOut = 0 as libc::c_int as libc::c_short;
    loop {
        nSamplesToBuffer = (*psEnc).sCmn.frame_length - (*psEnc).sCmn.inputBufIx;
        if API_fs_Hz
            == 1000 as libc::c_int as libc::c_short as libc::c_int
                * (*psEnc).sCmn.fs_kHz as libc::c_short as libc::c_int
        {
            nSamplesToBuffer = SKP_min_int(nSamplesToBuffer, nSamplesIn);
            nSamplesFromInput = nSamplesToBuffer;
            memcpy(
                &mut *((*psEnc).sCmn.inputBuf)
                    .as_mut_ptr()
                    .offset((*psEnc).sCmn.inputBufIx as isize) as *mut libc::c_short
                    as *mut libc::c_void,
                samplesIn as *const libc::c_void,
                (nSamplesFromInput as libc::c_ulong)
                    .wrapping_mul(
                        ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                    ),
            );
        } else {
            nSamplesToBuffer = if nSamplesToBuffer
                < 10 as libc::c_int * input_10ms * (*psEnc).sCmn.fs_kHz
            {
                nSamplesToBuffer
            } else {
                10 as libc::c_int * input_10ms * (*psEnc).sCmn.fs_kHz
            };
            nSamplesFromInput = nSamplesToBuffer * API_fs_Hz
                / ((*psEnc).sCmn.fs_kHz * 1000 as libc::c_int);
            ret
                += SKP_Silk_resampler(
                    &mut (*psEnc).sCmn.resampler_state,
                    &mut *((*psEnc).sCmn.inputBuf)
                        .as_mut_ptr()
                        .offset((*psEnc).sCmn.inputBufIx as isize),
                    samplesIn,
                    nSamplesFromInput,
                );
        }
        samplesIn = samplesIn.offset(nSamplesFromInput as isize);
        nSamplesIn -= nSamplesFromInput;
        (*psEnc).sCmn.inputBufIx += nSamplesToBuffer;
        if !((*psEnc).sCmn.inputBufIx >= (*psEnc).sCmn.frame_length) {
            break;
        }
        if MaxBytesOut as libc::c_int == 0 as libc::c_int {
            MaxBytesOut = *nBytesOut;
            ret = SKP_Silk_encode_frame_FIX(
                psEnc,
                outData,
                &mut MaxBytesOut,
                ((*psEnc).sCmn.inputBuf).as_mut_ptr(),
            );
            ret != 0 as libc::c_int;
        } else {
            ret = SKP_Silk_encode_frame_FIX(
                psEnc,
                outData,
                nBytesOut,
                ((*psEnc).sCmn.inputBuf).as_mut_ptr(),
            );
            ret != 0 as libc::c_int;
        }
        (*psEnc).sCmn.inputBufIx = 0 as libc::c_int;
        (*psEnc).sCmn.controlled_since_last_payload = 0 as libc::c_int;
        if nSamplesIn == 0 as libc::c_int {
            break;
        }
    }
    *nBytesOut = MaxBytesOut;
    if (*psEnc).sCmn.useDTX != 0 && (*psEnc).sCmn.inDTX != 0 {
        *nBytesOut = 0 as libc::c_int as libc::c_short;
    }
    return ret;
}
