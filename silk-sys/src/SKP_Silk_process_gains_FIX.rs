#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn SKP_Silk_sigm_Q15(in_Q5: libc::c_int) -> libc::c_int;
    fn SKP_Silk_log2lin(inLog_Q7: libc::c_int) -> libc::c_int;
    static SKP_Silk_Quantization_Offsets_Q10: [[libc::c_short; 2]; 2];
    fn SKP_Silk_gains_quant(
        ind: *mut libc::c_int,
        gain_Q16: *mut libc::c_int,
        prev_ind: *mut libc::c_int,
        conditional: libc::c_int,
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
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_process_gains_FIX(
    mut psEnc: *mut SKP_Silk_encoder_state_FIX,
    mut psEncCtrl: *mut SKP_Silk_encoder_control_FIX,
) {
    let mut psShapeSt: *mut SKP_Silk_shape_state_FIX = &mut (*psEnc).sShape;
    let mut k: libc::c_int = 0;
    let mut s_Q16: libc::c_int = 0;
    let mut InvMaxSqrVal_Q16: libc::c_int = 0;
    let mut gain: libc::c_int = 0;
    let mut gain_squared: libc::c_int = 0;
    let mut ResNrg: libc::c_int = 0;
    let mut ResNrgPart: libc::c_int = 0;
    let mut quant_offset_Q10: libc::c_int = 0;
    if (*psEncCtrl).sCmn.sigtype == 0 as libc::c_int {
        s_Q16 = -SKP_Silk_sigm_Q15(
            if 4 as libc::c_int == 1 as libc::c_int {
                ((*psEncCtrl).LTPredCodGain_Q7
                    - (12.0f64
                        * ((1 as libc::c_int as int64_t) << 7 as libc::c_int)
                            as libc::c_double + 0.5f64) as libc::c_int
                    >> 1 as libc::c_int)
                    + ((*psEncCtrl).LTPredCodGain_Q7
                        - (12.0f64
                            * ((1 as libc::c_int as int64_t) << 7 as libc::c_int)
                                as libc::c_double + 0.5f64) as libc::c_int
                        & 1 as libc::c_int)
            } else {
                ((*psEncCtrl).LTPredCodGain_Q7
                    - (12.0f64
                        * ((1 as libc::c_int as int64_t) << 7 as libc::c_int)
                            as libc::c_double + 0.5f64) as libc::c_int
                    >> 4 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                    >> 1 as libc::c_int
            },
        );
        k = 0 as libc::c_int;
        while k < 4 as libc::c_int {
            (*psEncCtrl)
                .Gains_Q16[k
                as usize] = (*psEncCtrl).Gains_Q16[k as usize]
                + (((*psEncCtrl).Gains_Q16[k as usize] >> 16 as libc::c_int)
                    * s_Q16 as libc::c_short as libc::c_int
                    + (((*psEncCtrl).Gains_Q16[k as usize] & 0xffff as libc::c_int)
                        * s_Q16 as libc::c_short as libc::c_int >> 16 as libc::c_int));
            k += 1;
            k;
        }
    }
    InvMaxSqrVal_Q16 = SKP_Silk_log2lin(
        ((70.0f64 * ((1 as libc::c_int as int64_t) << 7 as libc::c_int) as libc::c_double
            + 0.5f64) as libc::c_int - (*psEncCtrl).current_SNR_dB_Q7
            >> 16 as libc::c_int)
            * (0.33f64
                * ((1 as libc::c_int as int64_t) << 16 as libc::c_int) as libc::c_double
                + 0.5f64) as libc::c_int as libc::c_short as libc::c_int
            + (((70.0f64
                * ((1 as libc::c_int as int64_t) << 7 as libc::c_int) as libc::c_double
                + 0.5f64) as libc::c_int - (*psEncCtrl).current_SNR_dB_Q7
                & 0xffff as libc::c_int)
                * (0.33f64
                    * ((1 as libc::c_int as int64_t) << 16 as libc::c_int)
                        as libc::c_double + 0.5f64) as libc::c_int as libc::c_short
                    as libc::c_int >> 16 as libc::c_int),
    ) / (*psEnc).sCmn.subfr_length;
    k = 0 as libc::c_int;
    while k < 4 as libc::c_int {
        ResNrg = (*psEncCtrl).ResNrg[k as usize];
        ResNrgPart = (ResNrg >> 16 as libc::c_int)
            * InvMaxSqrVal_Q16 as libc::c_short as libc::c_int
            + ((ResNrg & 0xffff as libc::c_int)
                * InvMaxSqrVal_Q16 as libc::c_short as libc::c_int >> 16 as libc::c_int)
            + ResNrg
                * (if 16 as libc::c_int == 1 as libc::c_int {
                    (InvMaxSqrVal_Q16 >> 1 as libc::c_int)
                        + (InvMaxSqrVal_Q16 & 1 as libc::c_int)
                } else {
                    (InvMaxSqrVal_Q16 >> 16 as libc::c_int - 1 as libc::c_int)
                        + 1 as libc::c_int >> 1 as libc::c_int
                });
        if (*psEncCtrl).ResNrgQ[k as usize] > 0 as libc::c_int {
            if (*psEncCtrl).ResNrgQ[k as usize] < 32 as libc::c_int {
                ResNrgPart = if (*psEncCtrl).ResNrgQ[k as usize] == 1 as libc::c_int {
                    (ResNrgPart >> 1 as libc::c_int) + (ResNrgPart & 1 as libc::c_int)
                } else {
                    (ResNrgPart >> (*psEncCtrl).ResNrgQ[k as usize] - 1 as libc::c_int)
                        + 1 as libc::c_int >> 1 as libc::c_int
                };
            } else {
                ResNrgPart = 0 as libc::c_int;
            }
        } else if (*psEncCtrl).ResNrgQ[k as usize] != 0 as libc::c_int {
            if ResNrgPart
                > 0x7fffffff as libc::c_int >> -(*psEncCtrl).ResNrgQ[k as usize]
            {
                ResNrgPart = 0x7fffffff as libc::c_int;
            } else {
                ResNrgPart = ResNrgPart << -(*psEncCtrl).ResNrgQ[k as usize];
            }
        }
        gain = (*psEncCtrl).Gains_Q16[k as usize];
        gain_squared = if (ResNrgPart
            + (gain as int64_t * gain as int64_t >> 32 as libc::c_int) as libc::c_int)
            as libc::c_uint & 0x80000000 as libc::c_uint
            == 0 as libc::c_int as libc::c_uint
        {
            if (ResNrgPart
                & (gain as int64_t * gain as int64_t >> 32 as libc::c_int)
                    as libc::c_int) as libc::c_uint & 0x80000000 as libc::c_uint
                != 0 as libc::c_int as libc::c_uint
            {
                0x80000000 as libc::c_uint as libc::c_int
            } else {
                ResNrgPart
                    + (gain as int64_t * gain as int64_t >> 32 as libc::c_int)
                        as libc::c_int
            }
        } else if (ResNrgPart
            | (gain as int64_t * gain as int64_t >> 32 as libc::c_int) as libc::c_int)
            as libc::c_uint & 0x80000000 as libc::c_uint
            == 0 as libc::c_int as libc::c_uint
        {
            0x7fffffff as libc::c_int
        } else {
            ResNrgPart
                + (gain as int64_t * gain as int64_t >> 32 as libc::c_int) as libc::c_int
        };
        if gain_squared < 0x7fff as libc::c_int {
            gain_squared = (ResNrgPart << 16 as libc::c_int)
                + ((gain >> 16 as libc::c_int) * gain as libc::c_short as libc::c_int
                    + ((gain & 0xffff as libc::c_int)
                        * gain as libc::c_short as libc::c_int >> 16 as libc::c_int))
                + gain
                    * (if 16 as libc::c_int == 1 as libc::c_int {
                        (gain >> 1 as libc::c_int) + (gain & 1 as libc::c_int)
                    } else {
                        (gain >> 16 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                            >> 1 as libc::c_int
                    });
            gain = SKP_Silk_SQRT_APPROX(gain_squared);
            (*psEncCtrl)
                .Gains_Q16[k
                as usize] = (if 0x80000000 as libc::c_uint as libc::c_int
                >> 8 as libc::c_int > 0x7fffffff as libc::c_int >> 8 as libc::c_int
            {
                (if gain > 0x80000000 as libc::c_uint as libc::c_int >> 8 as libc::c_int
                {
                    0x80000000 as libc::c_uint as libc::c_int >> 8 as libc::c_int
                } else {
                    (if gain < 0x7fffffff as libc::c_int >> 8 as libc::c_int {
                        0x7fffffff as libc::c_int >> 8 as libc::c_int
                    } else {
                        gain
                    })
                })
            } else {
                (if gain > 0x7fffffff as libc::c_int >> 8 as libc::c_int {
                    0x7fffffff as libc::c_int >> 8 as libc::c_int
                } else {
                    (if gain
                        < 0x80000000 as libc::c_uint as libc::c_int >> 8 as libc::c_int
                    {
                        0x80000000 as libc::c_uint as libc::c_int >> 8 as libc::c_int
                    } else {
                        gain
                    })
                })
            }) << 8 as libc::c_int;
        } else {
            gain = SKP_Silk_SQRT_APPROX(gain_squared);
            (*psEncCtrl)
                .Gains_Q16[k
                as usize] = (if 0x80000000 as libc::c_uint as libc::c_int
                >> 16 as libc::c_int > 0x7fffffff as libc::c_int >> 16 as libc::c_int
            {
                (if gain > 0x80000000 as libc::c_uint as libc::c_int >> 16 as libc::c_int
                {
                    0x80000000 as libc::c_uint as libc::c_int >> 16 as libc::c_int
                } else {
                    (if gain < 0x7fffffff as libc::c_int >> 16 as libc::c_int {
                        0x7fffffff as libc::c_int >> 16 as libc::c_int
                    } else {
                        gain
                    })
                })
            } else {
                (if gain > 0x7fffffff as libc::c_int >> 16 as libc::c_int {
                    0x7fffffff as libc::c_int >> 16 as libc::c_int
                } else {
                    (if gain
                        < 0x80000000 as libc::c_uint as libc::c_int >> 16 as libc::c_int
                    {
                        0x80000000 as libc::c_uint as libc::c_int >> 16 as libc::c_int
                    } else {
                        gain
                    })
                })
            }) << 16 as libc::c_int;
        }
        k += 1;
        k;
    }
    SKP_Silk_gains_quant(
        ((*psEncCtrl).sCmn.GainsIndices).as_mut_ptr(),
        ((*psEncCtrl).Gains_Q16).as_mut_ptr(),
        &mut (*psShapeSt).LastGainIndex,
        (*psEnc).sCmn.nFramesInPayloadBuf,
    );
    if (*psEncCtrl).sCmn.sigtype == 0 as libc::c_int {
        if (*psEncCtrl).LTPredCodGain_Q7
            + ((*psEncCtrl).input_tilt_Q15 >> 8 as libc::c_int)
            > (1.0f64
                * ((1 as libc::c_int as int64_t) << 7 as libc::c_int) as libc::c_double
                + 0.5f64) as libc::c_int
        {
            (*psEncCtrl).sCmn.QuantOffsetType = 0 as libc::c_int;
        } else {
            (*psEncCtrl).sCmn.QuantOffsetType = 1 as libc::c_int;
        }
    }
    quant_offset_Q10 = SKP_Silk_Quantization_Offsets_Q10[(*psEncCtrl).sCmn.sigtype
        as usize][(*psEncCtrl).sCmn.QuantOffsetType as usize] as libc::c_int;
    (*psEncCtrl)
        .Lambda_Q10 = ((1.2f32
        * ((1 as libc::c_int as int64_t) << 10 as libc::c_int) as libc::c_float)
        as libc::c_double + 0.5f64) as libc::c_int
        + ((-0.05f32
            * ((1 as libc::c_int as int64_t) << 10 as libc::c_int) as libc::c_float)
            as libc::c_double + 0.5f64) as libc::c_int as libc::c_short as libc::c_int
            * (*psEnc).sCmn.nStatesDelayedDecision as libc::c_short as libc::c_int
        + ((((-0.3f32
            * ((1 as libc::c_int as int64_t) << 18 as libc::c_int) as libc::c_float)
            as libc::c_double + 0.5f64) as libc::c_int >> 16 as libc::c_int)
            * (*psEnc).speech_activity_Q8 as libc::c_short as libc::c_int
            + ((((-0.3f32
                * ((1 as libc::c_int as int64_t) << 18 as libc::c_int) as libc::c_float)
                as libc::c_double + 0.5f64) as libc::c_int & 0xffff as libc::c_int)
                * (*psEnc).speech_activity_Q8 as libc::c_short as libc::c_int
                >> 16 as libc::c_int))
        + ((((-0.2f32
            * ((1 as libc::c_int as int64_t) << 12 as libc::c_int) as libc::c_float)
            as libc::c_double + 0.5f64) as libc::c_int >> 16 as libc::c_int)
            * (*psEncCtrl).input_quality_Q14 as libc::c_short as libc::c_int
            + ((((-0.2f32
                * ((1 as libc::c_int as int64_t) << 12 as libc::c_int) as libc::c_float)
                as libc::c_double + 0.5f64) as libc::c_int & 0xffff as libc::c_int)
                * (*psEncCtrl).input_quality_Q14 as libc::c_short as libc::c_int
                >> 16 as libc::c_int))
        + ((((-0.1f32
            * ((1 as libc::c_int as int64_t) << 12 as libc::c_int) as libc::c_float)
            as libc::c_double + 0.5f64) as libc::c_int >> 16 as libc::c_int)
            * (*psEncCtrl).coding_quality_Q14 as libc::c_short as libc::c_int
            + ((((-0.1f32
                * ((1 as libc::c_int as int64_t) << 12 as libc::c_int) as libc::c_float)
                as libc::c_double + 0.5f64) as libc::c_int & 0xffff as libc::c_int)
                * (*psEncCtrl).coding_quality_Q14 as libc::c_short as libc::c_int
                >> 16 as libc::c_int))
        + ((((1.5f32
            * ((1 as libc::c_int as int64_t) << 16 as libc::c_int) as libc::c_float)
            as libc::c_double + 0.5f64) as libc::c_int >> 16 as libc::c_int)
            * quant_offset_Q10 as libc::c_short as libc::c_int
            + ((((1.5f32
                * ((1 as libc::c_int as int64_t) << 16 as libc::c_int) as libc::c_float)
                as libc::c_double + 0.5f64) as libc::c_int & 0xffff as libc::c_int)
                * quant_offset_Q10 as libc::c_short as libc::c_int
                >> 16 as libc::c_int));
}
