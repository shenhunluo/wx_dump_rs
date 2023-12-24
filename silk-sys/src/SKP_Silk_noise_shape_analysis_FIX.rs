#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn SKP_Silk_bwexpander_32(
        ar: *mut libc::c_int,
        d: libc::c_int,
        chirp_Q16: libc::c_int,
    );
    fn SKP_Silk_LPC_inverse_pred_gain_Q24(
        invGain_Q30: *mut libc::c_int,
        A_Q24: *const libc::c_int,
        order: libc::c_int,
    ) -> libc::c_int;
    fn SKP_Silk_lin2log(inLin: libc::c_int) -> libc::c_int;
    fn SKP_Silk_sigm_Q15(in_Q5: libc::c_int) -> libc::c_int;
    fn SKP_Silk_log2lin(inLog_Q7: libc::c_int) -> libc::c_int;
    fn SKP_Silk_sum_sqr_shift(
        energy: *mut libc::c_int,
        shift: *mut libc::c_int,
        x: *const libc::c_short,
        len: libc::c_int,
    );
    fn SKP_Silk_schur64(
        rc_Q16: *mut libc::c_int,
        c: *const libc::c_int,
        order: libc::c_int,
    ) -> libc::c_int;
    fn SKP_Silk_k2a_Q16(
        A_Q24: *mut libc::c_int,
        rc_Q16: *const libc::c_int,
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
    fn SKP_Silk_warped_autocorrelation_FIX(
        corr: *mut libc::c_int,
        scale: *mut libc::c_int,
        input: *const libc::c_short,
        warping_Q16: libc::c_short,
        length: libc::c_int,
        order: libc::c_int,
    );
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
unsafe extern "C" fn SKP_ROR32(
    mut a32: libc::c_int,
    mut rot: libc::c_int,
) -> libc::c_int {
    let mut x: libc::c_uint = a32 as libc::c_uint;
    let mut r: libc::c_uint = rot as libc::c_uint;
    let mut m: libc::c_uint = -rot as libc::c_uint;
    if rot <= 0 as libc::c_int {
        return (x << m | x >> (32 as libc::c_int as libc::c_uint).wrapping_sub(m))
            as libc::c_int
    } else {
        return (x << (32 as libc::c_int as libc::c_uint).wrapping_sub(r) | x >> r)
            as libc::c_int
    };
}
#[inline]
unsafe extern "C" fn SKP_max_32(mut a: libc::c_int, mut b: libc::c_int) -> libc::c_int {
    return if a > b { a } else { b };
}
#[inline]
unsafe extern "C" fn SKP_Silk_CLZ_FRAC(
    mut in_0: libc::c_int,
    mut lz: *mut libc::c_int,
    mut frac_Q7: *mut libc::c_int,
) {
    let mut lzeros: libc::c_int = SKP_Silk_CLZ32(in_0);
    *lz = lzeros;
    *frac_Q7 = SKP_ROR32(in_0, 24 as libc::c_int - lzeros) & 0x7f as libc::c_int;
}
#[inline]
unsafe extern "C" fn SKP_Silk_SQRT_APPROX(mut x: libc::c_int) -> libc::c_int {
    let mut y: libc::c_int = 0;
    let mut lz: libc::c_int = 0;
    let mut frac_Q7: libc::c_int = 0;
    if x <= 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    SKP_Silk_CLZ_FRAC(x, &mut lz, &mut frac_Q7);
    if lz & 1 as libc::c_int != 0 {
        y = 32768 as libc::c_int;
    } else {
        y = 46214 as libc::c_int;
    }
    y >>= lz >> 1 as libc::c_int;
    y = y
        + ((y >> 16 as libc::c_int)
            * (213 as libc::c_int as libc::c_short as libc::c_int
                * frac_Q7 as libc::c_short as libc::c_int) as libc::c_short
                as libc::c_int
            + ((y & 0xffff as libc::c_int)
                * (213 as libc::c_int as libc::c_short as libc::c_int
                    * frac_Q7 as libc::c_short as libc::c_int) as libc::c_short
                    as libc::c_int >> 16 as libc::c_int));
    return y;
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
#[inline]
unsafe extern "C" fn SKP_INVERSE32_varQ(
    b32: libc::c_int,
    Qres: libc::c_int,
) -> libc::c_int {
    let mut b_headrm: libc::c_int = 0;
    let mut lshift: libc::c_int = 0;
    let mut b32_inv: libc::c_int = 0;
    let mut b32_nrm: libc::c_int = 0;
    let mut err_Q32: libc::c_int = 0;
    let mut result: libc::c_int = 0;
    b_headrm = SKP_Silk_CLZ32((if b32 > 0 as libc::c_int { b32 } else { -b32 }))
        - 1 as libc::c_int;
    b32_nrm = b32 << b_headrm;
    b32_inv = (0x7fffffff as libc::c_int >> 2 as libc::c_int)
        / (b32_nrm >> 16 as libc::c_int);
    result = b32_inv << 16 as libc::c_int;
    err_Q32 = -((b32_nrm >> 16 as libc::c_int) * b32_inv as libc::c_short as libc::c_int
        + ((b32_nrm & 0xffff as libc::c_int) * b32_inv as libc::c_short as libc::c_int
            >> 16 as libc::c_int)) << 3 as libc::c_int;
    result = result
        + ((err_Q32 >> 16 as libc::c_int) * b32_inv as libc::c_short as libc::c_int
            + ((err_Q32 & 0xffff as libc::c_int)
                * b32_inv as libc::c_short as libc::c_int >> 16 as libc::c_int))
        + err_Q32
            * (if 16 as libc::c_int == 1 as libc::c_int {
                (b32_inv >> 1 as libc::c_int) + (b32_inv & 1 as libc::c_int)
            } else {
                (b32_inv >> 16 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                    >> 1 as libc::c_int
            });
    lshift = 61 as libc::c_int - b_headrm - Qres;
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
#[inline]
unsafe extern "C" fn warped_gain(
    mut coefs_Q24: *const libc::c_int,
    mut lambda_Q16: libc::c_int,
    mut order: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut gain_Q24: libc::c_int = 0;
    lambda_Q16 = -lambda_Q16;
    gain_Q24 = *coefs_Q24.offset((order - 1 as libc::c_int) as isize);
    i = order - 2 as libc::c_int;
    while i >= 0 as libc::c_int {
        gain_Q24 = *coefs_Q24.offset(i as isize)
            + ((gain_Q24 >> 16 as libc::c_int)
                * lambda_Q16 as libc::c_short as libc::c_int
                + ((gain_Q24 & 0xffff as libc::c_int)
                    * lambda_Q16 as libc::c_short as libc::c_int >> 16 as libc::c_int));
        i -= 1;
        i;
    }
    gain_Q24 = (1.0f64
        * ((1 as libc::c_int as int64_t) << 24 as libc::c_int) as libc::c_double
        + 0.5f64) as libc::c_int
        + ((gain_Q24 >> 16 as libc::c_int) * -lambda_Q16 as libc::c_short as libc::c_int
            + ((gain_Q24 & 0xffff as libc::c_int)
                * -lambda_Q16 as libc::c_short as libc::c_int >> 16 as libc::c_int));
    return SKP_INVERSE32_varQ(gain_Q24, 40 as libc::c_int);
}
#[inline]
unsafe extern "C" fn limit_warped_coefs(
    mut coefs_syn_Q24: *mut libc::c_int,
    mut coefs_ana_Q24: *mut libc::c_int,
    mut lambda_Q16: libc::c_int,
    mut limit_Q24: libc::c_int,
    mut order: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut iter: libc::c_int = 0;
    let mut ind: libc::c_int = 0 as libc::c_int;
    let mut tmp: libc::c_int = 0;
    let mut maxabs_Q24: libc::c_int = 0;
    let mut chirp_Q16: libc::c_int = 0;
    let mut gain_syn_Q16: libc::c_int = 0;
    let mut gain_ana_Q16: libc::c_int = 0;
    let mut nom_Q16: libc::c_int = 0;
    let mut den_Q24: libc::c_int = 0;
    lambda_Q16 = -lambda_Q16;
    i = order - 1 as libc::c_int;
    while i > 0 as libc::c_int {
        *coefs_syn_Q24
            .offset(
                (i - 1 as libc::c_int) as isize,
            ) = *coefs_syn_Q24.offset((i - 1 as libc::c_int) as isize)
            + ((*coefs_syn_Q24.offset(i as isize) >> 16 as libc::c_int)
                * lambda_Q16 as libc::c_short as libc::c_int
                + ((*coefs_syn_Q24.offset(i as isize) & 0xffff as libc::c_int)
                    * lambda_Q16 as libc::c_short as libc::c_int >> 16 as libc::c_int));
        *coefs_ana_Q24
            .offset(
                (i - 1 as libc::c_int) as isize,
            ) = *coefs_ana_Q24.offset((i - 1 as libc::c_int) as isize)
            + ((*coefs_ana_Q24.offset(i as isize) >> 16 as libc::c_int)
                * lambda_Q16 as libc::c_short as libc::c_int
                + ((*coefs_ana_Q24.offset(i as isize) & 0xffff as libc::c_int)
                    * lambda_Q16 as libc::c_short as libc::c_int >> 16 as libc::c_int));
        i -= 1;
        i;
    }
    lambda_Q16 = -lambda_Q16;
    nom_Q16 = (1.0f64
        * ((1 as libc::c_int as int64_t) << 16 as libc::c_int) as libc::c_double
        + 0.5f64) as libc::c_int
        + ((-lambda_Q16 >> 16 as libc::c_int)
            * lambda_Q16 as libc::c_short as libc::c_int
            + ((-lambda_Q16 & 0xffff as libc::c_int)
                * lambda_Q16 as libc::c_short as libc::c_int >> 16 as libc::c_int));
    den_Q24 = (1.0f64
        * ((1 as libc::c_int as int64_t) << 24 as libc::c_int) as libc::c_double
        + 0.5f64) as libc::c_int
        + ((*coefs_syn_Q24.offset(0 as libc::c_int as isize) >> 16 as libc::c_int)
            * lambda_Q16 as libc::c_short as libc::c_int
            + ((*coefs_syn_Q24.offset(0 as libc::c_int as isize) & 0xffff as libc::c_int)
                * lambda_Q16 as libc::c_short as libc::c_int >> 16 as libc::c_int));
    gain_syn_Q16 = SKP_DIV32_varQ(nom_Q16, den_Q24, 24 as libc::c_int);
    den_Q24 = (1.0f64
        * ((1 as libc::c_int as int64_t) << 24 as libc::c_int) as libc::c_double
        + 0.5f64) as libc::c_int
        + ((*coefs_ana_Q24.offset(0 as libc::c_int as isize) >> 16 as libc::c_int)
            * lambda_Q16 as libc::c_short as libc::c_int
            + ((*coefs_ana_Q24.offset(0 as libc::c_int as isize) & 0xffff as libc::c_int)
                * lambda_Q16 as libc::c_short as libc::c_int >> 16 as libc::c_int));
    gain_ana_Q16 = SKP_DIV32_varQ(nom_Q16, den_Q24, 24 as libc::c_int);
    i = 0 as libc::c_int;
    while i < order {
        *coefs_syn_Q24
            .offset(
                i as isize,
            ) = (gain_syn_Q16 >> 16 as libc::c_int)
            * *coefs_syn_Q24.offset(i as isize) as libc::c_short as libc::c_int
            + ((gain_syn_Q16 & 0xffff as libc::c_int)
                * *coefs_syn_Q24.offset(i as isize) as libc::c_short as libc::c_int
                >> 16 as libc::c_int)
            + gain_syn_Q16
                * (if 16 as libc::c_int == 1 as libc::c_int {
                    (*coefs_syn_Q24.offset(i as isize) >> 1 as libc::c_int)
                        + (*coefs_syn_Q24.offset(i as isize) & 1 as libc::c_int)
                } else {
                    (*coefs_syn_Q24.offset(i as isize)
                        >> 16 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                        >> 1 as libc::c_int
                });
        *coefs_ana_Q24
            .offset(
                i as isize,
            ) = (gain_ana_Q16 >> 16 as libc::c_int)
            * *coefs_ana_Q24.offset(i as isize) as libc::c_short as libc::c_int
            + ((gain_ana_Q16 & 0xffff as libc::c_int)
                * *coefs_ana_Q24.offset(i as isize) as libc::c_short as libc::c_int
                >> 16 as libc::c_int)
            + gain_ana_Q16
                * (if 16 as libc::c_int == 1 as libc::c_int {
                    (*coefs_ana_Q24.offset(i as isize) >> 1 as libc::c_int)
                        + (*coefs_ana_Q24.offset(i as isize) & 1 as libc::c_int)
                } else {
                    (*coefs_ana_Q24.offset(i as isize)
                        >> 16 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                        >> 1 as libc::c_int
                });
        i += 1;
        i;
    }
    iter = 0 as libc::c_int;
    while iter < 10 as libc::c_int {
        maxabs_Q24 = -(1 as libc::c_int);
        i = 0 as libc::c_int;
        while i < order {
            tmp = if (*coefs_syn_Q24.offset(i as isize)
                ^ *coefs_syn_Q24.offset(i as isize) >> 31 as libc::c_int)
                - (*coefs_syn_Q24.offset(i as isize) >> 31 as libc::c_int)
                > (*coefs_ana_Q24.offset(i as isize)
                    ^ *coefs_ana_Q24.offset(i as isize) >> 31 as libc::c_int)
                    - (*coefs_ana_Q24.offset(i as isize) >> 31 as libc::c_int)
            {
                (*coefs_syn_Q24.offset(i as isize)
                    ^ *coefs_syn_Q24.offset(i as isize) >> 31 as libc::c_int)
                    - (*coefs_syn_Q24.offset(i as isize) >> 31 as libc::c_int)
            } else {
                (*coefs_ana_Q24.offset(i as isize)
                    ^ *coefs_ana_Q24.offset(i as isize) >> 31 as libc::c_int)
                    - (*coefs_ana_Q24.offset(i as isize) >> 31 as libc::c_int)
            };
            if tmp > maxabs_Q24 {
                maxabs_Q24 = tmp;
                ind = i;
            }
            i += 1;
            i;
        }
        if maxabs_Q24 <= limit_Q24 {
            return;
        }
        i = 1 as libc::c_int;
        while i < order {
            *coefs_syn_Q24
                .offset(
                    (i - 1 as libc::c_int) as isize,
                ) = *coefs_syn_Q24.offset((i - 1 as libc::c_int) as isize)
                + ((*coefs_syn_Q24.offset(i as isize) >> 16 as libc::c_int)
                    * lambda_Q16 as libc::c_short as libc::c_int
                    + ((*coefs_syn_Q24.offset(i as isize) & 0xffff as libc::c_int)
                        * lambda_Q16 as libc::c_short as libc::c_int
                        >> 16 as libc::c_int));
            *coefs_ana_Q24
                .offset(
                    (i - 1 as libc::c_int) as isize,
                ) = *coefs_ana_Q24.offset((i - 1 as libc::c_int) as isize)
                + ((*coefs_ana_Q24.offset(i as isize) >> 16 as libc::c_int)
                    * lambda_Q16 as libc::c_short as libc::c_int
                    + ((*coefs_ana_Q24.offset(i as isize) & 0xffff as libc::c_int)
                        * lambda_Q16 as libc::c_short as libc::c_int
                        >> 16 as libc::c_int));
            i += 1;
            i;
        }
        gain_syn_Q16 = SKP_INVERSE32_varQ(gain_syn_Q16, 32 as libc::c_int);
        gain_ana_Q16 = SKP_INVERSE32_varQ(gain_ana_Q16, 32 as libc::c_int);
        i = 0 as libc::c_int;
        while i < order {
            *coefs_syn_Q24
                .offset(
                    i as isize,
                ) = (gain_syn_Q16 >> 16 as libc::c_int)
                * *coefs_syn_Q24.offset(i as isize) as libc::c_short as libc::c_int
                + ((gain_syn_Q16 & 0xffff as libc::c_int)
                    * *coefs_syn_Q24.offset(i as isize) as libc::c_short as libc::c_int
                    >> 16 as libc::c_int)
                + gain_syn_Q16
                    * (if 16 as libc::c_int == 1 as libc::c_int {
                        (*coefs_syn_Q24.offset(i as isize) >> 1 as libc::c_int)
                            + (*coefs_syn_Q24.offset(i as isize) & 1 as libc::c_int)
                    } else {
                        (*coefs_syn_Q24.offset(i as isize)
                            >> 16 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                            >> 1 as libc::c_int
                    });
            *coefs_ana_Q24
                .offset(
                    i as isize,
                ) = (gain_ana_Q16 >> 16 as libc::c_int)
                * *coefs_ana_Q24.offset(i as isize) as libc::c_short as libc::c_int
                + ((gain_ana_Q16 & 0xffff as libc::c_int)
                    * *coefs_ana_Q24.offset(i as isize) as libc::c_short as libc::c_int
                    >> 16 as libc::c_int)
                + gain_ana_Q16
                    * (if 16 as libc::c_int == 1 as libc::c_int {
                        (*coefs_ana_Q24.offset(i as isize) >> 1 as libc::c_int)
                            + (*coefs_ana_Q24.offset(i as isize) & 1 as libc::c_int)
                    } else {
                        (*coefs_ana_Q24.offset(i as isize)
                            >> 16 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                            >> 1 as libc::c_int
                    });
            i += 1;
            i;
        }
        chirp_Q16 = (0.99f64
            * ((1 as libc::c_int as int64_t) << 16 as libc::c_int) as libc::c_double
            + 0.5f64) as libc::c_int
            - SKP_DIV32_varQ(
                (maxabs_Q24 - limit_Q24 >> 16 as libc::c_int)
                    * ((0.8f64
                        * ((1 as libc::c_int as int64_t) << 10 as libc::c_int)
                            as libc::c_double + 0.5f64) as libc::c_int
                        + (0.1f64
                            * ((1 as libc::c_int as int64_t) << 10 as libc::c_int)
                                as libc::c_double + 0.5f64) as libc::c_int as libc::c_short
                            as libc::c_int * iter as libc::c_short as libc::c_int)
                        as libc::c_short as libc::c_int
                    + ((maxabs_Q24 - limit_Q24 & 0xffff as libc::c_int)
                        * ((0.8f64
                            * ((1 as libc::c_int as int64_t) << 10 as libc::c_int)
                                as libc::c_double + 0.5f64) as libc::c_int
                            + (0.1f64
                                * ((1 as libc::c_int as int64_t) << 10 as libc::c_int)
                                    as libc::c_double + 0.5f64) as libc::c_int as libc::c_short
                                as libc::c_int * iter as libc::c_short as libc::c_int)
                            as libc::c_short as libc::c_int >> 16 as libc::c_int),
                maxabs_Q24 * (ind + 1 as libc::c_int),
                22 as libc::c_int,
            );
        SKP_Silk_bwexpander_32(coefs_syn_Q24, order, chirp_Q16);
        SKP_Silk_bwexpander_32(coefs_ana_Q24, order, chirp_Q16);
        lambda_Q16 = -lambda_Q16;
        i = order - 1 as libc::c_int;
        while i > 0 as libc::c_int {
            *coefs_syn_Q24
                .offset(
                    (i - 1 as libc::c_int) as isize,
                ) = *coefs_syn_Q24.offset((i - 1 as libc::c_int) as isize)
                + ((*coefs_syn_Q24.offset(i as isize) >> 16 as libc::c_int)
                    * lambda_Q16 as libc::c_short as libc::c_int
                    + ((*coefs_syn_Q24.offset(i as isize) & 0xffff as libc::c_int)
                        * lambda_Q16 as libc::c_short as libc::c_int
                        >> 16 as libc::c_int));
            *coefs_ana_Q24
                .offset(
                    (i - 1 as libc::c_int) as isize,
                ) = *coefs_ana_Q24.offset((i - 1 as libc::c_int) as isize)
                + ((*coefs_ana_Q24.offset(i as isize) >> 16 as libc::c_int)
                    * lambda_Q16 as libc::c_short as libc::c_int
                    + ((*coefs_ana_Q24.offset(i as isize) & 0xffff as libc::c_int)
                        * lambda_Q16 as libc::c_short as libc::c_int
                        >> 16 as libc::c_int));
            i -= 1;
            i;
        }
        lambda_Q16 = -lambda_Q16;
        nom_Q16 = (1.0f64
            * ((1 as libc::c_int as int64_t) << 16 as libc::c_int) as libc::c_double
            + 0.5f64) as libc::c_int
            + ((-lambda_Q16 >> 16 as libc::c_int)
                * lambda_Q16 as libc::c_short as libc::c_int
                + ((-lambda_Q16 & 0xffff as libc::c_int)
                    * lambda_Q16 as libc::c_short as libc::c_int >> 16 as libc::c_int));
        den_Q24 = (1.0f64
            * ((1 as libc::c_int as int64_t) << 24 as libc::c_int) as libc::c_double
            + 0.5f64) as libc::c_int
            + ((*coefs_syn_Q24.offset(0 as libc::c_int as isize) >> 16 as libc::c_int)
                * lambda_Q16 as libc::c_short as libc::c_int
                + ((*coefs_syn_Q24.offset(0 as libc::c_int as isize)
                    & 0xffff as libc::c_int) * lambda_Q16 as libc::c_short as libc::c_int
                    >> 16 as libc::c_int));
        gain_syn_Q16 = SKP_DIV32_varQ(nom_Q16, den_Q24, 24 as libc::c_int);
        den_Q24 = (1.0f64
            * ((1 as libc::c_int as int64_t) << 24 as libc::c_int) as libc::c_double
            + 0.5f64) as libc::c_int
            + ((*coefs_ana_Q24.offset(0 as libc::c_int as isize) >> 16 as libc::c_int)
                * lambda_Q16 as libc::c_short as libc::c_int
                + ((*coefs_ana_Q24.offset(0 as libc::c_int as isize)
                    & 0xffff as libc::c_int) * lambda_Q16 as libc::c_short as libc::c_int
                    >> 16 as libc::c_int));
        gain_ana_Q16 = SKP_DIV32_varQ(nom_Q16, den_Q24, 24 as libc::c_int);
        i = 0 as libc::c_int;
        while i < order {
            *coefs_syn_Q24
                .offset(
                    i as isize,
                ) = (gain_syn_Q16 >> 16 as libc::c_int)
                * *coefs_syn_Q24.offset(i as isize) as libc::c_short as libc::c_int
                + ((gain_syn_Q16 & 0xffff as libc::c_int)
                    * *coefs_syn_Q24.offset(i as isize) as libc::c_short as libc::c_int
                    >> 16 as libc::c_int)
                + gain_syn_Q16
                    * (if 16 as libc::c_int == 1 as libc::c_int {
                        (*coefs_syn_Q24.offset(i as isize) >> 1 as libc::c_int)
                            + (*coefs_syn_Q24.offset(i as isize) & 1 as libc::c_int)
                    } else {
                        (*coefs_syn_Q24.offset(i as isize)
                            >> 16 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                            >> 1 as libc::c_int
                    });
            *coefs_ana_Q24
                .offset(
                    i as isize,
                ) = (gain_ana_Q16 >> 16 as libc::c_int)
                * *coefs_ana_Q24.offset(i as isize) as libc::c_short as libc::c_int
                + ((gain_ana_Q16 & 0xffff as libc::c_int)
                    * *coefs_ana_Q24.offset(i as isize) as libc::c_short as libc::c_int
                    >> 16 as libc::c_int)
                + gain_ana_Q16
                    * (if 16 as libc::c_int == 1 as libc::c_int {
                        (*coefs_ana_Q24.offset(i as isize) >> 1 as libc::c_int)
                            + (*coefs_ana_Q24.offset(i as isize) & 1 as libc::c_int)
                    } else {
                        (*coefs_ana_Q24.offset(i as isize)
                            >> 16 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                            >> 1 as libc::c_int
                    });
            i += 1;
            i;
        }
        iter += 1;
        iter;
    }
}
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_noise_shape_analysis_FIX(
    mut psEnc: *mut SKP_Silk_encoder_state_FIX,
    mut psEncCtrl: *mut SKP_Silk_encoder_control_FIX,
    mut pitch_res: *const libc::c_short,
    mut x: *const libc::c_short,
) {
    let mut psShapeSt: *mut SKP_Silk_shape_state_FIX = &mut (*psEnc).sShape;
    let mut k: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut nSamples: libc::c_int = 0;
    let mut Qnrg: libc::c_int = 0;
    let mut b_Q14: libc::c_int = 0;
    let mut warping_Q16: libc::c_int = 0;
    let mut scale: libc::c_int = 0 as libc::c_int;
    let mut SNR_adj_dB_Q7: libc::c_int = 0;
    let mut HarmBoost_Q16: libc::c_int = 0;
    let mut HarmShapeGain_Q16: libc::c_int = 0;
    let mut Tilt_Q16: libc::c_int = 0;
    let mut tmp32: libc::c_int = 0;
    let mut nrg: libc::c_int = 0;
    let mut pre_nrg_Q30: libc::c_int = 0;
    let mut log_energy_Q7: libc::c_int = 0;
    let mut log_energy_prev_Q7: libc::c_int = 0;
    let mut energy_variation_Q7: libc::c_int = 0;
    let mut delta_Q16: libc::c_int = 0;
    let mut BWExp1_Q16: libc::c_int = 0;
    let mut BWExp2_Q16: libc::c_int = 0;
    let mut gain_mult_Q16: libc::c_int = 0;
    let mut gain_add_Q16: libc::c_int = 0;
    let mut strength_Q16: libc::c_int = 0;
    let mut b_Q8: libc::c_int = 0;
    let mut auto_corr: [libc::c_int; 17] = [0; 17];
    let mut refl_coef_Q16: [libc::c_int; 16] = [0; 16];
    let mut AR1_Q24: [libc::c_int; 16] = [0; 16];
    let mut AR2_Q24: [libc::c_int; 16] = [0; 16];
    let mut x_windowed: [libc::c_short; 360] = [0; 360];
    let mut x_ptr: *const libc::c_short = 0 as *const libc::c_short;
    let mut pitch_res_ptr: *const libc::c_short = 0 as *const libc::c_short;
    x_ptr = x.offset(-((*psEnc).sCmn.la_shape as isize));
    (*psEncCtrl)
        .current_SNR_dB_Q7 = (*psEnc).SNR_dB_Q7
        - (((*psEnc).BufferedInChannel_ms << 7 as libc::c_int >> 16 as libc::c_int)
            * (0.05f64
                * ((1 as libc::c_int as int64_t) << 16 as libc::c_int) as libc::c_double
                + 0.5f64) as libc::c_int as libc::c_short as libc::c_int
            + (((*psEnc).BufferedInChannel_ms << 7 as libc::c_int
                & 0xffff as libc::c_int)
                * (0.05f64
                    * ((1 as libc::c_int as int64_t) << 16 as libc::c_int)
                        as libc::c_double + 0.5f64) as libc::c_int as libc::c_short
                    as libc::c_int >> 16 as libc::c_int));
    if (*psEnc).speech_activity_Q8
        > ((0.5f32
            * ((1 as libc::c_int as int64_t) << 8 as libc::c_int) as libc::c_float)
            as libc::c_double + 0.5f64) as libc::c_int
    {
        (*psEncCtrl).current_SNR_dB_Q7
            -= (*psEnc).inBandFEC_SNR_comp_Q8 >> 1 as libc::c_int;
    }
    (*psEncCtrl)
        .input_quality_Q14 = (*psEncCtrl)
        .input_quality_bands_Q15[0 as libc::c_int as usize]
        + (*psEncCtrl).input_quality_bands_Q15[1 as libc::c_int as usize]
        >> 2 as libc::c_int;
    (*psEncCtrl)
        .coding_quality_Q14 = SKP_Silk_sigm_Q15(
        (if 4 as libc::c_int == 1 as libc::c_int {
            ((*psEncCtrl).current_SNR_dB_Q7
                - (18.0f64
                    * ((1 as libc::c_int as int64_t) << 7 as libc::c_int)
                        as libc::c_double + 0.5f64) as libc::c_int >> 1 as libc::c_int)
                + ((*psEncCtrl).current_SNR_dB_Q7
                    - (18.0f64
                        * ((1 as libc::c_int as int64_t) << 7 as libc::c_int)
                            as libc::c_double + 0.5f64) as libc::c_int
                    & 1 as libc::c_int)
        } else {
            ((*psEncCtrl).current_SNR_dB_Q7
                - (18.0f64
                    * ((1 as libc::c_int as int64_t) << 7 as libc::c_int)
                        as libc::c_double + 0.5f64) as libc::c_int
                >> 4 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                >> 1 as libc::c_int
        }),
    ) >> 1 as libc::c_int;
    b_Q8 = (1.0f64
        * ((1 as libc::c_int as int64_t) << 8 as libc::c_int) as libc::c_double + 0.5f64)
        as libc::c_int - (*psEnc).speech_activity_Q8;
    b_Q8 = (b_Q8 << 8 as libc::c_int >> 16 as libc::c_int)
        * b_Q8 as libc::c_short as libc::c_int
        + ((b_Q8 << 8 as libc::c_int & 0xffff as libc::c_int)
            * b_Q8 as libc::c_short as libc::c_int >> 16 as libc::c_int);
    SNR_adj_dB_Q7 = (*psEncCtrl).current_SNR_dB_Q7
        + (((((-4.0f32
            * ((1 as libc::c_int as int64_t) << 7 as libc::c_int) as libc::c_float)
            as libc::c_double + 0.5f64) as libc::c_int
            >> 4 as libc::c_int + 1 as libc::c_int) as libc::c_short as libc::c_int
            * b_Q8 as libc::c_short as libc::c_int >> 16 as libc::c_int)
            * (((1.0f64
                * ((1 as libc::c_int as int64_t) << 14 as libc::c_int) as libc::c_double
                + 0.5f64) as libc::c_int + (*psEncCtrl).input_quality_Q14
                >> 16 as libc::c_int)
                * (*psEncCtrl).coding_quality_Q14 as libc::c_short as libc::c_int
                + (((1.0f64
                    * ((1 as libc::c_int as int64_t) << 14 as libc::c_int)
                        as libc::c_double + 0.5f64) as libc::c_int
                    + (*psEncCtrl).input_quality_Q14 & 0xffff as libc::c_int)
                    * (*psEncCtrl).coding_quality_Q14 as libc::c_short as libc::c_int
                    >> 16 as libc::c_int)) as libc::c_short as libc::c_int
            + (((((-4.0f32
                * ((1 as libc::c_int as int64_t) << 7 as libc::c_int) as libc::c_float)
                as libc::c_double + 0.5f64) as libc::c_int
                >> 4 as libc::c_int + 1 as libc::c_int) as libc::c_short as libc::c_int
                * b_Q8 as libc::c_short as libc::c_int & 0xffff as libc::c_int)
                * (((1.0f64
                    * ((1 as libc::c_int as int64_t) << 14 as libc::c_int)
                        as libc::c_double + 0.5f64) as libc::c_int
                    + (*psEncCtrl).input_quality_Q14 >> 16 as libc::c_int)
                    * (*psEncCtrl).coding_quality_Q14 as libc::c_short as libc::c_int
                    + (((1.0f64
                        * ((1 as libc::c_int as int64_t) << 14 as libc::c_int)
                            as libc::c_double + 0.5f64) as libc::c_int
                        + (*psEncCtrl).input_quality_Q14 & 0xffff as libc::c_int)
                        * (*psEncCtrl).coding_quality_Q14 as libc::c_short as libc::c_int
                        >> 16 as libc::c_int)) as libc::c_short as libc::c_int
                >> 16 as libc::c_int));
    if (*psEncCtrl).sCmn.sigtype == 0 as libc::c_int {
        SNR_adj_dB_Q7 = SNR_adj_dB_Q7
            + ((((2.0f32
                * ((1 as libc::c_int as int64_t) << 8 as libc::c_int) as libc::c_float)
                as libc::c_double + 0.5f64) as libc::c_int >> 16 as libc::c_int)
                * (*psEnc).LTPCorr_Q15 as libc::c_short as libc::c_int
                + ((((2.0f32
                    * ((1 as libc::c_int as int64_t) << 8 as libc::c_int)
                        as libc::c_float) as libc::c_double + 0.5f64) as libc::c_int
                    & 0xffff as libc::c_int)
                    * (*psEnc).LTPCorr_Q15 as libc::c_short as libc::c_int
                    >> 16 as libc::c_int));
    } else {
        SNR_adj_dB_Q7 = SNR_adj_dB_Q7
            + (((6.0f64
                * ((1 as libc::c_int as int64_t) << 9 as libc::c_int) as libc::c_double
                + 0.5f64) as libc::c_int
                + ((-((0.4f64
                    * ((1 as libc::c_int as int64_t) << 18 as libc::c_int)
                        as libc::c_double + 0.5f64) as libc::c_int) >> 16 as libc::c_int)
                    * (*psEncCtrl).current_SNR_dB_Q7 as libc::c_short as libc::c_int
                    + ((-((0.4f64
                        * ((1 as libc::c_int as int64_t) << 18 as libc::c_int)
                            as libc::c_double + 0.5f64) as libc::c_int)
                        & 0xffff as libc::c_int)
                        * (*psEncCtrl).current_SNR_dB_Q7 as libc::c_short as libc::c_int
                        >> 16 as libc::c_int)) >> 16 as libc::c_int)
                * ((1.0f64
                    * ((1 as libc::c_int as int64_t) << 14 as libc::c_int)
                        as libc::c_double + 0.5f64) as libc::c_int
                    - (*psEncCtrl).input_quality_Q14) as libc::c_short as libc::c_int
                + (((6.0f64
                    * ((1 as libc::c_int as int64_t) << 9 as libc::c_int)
                        as libc::c_double + 0.5f64) as libc::c_int
                    + ((-((0.4f64
                        * ((1 as libc::c_int as int64_t) << 18 as libc::c_int)
                            as libc::c_double + 0.5f64) as libc::c_int)
                        >> 16 as libc::c_int)
                        * (*psEncCtrl).current_SNR_dB_Q7 as libc::c_short as libc::c_int
                        + ((-((0.4f64
                            * ((1 as libc::c_int as int64_t) << 18 as libc::c_int)
                                as libc::c_double + 0.5f64) as libc::c_int)
                            & 0xffff as libc::c_int)
                            * (*psEncCtrl).current_SNR_dB_Q7 as libc::c_short
                                as libc::c_int >> 16 as libc::c_int))
                    & 0xffff as libc::c_int)
                    * ((1.0f64
                        * ((1 as libc::c_int as int64_t) << 14 as libc::c_int)
                            as libc::c_double + 0.5f64) as libc::c_int
                        - (*psEncCtrl).input_quality_Q14) as libc::c_short as libc::c_int
                    >> 16 as libc::c_int));
    }
    if (*psEncCtrl).sCmn.sigtype == 0 as libc::c_int {
        (*psEncCtrl).sCmn.QuantOffsetType = 0 as libc::c_int;
        (*psEncCtrl).sparseness_Q8 = 0 as libc::c_int;
    } else {
        nSamples = (*psEnc).sCmn.fs_kHz << 1 as libc::c_int;
        energy_variation_Q7 = 0 as libc::c_int;
        log_energy_prev_Q7 = 0 as libc::c_int;
        pitch_res_ptr = pitch_res;
        k = 0 as libc::c_int;
        while k < 20 as libc::c_int / 2 as libc::c_int {
            SKP_Silk_sum_sqr_shift(&mut nrg, &mut scale, pitch_res_ptr, nSamples);
            nrg += nSamples >> scale;
            log_energy_Q7 = SKP_Silk_lin2log(nrg);
            if k > 0 as libc::c_int {
                energy_variation_Q7
                    += if log_energy_Q7 - log_energy_prev_Q7 > 0 as libc::c_int {
                        log_energy_Q7 - log_energy_prev_Q7
                    } else {
                        -(log_energy_Q7 - log_energy_prev_Q7)
                    };
            }
            log_energy_prev_Q7 = log_energy_Q7;
            pitch_res_ptr = pitch_res_ptr.offset(nSamples as isize);
            k += 1;
            k;
        }
        (*psEncCtrl)
            .sparseness_Q8 = SKP_Silk_sigm_Q15(
            (energy_variation_Q7
                - (5.0f64
                    * ((1 as libc::c_int as int64_t) << 7 as libc::c_int)
                        as libc::c_double + 0.5f64) as libc::c_int >> 16 as libc::c_int)
                * (0.1f64
                    * ((1 as libc::c_int as int64_t) << 16 as libc::c_int)
                        as libc::c_double + 0.5f64) as libc::c_int as libc::c_short
                    as libc::c_int
                + ((energy_variation_Q7
                    - (5.0f64
                        * ((1 as libc::c_int as int64_t) << 7 as libc::c_int)
                            as libc::c_double + 0.5f64) as libc::c_int
                    & 0xffff as libc::c_int)
                    * (0.1f64
                        * ((1 as libc::c_int as int64_t) << 16 as libc::c_int)
                            as libc::c_double + 0.5f64) as libc::c_int as libc::c_short
                        as libc::c_int >> 16 as libc::c_int),
        ) >> 7 as libc::c_int;
        if (*psEncCtrl).sparseness_Q8
            > ((0.75f32
                * ((1 as libc::c_int as int64_t) << 8 as libc::c_int) as libc::c_float)
                as libc::c_double + 0.5f64) as libc::c_int
        {
            (*psEncCtrl).sCmn.QuantOffsetType = 0 as libc::c_int;
        } else {
            (*psEncCtrl).sCmn.QuantOffsetType = 1 as libc::c_int;
        }
        SNR_adj_dB_Q7 = SNR_adj_dB_Q7
            + ((((2.0f32
                * ((1 as libc::c_int as int64_t) << 15 as libc::c_int) as libc::c_float)
                as libc::c_double + 0.5f64) as libc::c_int >> 16 as libc::c_int)
                * ((*psEncCtrl).sparseness_Q8
                    - (0.5f64
                        * ((1 as libc::c_int as int64_t) << 8 as libc::c_int)
                            as libc::c_double + 0.5f64) as libc::c_int) as libc::c_short
                    as libc::c_int
                + ((((2.0f32
                    * ((1 as libc::c_int as int64_t) << 15 as libc::c_int)
                        as libc::c_float) as libc::c_double + 0.5f64) as libc::c_int
                    & 0xffff as libc::c_int)
                    * ((*psEncCtrl).sparseness_Q8
                        - (0.5f64
                            * ((1 as libc::c_int as int64_t) << 8 as libc::c_int)
                                as libc::c_double + 0.5f64) as libc::c_int) as libc::c_short
                        as libc::c_int >> 16 as libc::c_int));
    }
    strength_Q16 = ((*psEncCtrl).predGain_Q16 >> 16 as libc::c_int)
        * ((1e-3f32
            * ((1 as libc::c_int as int64_t) << 16 as libc::c_int) as libc::c_float)
            as libc::c_double + 0.5f64) as libc::c_int as libc::c_short as libc::c_int
        + (((*psEncCtrl).predGain_Q16 & 0xffff as libc::c_int)
            * ((1e-3f32
                * ((1 as libc::c_int as int64_t) << 16 as libc::c_int) as libc::c_float)
                as libc::c_double + 0.5f64) as libc::c_int as libc::c_short
                as libc::c_int >> 16 as libc::c_int);
    BWExp2_Q16 = SKP_DIV32_varQ(
        ((0.95f32
            * ((1 as libc::c_int as int64_t) << 16 as libc::c_int) as libc::c_float)
            as libc::c_double + 0.5f64) as libc::c_int,
        (1.0f64 * ((1 as libc::c_int as int64_t) << 16 as libc::c_int) as libc::c_double
            + 0.5f64) as libc::c_int
            + ((strength_Q16 >> 16 as libc::c_int)
                * strength_Q16 as libc::c_short as libc::c_int
                + ((strength_Q16 & 0xffff as libc::c_int)
                    * strength_Q16 as libc::c_short as libc::c_int >> 16 as libc::c_int))
            + strength_Q16
                * (if 16 as libc::c_int == 1 as libc::c_int {
                    (strength_Q16 >> 1 as libc::c_int)
                        + (strength_Q16 & 1 as libc::c_int)
                } else {
                    (strength_Q16 >> 16 as libc::c_int - 1 as libc::c_int)
                        + 1 as libc::c_int >> 1 as libc::c_int
                }),
        16 as libc::c_int,
    );
    BWExp1_Q16 = BWExp2_Q16;
    delta_Q16 = ((1.0f64
        * ((1 as libc::c_int as int64_t) << 16 as libc::c_int) as libc::c_double
        + 0.5f64) as libc::c_int
        - 3 as libc::c_int as libc::c_short as libc::c_int
            * (*psEncCtrl).coding_quality_Q14 as libc::c_short as libc::c_int
        >> 16 as libc::c_int)
        * ((0.01f32
            * ((1 as libc::c_int as int64_t) << 16 as libc::c_int) as libc::c_float)
            as libc::c_double + 0.5f64) as libc::c_int as libc::c_short as libc::c_int
        + (((1.0f64
            * ((1 as libc::c_int as int64_t) << 16 as libc::c_int) as libc::c_double
            + 0.5f64) as libc::c_int
            - 3 as libc::c_int as libc::c_short as libc::c_int
                * (*psEncCtrl).coding_quality_Q14 as libc::c_short as libc::c_int
            & 0xffff as libc::c_int)
            * ((0.01f32
                * ((1 as libc::c_int as int64_t) << 16 as libc::c_int) as libc::c_float)
                as libc::c_double + 0.5f64) as libc::c_int as libc::c_short
                as libc::c_int >> 16 as libc::c_int);
    BWExp1_Q16 = BWExp1_Q16 - delta_Q16;
    BWExp2_Q16 = BWExp2_Q16 + delta_Q16;
    BWExp1_Q16 = (BWExp1_Q16 << 14 as libc::c_int) / (BWExp2_Q16 >> 2 as libc::c_int);
    if (*psEnc).sCmn.warping_Q16 > 0 as libc::c_int {
        warping_Q16 = (*psEnc).sCmn.warping_Q16
            + (((*psEncCtrl).coding_quality_Q14 >> 16 as libc::c_int)
                * (0.01f64
                    * ((1 as libc::c_int as int64_t) << 18 as libc::c_int)
                        as libc::c_double + 0.5f64) as libc::c_int as libc::c_short
                    as libc::c_int
                + (((*psEncCtrl).coding_quality_Q14 & 0xffff as libc::c_int)
                    * (0.01f64
                        * ((1 as libc::c_int as int64_t) << 18 as libc::c_int)
                            as libc::c_double + 0.5f64) as libc::c_int as libc::c_short
                        as libc::c_int >> 16 as libc::c_int));
    } else {
        warping_Q16 = 0 as libc::c_int;
    }
    k = 0 as libc::c_int;
    while k < 4 as libc::c_int {
        let mut shift: libc::c_int = 0;
        let mut slope_part: libc::c_int = 0;
        let mut flat_part: libc::c_int = 0;
        flat_part = (*psEnc).sCmn.fs_kHz * 5 as libc::c_int;
        slope_part = (*psEnc).sCmn.shapeWinLength - flat_part >> 1 as libc::c_int;
        SKP_Silk_apply_sine_window(
            x_windowed.as_mut_ptr(),
            x_ptr,
            1 as libc::c_int,
            slope_part,
        );
        shift = slope_part;
        memcpy(
            x_windowed.as_mut_ptr().offset(shift as isize) as *mut libc::c_void,
            x_ptr.offset(shift as isize) as *const libc::c_void,
            (flat_part as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_short>() as libc::c_ulong),
        );
        shift += flat_part;
        SKP_Silk_apply_sine_window(
            x_windowed.as_mut_ptr().offset(shift as isize),
            x_ptr.offset(shift as isize),
            2 as libc::c_int,
            slope_part,
        );
        x_ptr = x_ptr.offset((*psEnc).sCmn.subfr_length as isize);
        if (*psEnc).sCmn.warping_Q16 > 0 as libc::c_int {
            SKP_Silk_warped_autocorrelation_FIX(
                auto_corr.as_mut_ptr(),
                &mut scale,
                x_windowed.as_mut_ptr(),
                warping_Q16 as libc::c_short,
                (*psEnc).sCmn.shapeWinLength,
                (*psEnc).sCmn.shapingLPCOrder,
            );
        } else {
            SKP_Silk_autocorr(
                auto_corr.as_mut_ptr(),
                &mut scale,
                x_windowed.as_mut_ptr(),
                (*psEnc).sCmn.shapeWinLength,
                (*psEnc).sCmn.shapingLPCOrder + 1 as libc::c_int,
            );
        }
        auto_corr[0 as libc::c_int
            as usize] = auto_corr[0 as libc::c_int as usize]
            + SKP_max_32(
                (auto_corr[0 as libc::c_int as usize] >> 4 as libc::c_int
                    >> 16 as libc::c_int)
                    * ((1e-5f32
                        * ((1 as libc::c_int as int64_t) << 20 as libc::c_int)
                            as libc::c_float) as libc::c_double + 0.5f64) as libc::c_int
                        as libc::c_short as libc::c_int
                    + ((auto_corr[0 as libc::c_int as usize] >> 4 as libc::c_int
                        & 0xffff as libc::c_int)
                        * ((1e-5f32
                            * ((1 as libc::c_int as int64_t) << 20 as libc::c_int)
                                as libc::c_float) as libc::c_double + 0.5f64) as libc::c_int
                            as libc::c_short as libc::c_int >> 16 as libc::c_int),
                1 as libc::c_int,
            );
        nrg = SKP_Silk_schur64(
            refl_coef_Q16.as_mut_ptr(),
            auto_corr.as_mut_ptr() as *const libc::c_int,
            (*psEnc).sCmn.shapingLPCOrder,
        );
        SKP_Silk_k2a_Q16(
            AR2_Q24.as_mut_ptr(),
            refl_coef_Q16.as_mut_ptr(),
            (*psEnc).sCmn.shapingLPCOrder,
        );
        Qnrg = -scale;
        if Qnrg & 1 as libc::c_int != 0 {
            Qnrg -= 1 as libc::c_int;
            nrg >>= 1 as libc::c_int;
        }
        tmp32 = SKP_Silk_SQRT_APPROX(nrg);
        Qnrg >>= 1 as libc::c_int;
        (*psEncCtrl)
            .Gains_Q16[k
            as usize] = (if 0x80000000 as libc::c_uint as libc::c_int
            >> 16 as libc::c_int - Qnrg
            > 0x7fffffff as libc::c_int >> 16 as libc::c_int - Qnrg
        {
            (if tmp32
                > 0x80000000 as libc::c_uint as libc::c_int >> 16 as libc::c_int - Qnrg
            {
                0x80000000 as libc::c_uint as libc::c_int >> 16 as libc::c_int - Qnrg
            } else {
                (if tmp32 < 0x7fffffff as libc::c_int >> 16 as libc::c_int - Qnrg {
                    0x7fffffff as libc::c_int >> 16 as libc::c_int - Qnrg
                } else {
                    tmp32
                })
            })
        } else {
            (if tmp32 > 0x7fffffff as libc::c_int >> 16 as libc::c_int - Qnrg {
                0x7fffffff as libc::c_int >> 16 as libc::c_int - Qnrg
            } else {
                (if tmp32
                    < 0x80000000 as libc::c_uint as libc::c_int
                        >> 16 as libc::c_int - Qnrg
                {
                    0x80000000 as libc::c_uint as libc::c_int >> 16 as libc::c_int - Qnrg
                } else {
                    tmp32
                })
            })
        }) << 16 as libc::c_int - Qnrg;
        if (*psEnc).sCmn.warping_Q16 > 0 as libc::c_int {
            gain_mult_Q16 = warped_gain(
                AR2_Q24.as_mut_ptr(),
                warping_Q16,
                (*psEnc).sCmn.shapingLPCOrder,
            );
            (*psEncCtrl)
                .Gains_Q16[k
                as usize] = ((*psEncCtrl).Gains_Q16[k as usize] >> 16 as libc::c_int)
                * gain_mult_Q16 as libc::c_short as libc::c_int
                + (((*psEncCtrl).Gains_Q16[k as usize] & 0xffff as libc::c_int)
                    * gain_mult_Q16 as libc::c_short as libc::c_int >> 16 as libc::c_int)
                + (*psEncCtrl).Gains_Q16[k as usize]
                    * (if 16 as libc::c_int == 1 as libc::c_int {
                        (gain_mult_Q16 >> 1 as libc::c_int)
                            + (gain_mult_Q16 & 1 as libc::c_int)
                    } else {
                        (gain_mult_Q16 >> 16 as libc::c_int - 1 as libc::c_int)
                            + 1 as libc::c_int >> 1 as libc::c_int
                    });
            if (*psEncCtrl).Gains_Q16[k as usize] < 0 as libc::c_int {
                (*psEncCtrl).Gains_Q16[k as usize] = 0x7fffffff as libc::c_int;
            }
        }
        SKP_Silk_bwexpander_32(
            AR2_Q24.as_mut_ptr(),
            (*psEnc).sCmn.shapingLPCOrder,
            BWExp2_Q16,
        );
        memcpy(
            AR1_Q24.as_mut_ptr() as *mut libc::c_void,
            AR2_Q24.as_mut_ptr() as *const libc::c_void,
            ((*psEnc).sCmn.shapingLPCOrder as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
        );
        SKP_Silk_bwexpander_32(
            AR1_Q24.as_mut_ptr(),
            (*psEnc).sCmn.shapingLPCOrder,
            BWExp1_Q16,
        );
        SKP_Silk_LPC_inverse_pred_gain_Q24(
            &mut pre_nrg_Q30,
            AR2_Q24.as_mut_ptr(),
            (*psEnc).sCmn.shapingLPCOrder,
        );
        SKP_Silk_LPC_inverse_pred_gain_Q24(
            &mut nrg,
            AR1_Q24.as_mut_ptr(),
            (*psEnc).sCmn.shapingLPCOrder,
        );
        pre_nrg_Q30 = (pre_nrg_Q30 >> 16 as libc::c_int)
            * (0.7f64
                * ((1 as libc::c_int as int64_t) << 15 as libc::c_int) as libc::c_double
                + 0.5f64) as libc::c_int as libc::c_short as libc::c_int
            + ((pre_nrg_Q30 & 0xffff as libc::c_int)
                * (0.7f64
                    * ((1 as libc::c_int as int64_t) << 15 as libc::c_int)
                        as libc::c_double + 0.5f64) as libc::c_int as libc::c_short
                    as libc::c_int >> 16 as libc::c_int) << 1 as libc::c_int;
        (*psEncCtrl)
            .GainsPre_Q14[k
            as usize] = (0.3f64
            * ((1 as libc::c_int as int64_t) << 14 as libc::c_int) as libc::c_double
            + 0.5f64) as libc::c_int
            + SKP_DIV32_varQ(pre_nrg_Q30, nrg, 14 as libc::c_int);
        limit_warped_coefs(
            AR2_Q24.as_mut_ptr(),
            AR1_Q24.as_mut_ptr(),
            warping_Q16,
            (3.999f64
                * ((1 as libc::c_int as int64_t) << 24 as libc::c_int) as libc::c_double
                + 0.5f64) as libc::c_int,
            (*psEnc).sCmn.shapingLPCOrder,
        );
        i = 0 as libc::c_int;
        while i < (*psEnc).sCmn.shapingLPCOrder {
            (*psEncCtrl)
                .AR1_Q13[(k * 16 as libc::c_int + i)
                as usize] = (if (if 11 as libc::c_int == 1 as libc::c_int {
                (AR1_Q24[i as usize] >> 1 as libc::c_int)
                    + (AR1_Q24[i as usize] & 1 as libc::c_int)
            } else {
                (AR1_Q24[i as usize] >> 11 as libc::c_int - 1 as libc::c_int)
                    + 1 as libc::c_int >> 1 as libc::c_int
            }) > 0x7fff as libc::c_int
            {
                0x7fff as libc::c_int
            } else if (if 11 as libc::c_int == 1 as libc::c_int {
                (AR1_Q24[i as usize] >> 1 as libc::c_int)
                    + (AR1_Q24[i as usize] & 1 as libc::c_int)
            } else {
                (AR1_Q24[i as usize] >> 11 as libc::c_int - 1 as libc::c_int)
                    + 1 as libc::c_int >> 1 as libc::c_int
            }) < 0x8000 as libc::c_int as libc::c_short as libc::c_int
            {
                0x8000 as libc::c_int as libc::c_short as libc::c_int
            } else if 11 as libc::c_int == 1 as libc::c_int {
                (AR1_Q24[i as usize] >> 1 as libc::c_int)
                    + (AR1_Q24[i as usize] & 1 as libc::c_int)
            } else {
                (AR1_Q24[i as usize] >> 11 as libc::c_int - 1 as libc::c_int)
                    + 1 as libc::c_int >> 1 as libc::c_int
            }) as libc::c_short;
            (*psEncCtrl)
                .AR2_Q13[(k * 16 as libc::c_int + i)
                as usize] = (if (if 11 as libc::c_int == 1 as libc::c_int {
                (AR2_Q24[i as usize] >> 1 as libc::c_int)
                    + (AR2_Q24[i as usize] & 1 as libc::c_int)
            } else {
                (AR2_Q24[i as usize] >> 11 as libc::c_int - 1 as libc::c_int)
                    + 1 as libc::c_int >> 1 as libc::c_int
            }) > 0x7fff as libc::c_int
            {
                0x7fff as libc::c_int
            } else if (if 11 as libc::c_int == 1 as libc::c_int {
                (AR2_Q24[i as usize] >> 1 as libc::c_int)
                    + (AR2_Q24[i as usize] & 1 as libc::c_int)
            } else {
                (AR2_Q24[i as usize] >> 11 as libc::c_int - 1 as libc::c_int)
                    + 1 as libc::c_int >> 1 as libc::c_int
            }) < 0x8000 as libc::c_int as libc::c_short as libc::c_int
            {
                0x8000 as libc::c_int as libc::c_short as libc::c_int
            } else if 11 as libc::c_int == 1 as libc::c_int {
                (AR2_Q24[i as usize] >> 1 as libc::c_int)
                    + (AR2_Q24[i as usize] & 1 as libc::c_int)
            } else {
                (AR2_Q24[i as usize] >> 11 as libc::c_int - 1 as libc::c_int)
                    + 1 as libc::c_int >> 1 as libc::c_int
            }) as libc::c_short;
            i += 1;
            i;
        }
        k += 1;
        k;
    }
    gain_mult_Q16 = SKP_Silk_log2lin(
        -(-((16.0f64
            * ((1 as libc::c_int as int64_t) << 7 as libc::c_int) as libc::c_double
            + 0.5f64) as libc::c_int)
            + ((SNR_adj_dB_Q7 >> 16 as libc::c_int)
                * (0.16f64
                    * ((1 as libc::c_int as int64_t) << 16 as libc::c_int)
                        as libc::c_double + 0.5f64) as libc::c_int as libc::c_short
                    as libc::c_int
                + ((SNR_adj_dB_Q7 & 0xffff as libc::c_int)
                    * (0.16f64
                        * ((1 as libc::c_int as int64_t) << 16 as libc::c_int)
                            as libc::c_double + 0.5f64) as libc::c_int as libc::c_short
                        as libc::c_int >> 16 as libc::c_int))),
    );
    gain_add_Q16 = SKP_Silk_log2lin(
        (16.0f64 * ((1 as libc::c_int as int64_t) << 7 as libc::c_int) as libc::c_double
            + 0.5f64) as libc::c_int
            + ((((4.0f32
                * ((1 as libc::c_int as int64_t) << 7 as libc::c_int) as libc::c_float)
                as libc::c_double + 0.5f64) as libc::c_int >> 16 as libc::c_int)
                * (0.16f64
                    * ((1 as libc::c_int as int64_t) << 16 as libc::c_int)
                        as libc::c_double + 0.5f64) as libc::c_int as libc::c_short
                    as libc::c_int
                + ((((4.0f32
                    * ((1 as libc::c_int as int64_t) << 7 as libc::c_int)
                        as libc::c_float) as libc::c_double + 0.5f64) as libc::c_int
                    & 0xffff as libc::c_int)
                    * (0.16f64
                        * ((1 as libc::c_int as int64_t) << 16 as libc::c_int)
                            as libc::c_double + 0.5f64) as libc::c_int as libc::c_short
                        as libc::c_int >> 16 as libc::c_int)),
    );
    tmp32 = SKP_Silk_log2lin(
        (16.0f64 * ((1 as libc::c_int as int64_t) << 7 as libc::c_int) as libc::c_double
            + 0.5f64) as libc::c_int
            + ((((-50.0f32
                * ((1 as libc::c_int as int64_t) << 7 as libc::c_int) as libc::c_float)
                as libc::c_double + 0.5f64) as libc::c_int >> 16 as libc::c_int)
                * (0.16f64
                    * ((1 as libc::c_int as int64_t) << 16 as libc::c_int)
                        as libc::c_double + 0.5f64) as libc::c_int as libc::c_short
                    as libc::c_int
                + ((((-50.0f32
                    * ((1 as libc::c_int as int64_t) << 7 as libc::c_int)
                        as libc::c_float) as libc::c_double + 0.5f64) as libc::c_int
                    & 0xffff as libc::c_int)
                    * (0.16f64
                        * ((1 as libc::c_int as int64_t) << 16 as libc::c_int)
                            as libc::c_double + 0.5f64) as libc::c_int as libc::c_short
                        as libc::c_int >> 16 as libc::c_int)),
    );
    tmp32 = ((*psEnc).avgGain_Q16 >> 16 as libc::c_int)
        * tmp32 as libc::c_short as libc::c_int
        + (((*psEnc).avgGain_Q16 & 0xffff as libc::c_int)
            * tmp32 as libc::c_short as libc::c_int >> 16 as libc::c_int)
        + (*psEnc).avgGain_Q16
            * (if 16 as libc::c_int == 1 as libc::c_int {
                (tmp32 >> 1 as libc::c_int) + (tmp32 & 1 as libc::c_int)
            } else {
                (tmp32 >> 16 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                    >> 1 as libc::c_int
            });
    gain_add_Q16 = if (gain_add_Q16 + tmp32) as libc::c_uint & 0x80000000 as libc::c_uint
        == 0 as libc::c_int as libc::c_uint
    {
        if (gain_add_Q16 & tmp32) as libc::c_uint & 0x80000000 as libc::c_uint
            != 0 as libc::c_int as libc::c_uint
        {
            0x80000000 as libc::c_uint as libc::c_int
        } else {
            gain_add_Q16 + tmp32
        }
    } else if (gain_add_Q16 | tmp32) as libc::c_uint & 0x80000000 as libc::c_uint
        == 0 as libc::c_int as libc::c_uint
    {
        0x7fffffff as libc::c_int
    } else {
        gain_add_Q16 + tmp32
    };
    k = 0 as libc::c_int;
    while k < 4 as libc::c_int {
        (*psEncCtrl)
            .Gains_Q16[k
            as usize] = ((*psEncCtrl).Gains_Q16[k as usize] >> 16 as libc::c_int)
            * gain_mult_Q16 as libc::c_short as libc::c_int
            + (((*psEncCtrl).Gains_Q16[k as usize] & 0xffff as libc::c_int)
                * gain_mult_Q16 as libc::c_short as libc::c_int >> 16 as libc::c_int)
            + (*psEncCtrl).Gains_Q16[k as usize]
                * (if 16 as libc::c_int == 1 as libc::c_int {
                    (gain_mult_Q16 >> 1 as libc::c_int)
                        + (gain_mult_Q16 & 1 as libc::c_int)
                } else {
                    (gain_mult_Q16 >> 16 as libc::c_int - 1 as libc::c_int)
                        + 1 as libc::c_int >> 1 as libc::c_int
                });
        if (*psEncCtrl).Gains_Q16[k as usize] < 0 as libc::c_int {
            (*psEncCtrl).Gains_Q16[k as usize] = 0x7fffffff as libc::c_int;
        }
        k += 1;
        k;
    }
    k = 0 as libc::c_int;
    while k < 4 as libc::c_int {
        (*psEncCtrl)
            .Gains_Q16[k
            as usize] = if ((*psEncCtrl).Gains_Q16[k as usize] + gain_add_Q16)
            as libc::c_uint & 0x80000000 as libc::c_uint != 0
        {
            0x7fffffff as libc::c_int
        } else {
            (*psEncCtrl).Gains_Q16[k as usize] + gain_add_Q16
        };
        (*psEnc)
            .avgGain_Q16 = if ((*psEnc).avgGain_Q16
            + (((*psEncCtrl).Gains_Q16[k as usize] - (*psEnc).avgGain_Q16
                >> 16 as libc::c_int)
                * (if 2 as libc::c_int == 1 as libc::c_int {
                    ((*psEnc).speech_activity_Q8 as libc::c_short as libc::c_int
                        * ((1e-3f32
                            * ((1 as libc::c_int as int64_t) << 10 as libc::c_int)
                                as libc::c_float) as libc::c_double + 0.5f64) as libc::c_int
                            as libc::c_short as libc::c_int >> 1 as libc::c_int)
                        + ((*psEnc).speech_activity_Q8 as libc::c_short as libc::c_int
                            * ((1e-3f32
                                * ((1 as libc::c_int as int64_t) << 10 as libc::c_int)
                                    as libc::c_float) as libc::c_double + 0.5f64) as libc::c_int
                                as libc::c_short as libc::c_int & 1 as libc::c_int)
                } else {
                    ((*psEnc).speech_activity_Q8 as libc::c_short as libc::c_int
                        * ((1e-3f32
                            * ((1 as libc::c_int as int64_t) << 10 as libc::c_int)
                                as libc::c_float) as libc::c_double + 0.5f64) as libc::c_int
                            as libc::c_short as libc::c_int
                        >> 2 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                        >> 1 as libc::c_int
                }) as libc::c_short as libc::c_int
                + (((*psEncCtrl).Gains_Q16[k as usize] - (*psEnc).avgGain_Q16
                    & 0xffff as libc::c_int)
                    * (if 2 as libc::c_int == 1 as libc::c_int {
                        ((*psEnc).speech_activity_Q8 as libc::c_short as libc::c_int
                            * ((1e-3f32
                                * ((1 as libc::c_int as int64_t) << 10 as libc::c_int)
                                    as libc::c_float) as libc::c_double + 0.5f64) as libc::c_int
                                as libc::c_short as libc::c_int >> 1 as libc::c_int)
                            + ((*psEnc).speech_activity_Q8 as libc::c_short
                                as libc::c_int
                                * ((1e-3f32
                                    * ((1 as libc::c_int as int64_t) << 10 as libc::c_int)
                                        as libc::c_float) as libc::c_double + 0.5f64) as libc::c_int
                                    as libc::c_short as libc::c_int & 1 as libc::c_int)
                    } else {
                        ((*psEnc).speech_activity_Q8 as libc::c_short as libc::c_int
                            * ((1e-3f32
                                * ((1 as libc::c_int as int64_t) << 10 as libc::c_int)
                                    as libc::c_float) as libc::c_double + 0.5f64) as libc::c_int
                                as libc::c_short as libc::c_int
                            >> 2 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                            >> 1 as libc::c_int
                    }) as libc::c_short as libc::c_int >> 16 as libc::c_int)))
            as libc::c_uint & 0x80000000 as libc::c_uint
            == 0 as libc::c_int as libc::c_uint
        {
            if ((*psEnc).avgGain_Q16
                & ((*psEncCtrl).Gains_Q16[k as usize] - (*psEnc).avgGain_Q16
                    >> 16 as libc::c_int)
                    * (if 2 as libc::c_int == 1 as libc::c_int {
                        ((*psEnc).speech_activity_Q8 as libc::c_short as libc::c_int
                            * ((1e-3f32
                                * ((1 as libc::c_int as int64_t) << 10 as libc::c_int)
                                    as libc::c_float) as libc::c_double + 0.5f64) as libc::c_int
                                as libc::c_short as libc::c_int >> 1 as libc::c_int)
                            + ((*psEnc).speech_activity_Q8 as libc::c_short
                                as libc::c_int
                                * ((1e-3f32
                                    * ((1 as libc::c_int as int64_t) << 10 as libc::c_int)
                                        as libc::c_float) as libc::c_double + 0.5f64) as libc::c_int
                                    as libc::c_short as libc::c_int & 1 as libc::c_int)
                    } else {
                        ((*psEnc).speech_activity_Q8 as libc::c_short as libc::c_int
                            * ((1e-3f32
                                * ((1 as libc::c_int as int64_t) << 10 as libc::c_int)
                                    as libc::c_float) as libc::c_double + 0.5f64) as libc::c_int
                                as libc::c_short as libc::c_int
                            >> 2 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                            >> 1 as libc::c_int
                    }) as libc::c_short as libc::c_int
                    + (((*psEncCtrl).Gains_Q16[k as usize] - (*psEnc).avgGain_Q16
                        & 0xffff as libc::c_int)
                        * (if 2 as libc::c_int == 1 as libc::c_int {
                            ((*psEnc).speech_activity_Q8 as libc::c_short as libc::c_int
                                * ((1e-3f32
                                    * ((1 as libc::c_int as int64_t) << 10 as libc::c_int)
                                        as libc::c_float) as libc::c_double + 0.5f64) as libc::c_int
                                    as libc::c_short as libc::c_int >> 1 as libc::c_int)
                                + ((*psEnc).speech_activity_Q8 as libc::c_short
                                    as libc::c_int
                                    * ((1e-3f32
                                        * ((1 as libc::c_int as int64_t) << 10 as libc::c_int)
                                            as libc::c_float) as libc::c_double + 0.5f64) as libc::c_int
                                        as libc::c_short as libc::c_int & 1 as libc::c_int)
                        } else {
                            ((*psEnc).speech_activity_Q8 as libc::c_short as libc::c_int
                                * ((1e-3f32
                                    * ((1 as libc::c_int as int64_t) << 10 as libc::c_int)
                                        as libc::c_float) as libc::c_double + 0.5f64) as libc::c_int
                                    as libc::c_short as libc::c_int
                                >> 2 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                                >> 1 as libc::c_int
                        }) as libc::c_short as libc::c_int >> 16 as libc::c_int))
                as libc::c_uint & 0x80000000 as libc::c_uint
                != 0 as libc::c_int as libc::c_uint
            {
                0x80000000 as libc::c_uint as libc::c_int
            } else {
                (*psEnc).avgGain_Q16
                    + (((*psEncCtrl).Gains_Q16[k as usize] - (*psEnc).avgGain_Q16
                        >> 16 as libc::c_int)
                        * (if 2 as libc::c_int == 1 as libc::c_int {
                            ((*psEnc).speech_activity_Q8 as libc::c_short as libc::c_int
                                * ((1e-3f32
                                    * ((1 as libc::c_int as int64_t) << 10 as libc::c_int)
                                        as libc::c_float) as libc::c_double + 0.5f64) as libc::c_int
                                    as libc::c_short as libc::c_int >> 1 as libc::c_int)
                                + ((*psEnc).speech_activity_Q8 as libc::c_short
                                    as libc::c_int
                                    * ((1e-3f32
                                        * ((1 as libc::c_int as int64_t) << 10 as libc::c_int)
                                            as libc::c_float) as libc::c_double + 0.5f64) as libc::c_int
                                        as libc::c_short as libc::c_int & 1 as libc::c_int)
                        } else {
                            ((*psEnc).speech_activity_Q8 as libc::c_short as libc::c_int
                                * ((1e-3f32
                                    * ((1 as libc::c_int as int64_t) << 10 as libc::c_int)
                                        as libc::c_float) as libc::c_double + 0.5f64) as libc::c_int
                                    as libc::c_short as libc::c_int
                                >> 2 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                                >> 1 as libc::c_int
                        }) as libc::c_short as libc::c_int
                        + (((*psEncCtrl).Gains_Q16[k as usize] - (*psEnc).avgGain_Q16
                            & 0xffff as libc::c_int)
                            * (if 2 as libc::c_int == 1 as libc::c_int {
                                ((*psEnc).speech_activity_Q8 as libc::c_short as libc::c_int
                                    * ((1e-3f32
                                        * ((1 as libc::c_int as int64_t) << 10 as libc::c_int)
                                            as libc::c_float) as libc::c_double + 0.5f64) as libc::c_int
                                        as libc::c_short as libc::c_int >> 1 as libc::c_int)
                                    + ((*psEnc).speech_activity_Q8 as libc::c_short
                                        as libc::c_int
                                        * ((1e-3f32
                                            * ((1 as libc::c_int as int64_t) << 10 as libc::c_int)
                                                as libc::c_float) as libc::c_double + 0.5f64) as libc::c_int
                                            as libc::c_short as libc::c_int & 1 as libc::c_int)
                            } else {
                                ((*psEnc).speech_activity_Q8 as libc::c_short as libc::c_int
                                    * ((1e-3f32
                                        * ((1 as libc::c_int as int64_t) << 10 as libc::c_int)
                                            as libc::c_float) as libc::c_double + 0.5f64) as libc::c_int
                                        as libc::c_short as libc::c_int
                                    >> 2 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                                    >> 1 as libc::c_int
                            }) as libc::c_short as libc::c_int >> 16 as libc::c_int))
            }
        } else if ((*psEnc).avgGain_Q16
            | ((*psEncCtrl).Gains_Q16[k as usize] - (*psEnc).avgGain_Q16
                >> 16 as libc::c_int)
                * (if 2 as libc::c_int == 1 as libc::c_int {
                    ((*psEnc).speech_activity_Q8 as libc::c_short as libc::c_int
                        * ((1e-3f32
                            * ((1 as libc::c_int as int64_t) << 10 as libc::c_int)
                                as libc::c_float) as libc::c_double + 0.5f64) as libc::c_int
                            as libc::c_short as libc::c_int >> 1 as libc::c_int)
                        + ((*psEnc).speech_activity_Q8 as libc::c_short as libc::c_int
                            * ((1e-3f32
                                * ((1 as libc::c_int as int64_t) << 10 as libc::c_int)
                                    as libc::c_float) as libc::c_double + 0.5f64) as libc::c_int
                                as libc::c_short as libc::c_int & 1 as libc::c_int)
                } else {
                    ((*psEnc).speech_activity_Q8 as libc::c_short as libc::c_int
                        * ((1e-3f32
                            * ((1 as libc::c_int as int64_t) << 10 as libc::c_int)
                                as libc::c_float) as libc::c_double + 0.5f64) as libc::c_int
                            as libc::c_short as libc::c_int
                        >> 2 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                        >> 1 as libc::c_int
                }) as libc::c_short as libc::c_int
                + (((*psEncCtrl).Gains_Q16[k as usize] - (*psEnc).avgGain_Q16
                    & 0xffff as libc::c_int)
                    * (if 2 as libc::c_int == 1 as libc::c_int {
                        ((*psEnc).speech_activity_Q8 as libc::c_short as libc::c_int
                            * ((1e-3f32
                                * ((1 as libc::c_int as int64_t) << 10 as libc::c_int)
                                    as libc::c_float) as libc::c_double + 0.5f64) as libc::c_int
                                as libc::c_short as libc::c_int >> 1 as libc::c_int)
                            + ((*psEnc).speech_activity_Q8 as libc::c_short
                                as libc::c_int
                                * ((1e-3f32
                                    * ((1 as libc::c_int as int64_t) << 10 as libc::c_int)
                                        as libc::c_float) as libc::c_double + 0.5f64) as libc::c_int
                                    as libc::c_short as libc::c_int & 1 as libc::c_int)
                    } else {
                        ((*psEnc).speech_activity_Q8 as libc::c_short as libc::c_int
                            * ((1e-3f32
                                * ((1 as libc::c_int as int64_t) << 10 as libc::c_int)
                                    as libc::c_float) as libc::c_double + 0.5f64) as libc::c_int
                                as libc::c_short as libc::c_int
                            >> 2 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                            >> 1 as libc::c_int
                    }) as libc::c_short as libc::c_int >> 16 as libc::c_int))
            as libc::c_uint & 0x80000000 as libc::c_uint
            == 0 as libc::c_int as libc::c_uint
        {
            0x7fffffff as libc::c_int
        } else {
            (*psEnc).avgGain_Q16
                + (((*psEncCtrl).Gains_Q16[k as usize] - (*psEnc).avgGain_Q16
                    >> 16 as libc::c_int)
                    * (if 2 as libc::c_int == 1 as libc::c_int {
                        ((*psEnc).speech_activity_Q8 as libc::c_short as libc::c_int
                            * ((1e-3f32
                                * ((1 as libc::c_int as int64_t) << 10 as libc::c_int)
                                    as libc::c_float) as libc::c_double + 0.5f64) as libc::c_int
                                as libc::c_short as libc::c_int >> 1 as libc::c_int)
                            + ((*psEnc).speech_activity_Q8 as libc::c_short
                                as libc::c_int
                                * ((1e-3f32
                                    * ((1 as libc::c_int as int64_t) << 10 as libc::c_int)
                                        as libc::c_float) as libc::c_double + 0.5f64) as libc::c_int
                                    as libc::c_short as libc::c_int & 1 as libc::c_int)
                    } else {
                        ((*psEnc).speech_activity_Q8 as libc::c_short as libc::c_int
                            * ((1e-3f32
                                * ((1 as libc::c_int as int64_t) << 10 as libc::c_int)
                                    as libc::c_float) as libc::c_double + 0.5f64) as libc::c_int
                                as libc::c_short as libc::c_int
                            >> 2 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                            >> 1 as libc::c_int
                    }) as libc::c_short as libc::c_int
                    + (((*psEncCtrl).Gains_Q16[k as usize] - (*psEnc).avgGain_Q16
                        & 0xffff as libc::c_int)
                        * (if 2 as libc::c_int == 1 as libc::c_int {
                            ((*psEnc).speech_activity_Q8 as libc::c_short as libc::c_int
                                * ((1e-3f32
                                    * ((1 as libc::c_int as int64_t) << 10 as libc::c_int)
                                        as libc::c_float) as libc::c_double + 0.5f64) as libc::c_int
                                    as libc::c_short as libc::c_int >> 1 as libc::c_int)
                                + ((*psEnc).speech_activity_Q8 as libc::c_short
                                    as libc::c_int
                                    * ((1e-3f32
                                        * ((1 as libc::c_int as int64_t) << 10 as libc::c_int)
                                            as libc::c_float) as libc::c_double + 0.5f64) as libc::c_int
                                        as libc::c_short as libc::c_int & 1 as libc::c_int)
                        } else {
                            ((*psEnc).speech_activity_Q8 as libc::c_short as libc::c_int
                                * ((1e-3f32
                                    * ((1 as libc::c_int as int64_t) << 10 as libc::c_int)
                                        as libc::c_float) as libc::c_double + 0.5f64) as libc::c_int
                                    as libc::c_short as libc::c_int
                                >> 2 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                                >> 1 as libc::c_int
                        }) as libc::c_short as libc::c_int >> 16 as libc::c_int))
        };
        k += 1;
        k;
    }
    gain_mult_Q16 = (1.0f64
        * ((1 as libc::c_int as int64_t) << 16 as libc::c_int) as libc::c_double
        + 0.5f64) as libc::c_int
        + (if 10 as libc::c_int == 1 as libc::c_int {
            (((0.05f32
                * ((1 as libc::c_int as int64_t) << 26 as libc::c_int) as libc::c_float)
                as libc::c_double + 0.5f64) as libc::c_int
                + (*psEncCtrl).coding_quality_Q14
                    * ((0.1f32
                        * ((1 as libc::c_int as int64_t) << 12 as libc::c_int)
                            as libc::c_float) as libc::c_double + 0.5f64) as libc::c_int
                >> 1 as libc::c_int)
                + (((0.05f32
                    * ((1 as libc::c_int as int64_t) << 26 as libc::c_int)
                        as libc::c_float) as libc::c_double + 0.5f64) as libc::c_int
                    + (*psEncCtrl).coding_quality_Q14
                        * ((0.1f32
                            * ((1 as libc::c_int as int64_t) << 12 as libc::c_int)
                                as libc::c_float) as libc::c_double + 0.5f64) as libc::c_int
                    & 1 as libc::c_int)
        } else {
            (((0.05f32
                * ((1 as libc::c_int as int64_t) << 26 as libc::c_int) as libc::c_float)
                as libc::c_double + 0.5f64) as libc::c_int
                + (*psEncCtrl).coding_quality_Q14
                    * ((0.1f32
                        * ((1 as libc::c_int as int64_t) << 12 as libc::c_int)
                            as libc::c_float) as libc::c_double + 0.5f64) as libc::c_int
                >> 10 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                >> 1 as libc::c_int
        });
    if (*psEncCtrl).input_tilt_Q15 <= 0 as libc::c_int
        && (*psEncCtrl).sCmn.sigtype == 1 as libc::c_int
    {
        if (*psEnc).sCmn.fs_kHz == 24 as libc::c_int {
            let mut essStrength_Q15: libc::c_int = (-(*psEncCtrl).input_tilt_Q15
                >> 16 as libc::c_int)
                * ((*psEnc).speech_activity_Q8 as libc::c_short as libc::c_int
                    * ((1.0f64
                        * ((1 as libc::c_int as int64_t) << 8 as libc::c_int)
                            as libc::c_double + 0.5f64) as libc::c_int
                        - (*psEncCtrl).sparseness_Q8) as libc::c_short as libc::c_int)
                    as libc::c_short as libc::c_int
                + ((-(*psEncCtrl).input_tilt_Q15 & 0xffff as libc::c_int)
                    * ((*psEnc).speech_activity_Q8 as libc::c_short as libc::c_int
                        * ((1.0f64
                            * ((1 as libc::c_int as int64_t) << 8 as libc::c_int)
                                as libc::c_double + 0.5f64) as libc::c_int
                            - (*psEncCtrl).sparseness_Q8) as libc::c_short
                            as libc::c_int) as libc::c_short as libc::c_int
                    >> 16 as libc::c_int)
                + -(*psEncCtrl).input_tilt_Q15
                    * (if 16 as libc::c_int == 1 as libc::c_int {
                        ((*psEnc).speech_activity_Q8 as libc::c_short as libc::c_int
                            * ((1.0f64
                                * ((1 as libc::c_int as int64_t) << 8 as libc::c_int)
                                    as libc::c_double + 0.5f64) as libc::c_int
                                - (*psEncCtrl).sparseness_Q8) as libc::c_short
                                as libc::c_int >> 1 as libc::c_int)
                            + ((*psEnc).speech_activity_Q8 as libc::c_short
                                as libc::c_int
                                * ((1.0f64
                                    * ((1 as libc::c_int as int64_t) << 8 as libc::c_int)
                                        as libc::c_double + 0.5f64) as libc::c_int
                                    - (*psEncCtrl).sparseness_Q8) as libc::c_short
                                    as libc::c_int & 1 as libc::c_int)
                    } else {
                        ((*psEnc).speech_activity_Q8 as libc::c_short as libc::c_int
                            * ((1.0f64
                                * ((1 as libc::c_int as int64_t) << 8 as libc::c_int)
                                    as libc::c_double + 0.5f64) as libc::c_int
                                - (*psEncCtrl).sparseness_Q8) as libc::c_short
                                as libc::c_int >> 16 as libc::c_int - 1 as libc::c_int)
                            + 1 as libc::c_int >> 1 as libc::c_int
                    });
            tmp32 = SKP_Silk_log2lin(
                (16.0f64
                    * ((1 as libc::c_int as int64_t) << 7 as libc::c_int)
                        as libc::c_double + 0.5f64) as libc::c_int
                    - ((essStrength_Q15 >> 16 as libc::c_int)
                        * ((((2.0f32
                            * ((1 as libc::c_int as int64_t) << 7 as libc::c_int)
                                as libc::c_float) as libc::c_double + 0.5f64) as libc::c_int
                            >> 16 as libc::c_int)
                            * (0.16f64
                                * ((1 as libc::c_int as int64_t) << 17 as libc::c_int)
                                    as libc::c_double + 0.5f64) as libc::c_int as libc::c_short
                                as libc::c_int
                            + ((((2.0f32
                                * ((1 as libc::c_int as int64_t) << 7 as libc::c_int)
                                    as libc::c_float) as libc::c_double + 0.5f64) as libc::c_int
                                & 0xffff as libc::c_int)
                                * (0.16f64
                                    * ((1 as libc::c_int as int64_t) << 17 as libc::c_int)
                                        as libc::c_double + 0.5f64) as libc::c_int as libc::c_short
                                    as libc::c_int >> 16 as libc::c_int)) as libc::c_short
                            as libc::c_int
                        + ((essStrength_Q15 & 0xffff as libc::c_int)
                            * ((((2.0f32
                                * ((1 as libc::c_int as int64_t) << 7 as libc::c_int)
                                    as libc::c_float) as libc::c_double + 0.5f64) as libc::c_int
                                >> 16 as libc::c_int)
                                * (0.16f64
                                    * ((1 as libc::c_int as int64_t) << 17 as libc::c_int)
                                        as libc::c_double + 0.5f64) as libc::c_int as libc::c_short
                                    as libc::c_int
                                + ((((2.0f32
                                    * ((1 as libc::c_int as int64_t) << 7 as libc::c_int)
                                        as libc::c_float) as libc::c_double + 0.5f64) as libc::c_int
                                    & 0xffff as libc::c_int)
                                    * (0.16f64
                                        * ((1 as libc::c_int as int64_t) << 17 as libc::c_int)
                                            as libc::c_double + 0.5f64) as libc::c_int as libc::c_short
                                        as libc::c_int >> 16 as libc::c_int)) as libc::c_short
                                as libc::c_int >> 16 as libc::c_int)),
            );
            gain_mult_Q16 = (gain_mult_Q16 >> 16 as libc::c_int)
                * tmp32 as libc::c_short as libc::c_int
                + ((gain_mult_Q16 & 0xffff as libc::c_int)
                    * tmp32 as libc::c_short as libc::c_int >> 16 as libc::c_int)
                + gain_mult_Q16
                    * (if 16 as libc::c_int == 1 as libc::c_int {
                        (tmp32 >> 1 as libc::c_int) + (tmp32 & 1 as libc::c_int)
                    } else {
                        (tmp32 >> 16 as libc::c_int - 1 as libc::c_int)
                            + 1 as libc::c_int >> 1 as libc::c_int
                    });
        } else if (*psEnc).sCmn.fs_kHz == 16 as libc::c_int {
            let mut essStrength_Q15_0: libc::c_int = (-(*psEncCtrl).input_tilt_Q15
                >> 16 as libc::c_int)
                * ((*psEnc).speech_activity_Q8 as libc::c_short as libc::c_int
                    * ((1.0f64
                        * ((1 as libc::c_int as int64_t) << 8 as libc::c_int)
                            as libc::c_double + 0.5f64) as libc::c_int
                        - (*psEncCtrl).sparseness_Q8) as libc::c_short as libc::c_int)
                    as libc::c_short as libc::c_int
                + ((-(*psEncCtrl).input_tilt_Q15 & 0xffff as libc::c_int)
                    * ((*psEnc).speech_activity_Q8 as libc::c_short as libc::c_int
                        * ((1.0f64
                            * ((1 as libc::c_int as int64_t) << 8 as libc::c_int)
                                as libc::c_double + 0.5f64) as libc::c_int
                            - (*psEncCtrl).sparseness_Q8) as libc::c_short
                            as libc::c_int) as libc::c_short as libc::c_int
                    >> 16 as libc::c_int)
                + -(*psEncCtrl).input_tilt_Q15
                    * (if 16 as libc::c_int == 1 as libc::c_int {
                        ((*psEnc).speech_activity_Q8 as libc::c_short as libc::c_int
                            * ((1.0f64
                                * ((1 as libc::c_int as int64_t) << 8 as libc::c_int)
                                    as libc::c_double + 0.5f64) as libc::c_int
                                - (*psEncCtrl).sparseness_Q8) as libc::c_short
                                as libc::c_int >> 1 as libc::c_int)
                            + ((*psEnc).speech_activity_Q8 as libc::c_short
                                as libc::c_int
                                * ((1.0f64
                                    * ((1 as libc::c_int as int64_t) << 8 as libc::c_int)
                                        as libc::c_double + 0.5f64) as libc::c_int
                                    - (*psEncCtrl).sparseness_Q8) as libc::c_short
                                    as libc::c_int & 1 as libc::c_int)
                    } else {
                        ((*psEnc).speech_activity_Q8 as libc::c_short as libc::c_int
                            * ((1.0f64
                                * ((1 as libc::c_int as int64_t) << 8 as libc::c_int)
                                    as libc::c_double + 0.5f64) as libc::c_int
                                - (*psEncCtrl).sparseness_Q8) as libc::c_short
                                as libc::c_int >> 16 as libc::c_int - 1 as libc::c_int)
                            + 1 as libc::c_int >> 1 as libc::c_int
                    });
            tmp32 = SKP_Silk_log2lin(
                (16.0f64
                    * ((1 as libc::c_int as int64_t) << 7 as libc::c_int)
                        as libc::c_double + 0.5f64) as libc::c_int
                    - ((essStrength_Q15_0 >> 16 as libc::c_int)
                        * ((((1.0f32
                            * ((1 as libc::c_int as int64_t) << 7 as libc::c_int)
                                as libc::c_float) as libc::c_double + 0.5f64) as libc::c_int
                            >> 16 as libc::c_int)
                            * (0.16f64
                                * ((1 as libc::c_int as int64_t) << 17 as libc::c_int)
                                    as libc::c_double + 0.5f64) as libc::c_int as libc::c_short
                                as libc::c_int
                            + ((((1.0f32
                                * ((1 as libc::c_int as int64_t) << 7 as libc::c_int)
                                    as libc::c_float) as libc::c_double + 0.5f64) as libc::c_int
                                & 0xffff as libc::c_int)
                                * (0.16f64
                                    * ((1 as libc::c_int as int64_t) << 17 as libc::c_int)
                                        as libc::c_double + 0.5f64) as libc::c_int as libc::c_short
                                    as libc::c_int >> 16 as libc::c_int)) as libc::c_short
                            as libc::c_int
                        + ((essStrength_Q15_0 & 0xffff as libc::c_int)
                            * ((((1.0f32
                                * ((1 as libc::c_int as int64_t) << 7 as libc::c_int)
                                    as libc::c_float) as libc::c_double + 0.5f64) as libc::c_int
                                >> 16 as libc::c_int)
                                * (0.16f64
                                    * ((1 as libc::c_int as int64_t) << 17 as libc::c_int)
                                        as libc::c_double + 0.5f64) as libc::c_int as libc::c_short
                                    as libc::c_int
                                + ((((1.0f32
                                    * ((1 as libc::c_int as int64_t) << 7 as libc::c_int)
                                        as libc::c_float) as libc::c_double + 0.5f64) as libc::c_int
                                    & 0xffff as libc::c_int)
                                    * (0.16f64
                                        * ((1 as libc::c_int as int64_t) << 17 as libc::c_int)
                                            as libc::c_double + 0.5f64) as libc::c_int as libc::c_short
                                        as libc::c_int >> 16 as libc::c_int)) as libc::c_short
                                as libc::c_int >> 16 as libc::c_int)),
            );
            gain_mult_Q16 = (gain_mult_Q16 >> 16 as libc::c_int)
                * tmp32 as libc::c_short as libc::c_int
                + ((gain_mult_Q16 & 0xffff as libc::c_int)
                    * tmp32 as libc::c_short as libc::c_int >> 16 as libc::c_int)
                + gain_mult_Q16
                    * (if 16 as libc::c_int == 1 as libc::c_int {
                        (tmp32 >> 1 as libc::c_int) + (tmp32 & 1 as libc::c_int)
                    } else {
                        (tmp32 >> 16 as libc::c_int - 1 as libc::c_int)
                            + 1 as libc::c_int >> 1 as libc::c_int
                    });
        }
    }
    k = 0 as libc::c_int;
    while k < 4 as libc::c_int {
        (*psEncCtrl)
            .GainsPre_Q14[k
            as usize] = (gain_mult_Q16 >> 16 as libc::c_int)
            * (*psEncCtrl).GainsPre_Q14[k as usize] as libc::c_short as libc::c_int
            + ((gain_mult_Q16 & 0xffff as libc::c_int)
                * (*psEncCtrl).GainsPre_Q14[k as usize] as libc::c_short as libc::c_int
                >> 16 as libc::c_int);
        k += 1;
        k;
    }
    strength_Q16 = ((3.0f32
        * ((1 as libc::c_int as int64_t) << 0 as libc::c_int) as libc::c_float)
        as libc::c_double + 0.5f64) as libc::c_int
        * ((1.0f64
            * ((1 as libc::c_int as int64_t) << 16 as libc::c_int) as libc::c_double
            + 0.5f64) as libc::c_int
            + ((0.5f32
                * ((1 as libc::c_int as int64_t) << 1 as libc::c_int) as libc::c_float)
                as libc::c_double + 0.5f64) as libc::c_int as libc::c_short
                as libc::c_int
                * ((*psEncCtrl).input_quality_bands_Q15[0 as libc::c_int as usize]
                    - (1.0f64
                        * ((1 as libc::c_int as int64_t) << 15 as libc::c_int)
                            as libc::c_double + 0.5f64) as libc::c_int) as libc::c_short
                    as libc::c_int);
    if (*psEncCtrl).sCmn.sigtype == 0 as libc::c_int {
        let mut fs_kHz_inv: libc::c_int = (0.2f64
            * ((1 as libc::c_int as int64_t) << 14 as libc::c_int) as libc::c_double
            + 0.5f64) as libc::c_int / (*psEnc).sCmn.fs_kHz;
        k = 0 as libc::c_int;
        while k < 4 as libc::c_int {
            b_Q14 = fs_kHz_inv
                + (3.0f64
                    * ((1 as libc::c_int as int64_t) << 14 as libc::c_int)
                        as libc::c_double + 0.5f64) as libc::c_int
                    / (*psEncCtrl).sCmn.pitchL[k as usize];
            (*psEncCtrl)
                .LF_shp_Q14[k
                as usize] = (1.0f64
                * ((1 as libc::c_int as int64_t) << 14 as libc::c_int) as libc::c_double
                + 0.5f64) as libc::c_int - b_Q14
                - ((strength_Q16 >> 16 as libc::c_int)
                    * b_Q14 as libc::c_short as libc::c_int
                    + ((strength_Q16 & 0xffff as libc::c_int)
                        * b_Q14 as libc::c_short as libc::c_int >> 16 as libc::c_int))
                << 16 as libc::c_int;
            (*psEncCtrl).LF_shp_Q14[k as usize]
                |= (b_Q14
                    - (1.0f64
                        * ((1 as libc::c_int as int64_t) << 14 as libc::c_int)
                            as libc::c_double + 0.5f64) as libc::c_int) as libc::c_ushort
                    as libc::c_int;
            k += 1;
            k;
        }
        Tilt_Q16 = -(((0.3f32
            * ((1 as libc::c_int as int64_t) << 16 as libc::c_int) as libc::c_float)
            as libc::c_double + 0.5f64) as libc::c_int)
            - (((1.0f64
                * ((1 as libc::c_int as int64_t) << 16 as libc::c_int) as libc::c_double
                + 0.5f64) as libc::c_int
                - ((0.3f32
                    * ((1 as libc::c_int as int64_t) << 16 as libc::c_int)
                        as libc::c_float) as libc::c_double + 0.5f64) as libc::c_int
                >> 16 as libc::c_int)
                * ((((0.35f32
                    * ((1 as libc::c_int as int64_t) << 24 as libc::c_int)
                        as libc::c_float) as libc::c_double + 0.5f64) as libc::c_int
                    >> 16 as libc::c_int)
                    * (*psEnc).speech_activity_Q8 as libc::c_short as libc::c_int
                    + ((((0.35f32
                        * ((1 as libc::c_int as int64_t) << 24 as libc::c_int)
                            as libc::c_float) as libc::c_double + 0.5f64) as libc::c_int
                        & 0xffff as libc::c_int)
                        * (*psEnc).speech_activity_Q8 as libc::c_short as libc::c_int
                        >> 16 as libc::c_int)) as libc::c_short as libc::c_int
                + (((1.0f64
                    * ((1 as libc::c_int as int64_t) << 16 as libc::c_int)
                        as libc::c_double + 0.5f64) as libc::c_int
                    - ((0.3f32
                        * ((1 as libc::c_int as int64_t) << 16 as libc::c_int)
                            as libc::c_float) as libc::c_double + 0.5f64) as libc::c_int
                    & 0xffff as libc::c_int)
                    * ((((0.35f32
                        * ((1 as libc::c_int as int64_t) << 24 as libc::c_int)
                            as libc::c_float) as libc::c_double + 0.5f64) as libc::c_int
                        >> 16 as libc::c_int)
                        * (*psEnc).speech_activity_Q8 as libc::c_short as libc::c_int
                        + ((((0.35f32
                            * ((1 as libc::c_int as int64_t) << 24 as libc::c_int)
                                as libc::c_float) as libc::c_double + 0.5f64) as libc::c_int
                            & 0xffff as libc::c_int)
                            * (*psEnc).speech_activity_Q8 as libc::c_short as libc::c_int
                            >> 16 as libc::c_int)) as libc::c_short as libc::c_int
                    >> 16 as libc::c_int));
    } else {
        b_Q14 = 21299 as libc::c_int / (*psEnc).sCmn.fs_kHz;
        (*psEncCtrl)
            .LF_shp_Q14[0 as libc::c_int
            as usize] = (1.0f64
            * ((1 as libc::c_int as int64_t) << 14 as libc::c_int) as libc::c_double
            + 0.5f64) as libc::c_int - b_Q14
            - ((strength_Q16 >> 16 as libc::c_int)
                * (((0.6f64
                    * ((1 as libc::c_int as int64_t) << 16 as libc::c_int)
                        as libc::c_double + 0.5f64) as libc::c_int >> 16 as libc::c_int)
                    * b_Q14 as libc::c_short as libc::c_int
                    + (((0.6f64
                        * ((1 as libc::c_int as int64_t) << 16 as libc::c_int)
                            as libc::c_double + 0.5f64) as libc::c_int
                        & 0xffff as libc::c_int) * b_Q14 as libc::c_short as libc::c_int
                        >> 16 as libc::c_int)) as libc::c_short as libc::c_int
                + ((strength_Q16 & 0xffff as libc::c_int)
                    * (((0.6f64
                        * ((1 as libc::c_int as int64_t) << 16 as libc::c_int)
                            as libc::c_double + 0.5f64) as libc::c_int
                        >> 16 as libc::c_int) * b_Q14 as libc::c_short as libc::c_int
                        + (((0.6f64
                            * ((1 as libc::c_int as int64_t) << 16 as libc::c_int)
                                as libc::c_double + 0.5f64) as libc::c_int
                            & 0xffff as libc::c_int)
                            * b_Q14 as libc::c_short as libc::c_int
                            >> 16 as libc::c_int)) as libc::c_short as libc::c_int
                    >> 16 as libc::c_int)) << 16 as libc::c_int;
        (*psEncCtrl).LF_shp_Q14[0 as libc::c_int as usize]
            |= (b_Q14
                - (1.0f64
                    * ((1 as libc::c_int as int64_t) << 14 as libc::c_int)
                        as libc::c_double + 0.5f64) as libc::c_int) as libc::c_ushort
                as libc::c_int;
        k = 1 as libc::c_int;
        while k < 4 as libc::c_int {
            (*psEncCtrl)
                .LF_shp_Q14[k
                as usize] = (*psEncCtrl).LF_shp_Q14[0 as libc::c_int as usize];
            k += 1;
            k;
        }
        Tilt_Q16 = -(((0.3f32
            * ((1 as libc::c_int as int64_t) << 16 as libc::c_int) as libc::c_float)
            as libc::c_double + 0.5f64) as libc::c_int);
    }
    HarmBoost_Q16 = (((1.0f64
        * ((1 as libc::c_int as int64_t) << 17 as libc::c_int) as libc::c_double
        + 0.5f64) as libc::c_int - ((*psEncCtrl).coding_quality_Q14 << 3 as libc::c_int)
        >> 16 as libc::c_int) * (*psEnc).LTPCorr_Q15 as libc::c_short as libc::c_int
        + (((1.0f64
            * ((1 as libc::c_int as int64_t) << 17 as libc::c_int) as libc::c_double
            + 0.5f64) as libc::c_int
            - ((*psEncCtrl).coding_quality_Q14 << 3 as libc::c_int)
            & 0xffff as libc::c_int)
            * (*psEnc).LTPCorr_Q15 as libc::c_short as libc::c_int >> 16 as libc::c_int)
        >> 16 as libc::c_int)
        * ((0.1f32
            * ((1 as libc::c_int as int64_t) << 16 as libc::c_int) as libc::c_float)
            as libc::c_double + 0.5f64) as libc::c_int as libc::c_short as libc::c_int
        + ((((1.0f64
            * ((1 as libc::c_int as int64_t) << 17 as libc::c_int) as libc::c_double
            + 0.5f64) as libc::c_int
            - ((*psEncCtrl).coding_quality_Q14 << 3 as libc::c_int) >> 16 as libc::c_int)
            * (*psEnc).LTPCorr_Q15 as libc::c_short as libc::c_int
            + (((1.0f64
                * ((1 as libc::c_int as int64_t) << 17 as libc::c_int) as libc::c_double
                + 0.5f64) as libc::c_int
                - ((*psEncCtrl).coding_quality_Q14 << 3 as libc::c_int)
                & 0xffff as libc::c_int)
                * (*psEnc).LTPCorr_Q15 as libc::c_short as libc::c_int
                >> 16 as libc::c_int) & 0xffff as libc::c_int)
            * ((0.1f32
                * ((1 as libc::c_int as int64_t) << 16 as libc::c_int) as libc::c_float)
                as libc::c_double + 0.5f64) as libc::c_int as libc::c_short
                as libc::c_int >> 16 as libc::c_int);
    HarmBoost_Q16 = HarmBoost_Q16
        + (((1.0f64
            * ((1 as libc::c_int as int64_t) << 16 as libc::c_int) as libc::c_double
            + 0.5f64) as libc::c_int
            - ((*psEncCtrl).input_quality_Q14 << 2 as libc::c_int) >> 16 as libc::c_int)
            * ((0.1f32
                * ((1 as libc::c_int as int64_t) << 16 as libc::c_int) as libc::c_float)
                as libc::c_double + 0.5f64) as libc::c_int as libc::c_short
                as libc::c_int
            + (((1.0f64
                * ((1 as libc::c_int as int64_t) << 16 as libc::c_int) as libc::c_double
                + 0.5f64) as libc::c_int
                - ((*psEncCtrl).input_quality_Q14 << 2 as libc::c_int)
                & 0xffff as libc::c_int)
                * ((0.1f32
                    * ((1 as libc::c_int as int64_t) << 16 as libc::c_int)
                        as libc::c_float) as libc::c_double + 0.5f64) as libc::c_int
                    as libc::c_short as libc::c_int >> 16 as libc::c_int));
    if 1 as libc::c_int != 0 && (*psEncCtrl).sCmn.sigtype == 0 as libc::c_int {
        HarmShapeGain_Q16 = ((0.3f32
            * ((1 as libc::c_int as int64_t) << 16 as libc::c_int) as libc::c_float)
            as libc::c_double + 0.5f64) as libc::c_int
            + (((1.0f64
                * ((1 as libc::c_int as int64_t) << 16 as libc::c_int) as libc::c_double
                + 0.5f64) as libc::c_int
                - (((1.0f64
                    * ((1 as libc::c_int as int64_t) << 18 as libc::c_int)
                        as libc::c_double + 0.5f64) as libc::c_int
                    - ((*psEncCtrl).coding_quality_Q14 << 4 as libc::c_int)
                    >> 16 as libc::c_int)
                    * (*psEncCtrl).input_quality_Q14 as libc::c_short as libc::c_int
                    + (((1.0f64
                        * ((1 as libc::c_int as int64_t) << 18 as libc::c_int)
                            as libc::c_double + 0.5f64) as libc::c_int
                        - ((*psEncCtrl).coding_quality_Q14 << 4 as libc::c_int)
                        & 0xffff as libc::c_int)
                        * (*psEncCtrl).input_quality_Q14 as libc::c_short as libc::c_int
                        >> 16 as libc::c_int)) >> 16 as libc::c_int)
                * ((0.2f32
                    * ((1 as libc::c_int as int64_t) << 16 as libc::c_int)
                        as libc::c_float) as libc::c_double + 0.5f64) as libc::c_int
                    as libc::c_short as libc::c_int
                + (((1.0f64
                    * ((1 as libc::c_int as int64_t) << 16 as libc::c_int)
                        as libc::c_double + 0.5f64) as libc::c_int
                    - (((1.0f64
                        * ((1 as libc::c_int as int64_t) << 18 as libc::c_int)
                            as libc::c_double + 0.5f64) as libc::c_int
                        - ((*psEncCtrl).coding_quality_Q14 << 4 as libc::c_int)
                        >> 16 as libc::c_int)
                        * (*psEncCtrl).input_quality_Q14 as libc::c_short as libc::c_int
                        + (((1.0f64
                            * ((1 as libc::c_int as int64_t) << 18 as libc::c_int)
                                as libc::c_double + 0.5f64) as libc::c_int
                            - ((*psEncCtrl).coding_quality_Q14 << 4 as libc::c_int)
                            & 0xffff as libc::c_int)
                            * (*psEncCtrl).input_quality_Q14 as libc::c_short
                                as libc::c_int >> 16 as libc::c_int))
                    & 0xffff as libc::c_int)
                    * ((0.2f32
                        * ((1 as libc::c_int as int64_t) << 16 as libc::c_int)
                            as libc::c_float) as libc::c_double + 0.5f64) as libc::c_int
                        as libc::c_short as libc::c_int >> 16 as libc::c_int));
        HarmShapeGain_Q16 = (HarmShapeGain_Q16 << 1 as libc::c_int >> 16 as libc::c_int)
            * SKP_Silk_SQRT_APPROX((*psEnc).LTPCorr_Q15 << 15 as libc::c_int)
                as libc::c_short as libc::c_int
            + ((HarmShapeGain_Q16 << 1 as libc::c_int & 0xffff as libc::c_int)
                * SKP_Silk_SQRT_APPROX((*psEnc).LTPCorr_Q15 << 15 as libc::c_int)
                    as libc::c_short as libc::c_int >> 16 as libc::c_int);
    } else {
        HarmShapeGain_Q16 = 0 as libc::c_int;
    }
    k = 0 as libc::c_int;
    while k < 4 as libc::c_int {
        (*psShapeSt)
            .HarmBoost_smth_Q16 = (*psShapeSt).HarmBoost_smth_Q16
            + ((HarmBoost_Q16 - (*psShapeSt).HarmBoost_smth_Q16 >> 16 as libc::c_int)
                * ((0.4f32
                    * ((1 as libc::c_int as int64_t) << 16 as libc::c_int)
                        as libc::c_float) as libc::c_double + 0.5f64) as libc::c_int
                    as libc::c_short as libc::c_int
                + ((HarmBoost_Q16 - (*psShapeSt).HarmBoost_smth_Q16
                    & 0xffff as libc::c_int)
                    * ((0.4f32
                        * ((1 as libc::c_int as int64_t) << 16 as libc::c_int)
                            as libc::c_float) as libc::c_double + 0.5f64) as libc::c_int
                        as libc::c_short as libc::c_int >> 16 as libc::c_int));
        (*psShapeSt)
            .HarmShapeGain_smth_Q16 = (*psShapeSt).HarmShapeGain_smth_Q16
            + ((HarmShapeGain_Q16 - (*psShapeSt).HarmShapeGain_smth_Q16
                >> 16 as libc::c_int)
                * ((0.4f32
                    * ((1 as libc::c_int as int64_t) << 16 as libc::c_int)
                        as libc::c_float) as libc::c_double + 0.5f64) as libc::c_int
                    as libc::c_short as libc::c_int
                + ((HarmShapeGain_Q16 - (*psShapeSt).HarmShapeGain_smth_Q16
                    & 0xffff as libc::c_int)
                    * ((0.4f32
                        * ((1 as libc::c_int as int64_t) << 16 as libc::c_int)
                            as libc::c_float) as libc::c_double + 0.5f64) as libc::c_int
                        as libc::c_short as libc::c_int >> 16 as libc::c_int));
        (*psShapeSt)
            .Tilt_smth_Q16 = (*psShapeSt).Tilt_smth_Q16
            + ((Tilt_Q16 - (*psShapeSt).Tilt_smth_Q16 >> 16 as libc::c_int)
                * ((0.4f32
                    * ((1 as libc::c_int as int64_t) << 16 as libc::c_int)
                        as libc::c_float) as libc::c_double + 0.5f64) as libc::c_int
                    as libc::c_short as libc::c_int
                + ((Tilt_Q16 - (*psShapeSt).Tilt_smth_Q16 & 0xffff as libc::c_int)
                    * ((0.4f32
                        * ((1 as libc::c_int as int64_t) << 16 as libc::c_int)
                            as libc::c_float) as libc::c_double + 0.5f64) as libc::c_int
                        as libc::c_short as libc::c_int >> 16 as libc::c_int));
        (*psEncCtrl)
            .HarmBoost_Q14[k
            as usize] = if 2 as libc::c_int == 1 as libc::c_int {
            ((*psShapeSt).HarmBoost_smth_Q16 >> 1 as libc::c_int)
                + ((*psShapeSt).HarmBoost_smth_Q16 & 1 as libc::c_int)
        } else {
            ((*psShapeSt).HarmBoost_smth_Q16 >> 2 as libc::c_int - 1 as libc::c_int)
                + 1 as libc::c_int >> 1 as libc::c_int
        };
        (*psEncCtrl)
            .HarmShapeGain_Q14[k
            as usize] = if 2 as libc::c_int == 1 as libc::c_int {
            ((*psShapeSt).HarmShapeGain_smth_Q16 >> 1 as libc::c_int)
                + ((*psShapeSt).HarmShapeGain_smth_Q16 & 1 as libc::c_int)
        } else {
            ((*psShapeSt).HarmShapeGain_smth_Q16 >> 2 as libc::c_int - 1 as libc::c_int)
                + 1 as libc::c_int >> 1 as libc::c_int
        };
        (*psEncCtrl)
            .Tilt_Q14[k
            as usize] = if 2 as libc::c_int == 1 as libc::c_int {
            ((*psShapeSt).Tilt_smth_Q16 >> 1 as libc::c_int)
                + ((*psShapeSt).Tilt_smth_Q16 & 1 as libc::c_int)
        } else {
            ((*psShapeSt).Tilt_smth_Q16 >> 2 as libc::c_int - 1 as libc::c_int)
                + 1 as libc::c_int >> 1 as libc::c_int
        };
        k += 1;
        k;
    }
}
