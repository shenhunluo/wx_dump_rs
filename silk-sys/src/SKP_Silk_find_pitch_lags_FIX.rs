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
    fn SKP_Silk_MA_Prediction(
        in_0: *const libc::c_short,
        B: *const libc::c_short,
        S: *mut libc::c_int,
        out: *mut libc::c_short,
        len: libc::c_int,
        order: libc::c_int,
    );
    fn SKP_Silk_bwexpander(
        ar: *mut libc::c_short,
        d: libc::c_int,
        chirp_Q16: libc::c_int,
    );
    fn SKP_Silk_schur(
        rc_Q15: *mut libc::c_short,
        c: *const libc::c_int,
        order: libc::c_int,
    ) -> libc::c_int;
    fn SKP_Silk_k2a(
        A_Q24: *mut libc::c_int,
        rc_Q15: *const libc::c_short,
        order: libc::c_int,
    );
    fn SKP_Silk_apply_sine_window(
        px_win: *mut libc::c_short,
        px: *const libc::c_short,
        win_type: libc::c_int,
        length: libc::c_int,
    );
    fn SKP_Silk_autocorr(
        results: *mut libc::c_int,
        scale: *mut libc::c_int,
        inputData: *const libc::c_short,
        inputDataSize: libc::c_int,
        correlationCount: libc::c_int,
    );
    fn SKP_Silk_pitch_analysis_core(
        signal: *const libc::c_short,
        pitch_out: *mut libc::c_int,
        lagIndex: *mut libc::c_int,
        contourIndex: *mut libc::c_int,
        LTPCorr_Q15: *mut libc::c_int,
        prevLag: libc::c_int,
        search_thres1_Q16: libc::c_int,
        search_thres2_Q15: libc::c_int,
        Fs_kHz: libc::c_int,
        complexity: libc::c_int,
        forLJC: libc::c_int,
    ) -> libc::c_int;
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
unsafe extern "C" fn SKP_max_int(mut a: libc::c_int, mut b: libc::c_int) -> libc::c_int {
    return if a > b { a } else { b };
}
#[inline]
unsafe extern "C" fn SKP_DIV32_varQ(
    a32: libc::c_int,
    b32: libc::c_int,
    Qres: libc::c_int,
) -> libc::c_int {
    let mut a_headrm: libc::c_int = 0;
    let mut b_headrm: libc::c_int = 0;
    let mut lshift: libc::c_int = 0;
    let mut b32_inv: libc::c_int = 0;
    let mut a32_nrm: libc::c_int = 0;
    let mut b32_nrm: libc::c_int = 0;
    let mut result: libc::c_int = 0;
    a_headrm = SKP_Silk_CLZ32((if a32 > 0 as libc::c_int { a32 } else { -a32 }))
        - 1 as libc::c_int;
    a32_nrm = a32 << a_headrm;
    b_headrm = SKP_Silk_CLZ32((if b32 > 0 as libc::c_int { b32 } else { -b32 }))
        - 1 as libc::c_int;
    b32_nrm = b32 << b_headrm;
    b32_inv = (0x7fffffff as libc::c_int >> 2 as libc::c_int)
        / (b32_nrm >> 16 as libc::c_int);
    result = (a32_nrm >> 16 as libc::c_int) * b32_inv as libc::c_short as libc::c_int
        + ((a32_nrm & 0xffff as libc::c_int) * b32_inv as libc::c_short as libc::c_int
            >> 16 as libc::c_int);
    a32_nrm
        -= ((b32_nrm as int64_t * result as int64_t >> 32 as libc::c_int) as libc::c_int)
            << 3 as libc::c_int;
    result = result
        + ((a32_nrm >> 16 as libc::c_int) * b32_inv as libc::c_short as libc::c_int
            + ((a32_nrm & 0xffff as libc::c_int)
                * b32_inv as libc::c_short as libc::c_int >> 16 as libc::c_int));
    lshift = 29 as libc::c_int + a_headrm - b_headrm - Qres;
    if lshift <= 0 as libc::c_int {
        return (if 0x80000000 as libc::c_uint as libc::c_int >> -lshift
            > 0x7fffffff as libc::c_int >> -lshift
        {
            (if result > 0x80000000 as libc::c_uint as libc::c_int >> -lshift {
                0x80000000 as libc::c_uint as libc::c_int >> -lshift
            } else {
                (if result < 0x7fffffff as libc::c_int >> -lshift {
                    0x7fffffff as libc::c_int >> -lshift
                } else {
                    result
                })
            })
        } else {
            (if result > 0x7fffffff as libc::c_int >> -lshift {
                0x7fffffff as libc::c_int >> -lshift
            } else {
                (if result < 0x80000000 as libc::c_uint as libc::c_int >> -lshift {
                    0x80000000 as libc::c_uint as libc::c_int >> -lshift
                } else {
                    result
                })
            })
        }) << -lshift
    } else if lshift < 32 as libc::c_int {
        return result >> lshift
    } else {
        return 0 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_find_pitch_lags_FIX(
    mut psEnc: *mut SKP_Silk_encoder_state_FIX,
    mut psEncCtrl: *mut SKP_Silk_encoder_control_FIX,
    mut res: *mut libc::c_short,
    mut x: *const libc::c_short,
) {
    let mut psPredSt: *mut SKP_Silk_predict_state_FIX = &mut (*psEnc).sPred;
    let mut buf_len: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut scale: libc::c_int = 0;
    let mut thrhld_Q15: libc::c_int = 0;
    let mut res_nrg: libc::c_int = 0;
    let mut x_buf: *const libc::c_short = 0 as *const libc::c_short;
    let mut x_buf_ptr: *const libc::c_short = 0 as *const libc::c_short;
    let mut Wsig: [libc::c_short; 576] = [0; 576];
    let mut Wsig_ptr: *mut libc::c_short = 0 as *mut libc::c_short;
    let mut auto_corr: [libc::c_int; 17] = [0; 17];
    let mut rc_Q15: [libc::c_short; 16] = [0; 16];
    let mut A_Q24: [libc::c_int; 16] = [0; 16];
    let mut FiltState: [libc::c_int; 16] = [0; 16];
    let mut A_Q12: [libc::c_short; 16] = [0; 16];
    buf_len = (*psEnc).sCmn.la_pitch + ((*psEnc).sCmn.frame_length << 1 as libc::c_int);
    x_buf = x.offset(-((*psEnc).sCmn.frame_length as isize));
    x_buf_ptr = x_buf
        .offset(buf_len as isize)
        .offset(-((*psPredSt).pitch_LPC_win_length as isize));
    Wsig_ptr = Wsig.as_mut_ptr();
    SKP_Silk_apply_sine_window(
        Wsig_ptr,
        x_buf_ptr,
        1 as libc::c_int,
        (*psEnc).sCmn.la_pitch,
    );
    Wsig_ptr = Wsig_ptr.offset((*psEnc).sCmn.la_pitch as isize);
    x_buf_ptr = x_buf_ptr.offset((*psEnc).sCmn.la_pitch as isize);
    memcpy(
        Wsig_ptr as *mut libc::c_void,
        x_buf_ptr as *const libc::c_void,
        (((*psPredSt).pitch_LPC_win_length
            - ((*psEnc).sCmn.la_pitch << 1 as libc::c_int)) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as libc::c_ulong),
    );
    Wsig_ptr = Wsig_ptr
        .offset(
            ((*psPredSt).pitch_LPC_win_length
                - ((*psEnc).sCmn.la_pitch << 1 as libc::c_int)) as isize,
        );
    x_buf_ptr = x_buf_ptr
        .offset(
            ((*psPredSt).pitch_LPC_win_length
                - ((*psEnc).sCmn.la_pitch << 1 as libc::c_int)) as isize,
        );
    SKP_Silk_apply_sine_window(
        Wsig_ptr,
        x_buf_ptr,
        2 as libc::c_int,
        (*psEnc).sCmn.la_pitch,
    );
    SKP_Silk_autocorr(
        auto_corr.as_mut_ptr(),
        &mut scale,
        Wsig.as_mut_ptr(),
        (*psPredSt).pitch_LPC_win_length,
        (*psEnc).sCmn.pitchEstimationLPCOrder + 1 as libc::c_int,
    );
    auto_corr[0 as libc::c_int
        as usize] = auto_corr[0 as libc::c_int as usize]
        + ((auto_corr[0 as libc::c_int as usize] >> 16 as libc::c_int)
            * ((1e-3f32
                * ((1 as libc::c_int as int64_t) << 16 as libc::c_int) as libc::c_float)
                as libc::c_double + 0.5f64) as libc::c_int as libc::c_short
                as libc::c_int
            + ((auto_corr[0 as libc::c_int as usize] & 0xffff as libc::c_int)
                * ((1e-3f32
                    * ((1 as libc::c_int as int64_t) << 16 as libc::c_int)
                        as libc::c_float) as libc::c_double + 0.5f64) as libc::c_int
                    as libc::c_short as libc::c_int >> 16 as libc::c_int));
    res_nrg = SKP_Silk_schur(
        rc_Q15.as_mut_ptr(),
        auto_corr.as_mut_ptr(),
        (*psEnc).sCmn.pitchEstimationLPCOrder,
    );
    (*psEncCtrl)
        .predGain_Q16 = SKP_DIV32_varQ(
        auto_corr[0 as libc::c_int as usize],
        SKP_max_int(res_nrg, 1 as libc::c_int),
        16 as libc::c_int,
    );
    SKP_Silk_k2a(
        A_Q24.as_mut_ptr(),
        rc_Q15.as_mut_ptr(),
        (*psEnc).sCmn.pitchEstimationLPCOrder,
    );
    i = 0 as libc::c_int;
    while i < (*psEnc).sCmn.pitchEstimationLPCOrder {
        A_Q12[i
            as usize] = (if A_Q24[i as usize] >> 12 as libc::c_int
            > 0x7fff as libc::c_int
        {
            0x7fff as libc::c_int
        } else if (A_Q24[i as usize] >> 12 as libc::c_int)
            < 0x8000 as libc::c_int as libc::c_short as libc::c_int
        {
            0x8000 as libc::c_int as libc::c_short as libc::c_int
        } else {
            A_Q24[i as usize] >> 12 as libc::c_int
        }) as libc::c_short;
        i += 1;
        i;
    }
    SKP_Silk_bwexpander(
        A_Q12.as_mut_ptr(),
        (*psEnc).sCmn.pitchEstimationLPCOrder,
        ((0.99f32
            * ((1 as libc::c_int as int64_t) << 16 as libc::c_int) as libc::c_float)
            as libc::c_double + 0.5f64) as libc::c_int,
    );
    memset(
        FiltState.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ((*psEnc).sCmn.pitchEstimationLPCOrder as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    SKP_Silk_MA_Prediction(
        x_buf,
        A_Q12.as_mut_ptr(),
        FiltState.as_mut_ptr(),
        res,
        buf_len,
        (*psEnc).sCmn.pitchEstimationLPCOrder,
    );
    memset(
        res as *mut libc::c_void,
        0 as libc::c_int,
        ((*psEnc).sCmn.pitchEstimationLPCOrder as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as libc::c_ulong),
    );
    thrhld_Q15 = (0.45f64
        * ((1 as libc::c_int as int64_t) << 15 as libc::c_int) as libc::c_double
        + 0.5f64) as libc::c_int;
    thrhld_Q15 = thrhld_Q15
        + (-0.004f64
            * ((1 as libc::c_int as int64_t) << 15 as libc::c_int) as libc::c_double
            + 0.5f64) as libc::c_int as libc::c_short as libc::c_int
            * (*psEnc).sCmn.pitchEstimationLPCOrder as libc::c_short as libc::c_int;
    thrhld_Q15 = thrhld_Q15
        + (-0.1f64
            * ((1 as libc::c_int as int64_t) << 7 as libc::c_int) as libc::c_double
            + 0.5f64) as libc::c_int as libc::c_short as libc::c_int
            * (*psEnc).speech_activity_Q8 as libc::c_short as libc::c_int;
    thrhld_Q15 = thrhld_Q15
        + (0.15f64
            * ((1 as libc::c_int as int64_t) << 15 as libc::c_int) as libc::c_double
            + 0.5f64) as libc::c_int as libc::c_short as libc::c_int
            * (*psEnc).sCmn.prev_sigtype as libc::c_short as libc::c_int;
    thrhld_Q15 = thrhld_Q15
        + (((-0.1f64
            * ((1 as libc::c_int as int64_t) << 16 as libc::c_int) as libc::c_double
            + 0.5f64) as libc::c_int >> 16 as libc::c_int)
            * (*psEncCtrl).input_tilt_Q15 as libc::c_short as libc::c_int
            + (((-0.1f64
                * ((1 as libc::c_int as int64_t) << 16 as libc::c_int) as libc::c_double
                + 0.5f64) as libc::c_int & 0xffff as libc::c_int)
                * (*psEncCtrl).input_tilt_Q15 as libc::c_short as libc::c_int
                >> 16 as libc::c_int));
    thrhld_Q15 = if thrhld_Q15 > 0x7fff as libc::c_int {
        0x7fff as libc::c_int
    } else if thrhld_Q15 < 0x8000 as libc::c_int as libc::c_short as libc::c_int {
        0x8000 as libc::c_int as libc::c_short as libc::c_int
    } else {
        thrhld_Q15
    };
    (*psEncCtrl)
        .sCmn
        .sigtype = SKP_Silk_pitch_analysis_core(
        res as *const libc::c_short,
        ((*psEncCtrl).sCmn.pitchL).as_mut_ptr(),
        &mut (*psEncCtrl).sCmn.lagIndex,
        &mut (*psEncCtrl).sCmn.contourIndex,
        &mut (*psEnc).LTPCorr_Q15,
        (*psEnc).sCmn.prevLag,
        (*psEnc).sCmn.pitchEstimationThreshold_Q16,
        thrhld_Q15 as libc::c_short as libc::c_int,
        (*psEnc).sCmn.fs_kHz,
        (*psEnc).sCmn.pitchEstimationComplexity,
        0 as libc::c_int,
    );
}
