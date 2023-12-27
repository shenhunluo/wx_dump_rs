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
    fn SKP_Silk_find_LTP_FIX(
        b_Q14: *mut libc::c_short,
        WLTP: *mut libc::c_int,
        LTPredCodGain_Q7: *mut libc::c_int,
        r_first: *const libc::c_short,
        r_last: *const libc::c_short,
        lag: *const libc::c_int,
        Wght_Q15: *const libc::c_int,
        subfr_length: libc::c_int,
        mem_offset: libc::c_int,
        corr_rshifts: *mut libc::c_int,
    );
    fn SKP_Silk_quant_LTP_gains_FIX(
        B_Q14: *mut libc::c_short,
        cbk_index: *mut libc::c_int,
        periodicity_index: *mut libc::c_int,
        W_Q18: *const libc::c_int,
        mu_Q8: libc::c_int,
        lowComplexity: libc::c_int,
    );
    fn SKP_Silk_LTP_analysis_filter_FIX(
        LTP_res: *mut libc::c_short,
        x: *const libc::c_short,
        LTPCoef_Q14: *const libc::c_short,
        pitchL: *const libc::c_int,
        invGains_Q16: *const libc::c_int,
        subfr_length: libc::c_int,
        pre_length: libc::c_int,
    );
    fn SKP_Silk_find_LPC_FIX(
        NLSF_Q15: *mut libc::c_int,
        interpIndex: *mut libc::c_int,
        prev_NLSFq_Q15: *const libc::c_int,
        useInterpolatedLSFs: libc::c_int,
        LPC_order: libc::c_int,
        x: *const libc::c_short,
        subfr_length: libc::c_int,
    );
    fn SKP_Silk_process_NLSFs_FIX(
        psEnc: *mut SKP_Silk_encoder_state_FIX,
        psEncCtrl: *mut SKP_Silk_encoder_control_FIX,
        pNLSF_Q15: *mut libc::c_int,
    );
    fn SKP_Silk_residual_energy_FIX(
        nrgs: *mut libc::c_int,
        nrgsQ: *mut libc::c_int,
        x: *const libc::c_short,
        a_Q12: *mut [libc::c_short; 16],
        gains: *const libc::c_int,
        subfr_length: libc::c_int,
        LPC_order: libc::c_int,
    );
    fn SKP_Silk_LTP_scale_ctrl_FIX(
        psEnc: *mut SKP_Silk_encoder_state_FIX,
        psEncCtrl: *mut SKP_Silk_encoder_control_FIX,
    );
    fn SKP_Silk_scale_copy_vector16(
        data_out: *mut libc::c_short,
        data_in: *const libc::c_short,
        gain_Q16: libc::c_int,
        dataSize: libc::c_int,
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
pub unsafe extern "C" fn SKP_Silk_find_pred_coefs_FIX(
    mut psEnc: *mut SKP_Silk_encoder_state_FIX,
    mut psEncCtrl: *mut SKP_Silk_encoder_control_FIX,
    mut res_pitch: *const libc::c_short,
) {
    let mut i: libc::c_int = 0;
    let mut WLTP: [libc::c_int; 100] = [0; 100];
    let mut invGains_Q16: [libc::c_int; 4] = [0; 4];
    let mut local_gains: [libc::c_int; 4] = [0; 4];
    let mut Wght_Q15: [libc::c_int; 4] = [0; 4];
    let mut NLSF_Q15: [libc::c_int; 16] = [0; 16];
    let mut x_ptr: *const libc::c_short = 0 as *const libc::c_short;
    let mut x_pre_ptr: *mut libc::c_short = 0 as *mut libc::c_short;
    let mut LPC_in_pre: [libc::c_short; 544] = [0; 544];
    let mut tmp: libc::c_int = 0;
    let mut min_gain_Q16: libc::c_int = 0;
    let mut LTP_corrs_rshift: [libc::c_int; 4] = [0; 4];
    min_gain_Q16 = 0x7fffffff as libc::c_int >> 6 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        min_gain_Q16 = if min_gain_Q16 < (*psEncCtrl).Gains_Q16[i as usize] {
            min_gain_Q16
        } else {
            (*psEncCtrl).Gains_Q16[i as usize]
        };
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        invGains_Q16[i
            as usize] = SKP_DIV32_varQ(
            min_gain_Q16,
            (*psEncCtrl).Gains_Q16[i as usize],
            16 as libc::c_int - 2 as libc::c_int,
        );
        invGains_Q16[i
            as usize] = if invGains_Q16[i as usize] > 363 as libc::c_int {
            invGains_Q16[i as usize]
        } else {
            363 as libc::c_int
        };
        tmp = (invGains_Q16[i as usize] >> 16 as libc::c_int)
            * invGains_Q16[i as usize] as libc::c_short as libc::c_int
            + ((invGains_Q16[i as usize] & 0xffff as libc::c_int)
                * invGains_Q16[i as usize] as libc::c_short as libc::c_int
                >> 16 as libc::c_int);
        Wght_Q15[i as usize] = tmp >> 1 as libc::c_int;
        local_gains[i
            as usize] = ((1 as libc::c_int) << 16 as libc::c_int)
            / invGains_Q16[i as usize];
        i += 1;
        i;
    }
    if (*psEncCtrl).sCmn.sigtype == 0 as libc::c_int {
        SKP_Silk_find_LTP_FIX(
            ((*psEncCtrl).LTPCoef_Q14).as_mut_ptr(),
            WLTP.as_mut_ptr(),
            &mut (*psEncCtrl).LTPredCodGain_Q7,
            res_pitch,
            res_pitch.offset(((*psEnc).sCmn.frame_length >> 1 as libc::c_int) as isize),
            ((*psEncCtrl).sCmn.pitchL).as_mut_ptr() as *const libc::c_int,
            Wght_Q15.as_mut_ptr() as *const libc::c_int,
            (*psEnc).sCmn.subfr_length,
            (*psEnc).sCmn.frame_length,
            LTP_corrs_rshift.as_mut_ptr(),
        );
        SKP_Silk_quant_LTP_gains_FIX(
            ((*psEncCtrl).LTPCoef_Q14).as_mut_ptr(),
            ((*psEncCtrl).sCmn.LTPIndex).as_mut_ptr(),
            &mut (*psEncCtrl).sCmn.PERIndex,
            WLTP.as_mut_ptr() as *const libc::c_int,
            (*psEnc).mu_LTP_Q8,
            (*psEnc).sCmn.LTPQuantLowComplexity,
        );
        SKP_Silk_LTP_scale_ctrl_FIX(psEnc, psEncCtrl);
        SKP_Silk_LTP_analysis_filter_FIX(
            LPC_in_pre.as_mut_ptr(),
            ((*psEnc).x_buf)
                .as_mut_ptr()
                .offset((*psEnc).sCmn.frame_length as isize)
                .offset(-((*psEnc).sCmn.predictLPCOrder as isize)),
            ((*psEncCtrl).LTPCoef_Q14).as_mut_ptr() as *const libc::c_short,
            ((*psEncCtrl).sCmn.pitchL).as_mut_ptr() as *const libc::c_int,
            invGains_Q16.as_mut_ptr() as *const libc::c_int,
            (*psEnc).sCmn.subfr_length,
            (*psEnc).sCmn.predictLPCOrder,
        );
    } else {
        x_ptr = ((*psEnc).x_buf)
            .as_mut_ptr()
            .offset((*psEnc).sCmn.frame_length as isize)
            .offset(-((*psEnc).sCmn.predictLPCOrder as isize));
        x_pre_ptr = LPC_in_pre.as_mut_ptr();
        i = 0 as libc::c_int;
        while i < 4 as libc::c_int {
            SKP_Silk_scale_copy_vector16(
                x_pre_ptr,
                x_ptr,
                invGains_Q16[i as usize],
                (*psEnc).sCmn.subfr_length + (*psEnc).sCmn.predictLPCOrder,
            );
            x_pre_ptr = x_pre_ptr
                .offset(
                    ((*psEnc).sCmn.subfr_length + (*psEnc).sCmn.predictLPCOrder) as isize,
                );
            x_ptr = x_ptr.offset((*psEnc).sCmn.subfr_length as isize);
            i += 1;
            i;
        }
        memset(
            ((*psEncCtrl).LTPCoef_Q14).as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            ((4 as libc::c_int * 5 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_short>() as libc::c_ulong),
        );
        (*psEncCtrl).LTPredCodGain_Q7 = 0 as libc::c_int;
    }
    SKP_Silk_find_LPC_FIX(
        NLSF_Q15.as_mut_ptr(),
        &mut (*psEncCtrl).sCmn.NLSFInterpCoef_Q2,
        ((*psEnc).sPred.prev_NLSFq_Q15).as_mut_ptr() as *const libc::c_int,
        (*psEnc).sCmn.useInterpolatedNLSFs
            * (1 as libc::c_int - (*psEnc).sCmn.first_frame_after_reset),
        (*psEnc).sCmn.predictLPCOrder,
        LPC_in_pre.as_mut_ptr() as *const libc::c_short,
        (*psEnc).sCmn.subfr_length + (*psEnc).sCmn.predictLPCOrder,
    );
    SKP_Silk_process_NLSFs_FIX(psEnc, psEncCtrl, NLSF_Q15.as_mut_ptr());
    SKP_Silk_residual_energy_FIX(
        ((*psEncCtrl).ResNrg).as_mut_ptr(),
        ((*psEncCtrl).ResNrgQ).as_mut_ptr(),
        LPC_in_pre.as_mut_ptr() as *const libc::c_short,
        ((*psEncCtrl).PredCoef_Q12).as_mut_ptr(),
        local_gains.as_mut_ptr() as *const libc::c_int,
        (*psEnc).sCmn.subfr_length,
        (*psEnc).sCmn.predictLPCOrder,
    );
    memcpy(
        ((*psEnc).sPred.prev_NLSFq_Q15).as_mut_ptr() as *mut libc::c_void,
        NLSF_Q15.as_mut_ptr() as *const libc::c_void,
        ((*psEnc).sCmn.predictLPCOrder as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
}
