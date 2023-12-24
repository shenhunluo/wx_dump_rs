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
    static SKP_Silk_Quantization_Offsets_Q10: [[libc::c_short; 2]; 2];
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
pub struct NSQ_del_dec_struct {
    pub RandState: [libc::c_int; 32],
    pub Q_Q10: [libc::c_int; 32],
    pub Xq_Q10: [libc::c_int; 32],
    pub Pred_Q16: [libc::c_int; 32],
    pub Shape_Q10: [libc::c_int; 32],
    pub Gain_Q16: [libc::c_int; 32],
    pub sAR2_Q14: [libc::c_int; 16],
    pub sLPC_Q14: [libc::c_int; 152],
    pub LF_AR_Q12: libc::c_int,
    pub Seed: libc::c_int,
    pub SeedInit: libc::c_int,
    pub RD_Q10: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NSQ_sample_struct {
    pub Q_Q10: libc::c_int,
    pub RD_Q10: libc::c_int,
    pub xq_Q14: libc::c_int,
    pub LF_AR_Q12: libc::c_int,
    pub sLTP_shp_Q10: libc::c_int,
    pub LPC_exc_Q16: libc::c_int,
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
unsafe extern "C" fn SKP_min_int(mut a: libc::c_int, mut b: libc::c_int) -> libc::c_int {
    return if a < b { a } else { b };
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
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_NSQ_del_dec(
    mut psEncC: *mut SKP_Silk_encoder_state,
    mut psEncCtrlC: *mut SKP_Silk_encoder_control,
    mut NSQ: *mut SKP_Silk_nsq_state,
    mut x: *const libc::c_short,
    mut q: *mut libc::c_schar,
    LSFInterpFactor_Q2: libc::c_int,
    mut PredCoef_Q12: *const libc::c_short,
    mut LTPCoef_Q14: *const libc::c_short,
    mut AR2_Q13: *const libc::c_short,
    mut HarmShapeGain_Q14: *const libc::c_int,
    mut Tilt_Q14: *const libc::c_int,
    mut LF_shp_Q14: *const libc::c_int,
    mut Gains_Q16: *const libc::c_int,
    Lambda_Q10: libc::c_int,
    LTP_scale_Q14: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut lag: libc::c_int = 0;
    let mut start_idx: libc::c_int = 0;
    let mut LSF_interpolation_flag: libc::c_int = 0;
    let mut Winner_ind: libc::c_int = 0;
    let mut subfr: libc::c_int = 0;
    let mut last_smple_idx: libc::c_int = 0;
    let mut smpl_buf_idx: libc::c_int = 0;
    let mut decisionDelay: libc::c_int = 0;
    let mut subfr_length: libc::c_int = 0;
    let mut A_Q12: *const libc::c_short = 0 as *const libc::c_short;
    let mut B_Q14: *const libc::c_short = 0 as *const libc::c_short;
    let mut AR_shp_Q13: *const libc::c_short = 0 as *const libc::c_short;
    let mut pxq: *mut libc::c_short = 0 as *mut libc::c_short;
    let mut sLTP_Q16: [libc::c_int; 960] = [0; 960];
    let mut sLTP: [libc::c_short; 960] = [0; 960];
    let mut HarmShapeFIRPacked_Q14: libc::c_int = 0;
    let mut offset_Q10: libc::c_int = 0;
    let mut FiltState: [libc::c_int; 16] = [0; 16];
    let mut RDmin_Q10: libc::c_int = 0;
    let mut x_sc_Q10: [libc::c_int; 120] = [0; 120];
    let mut psDelDec: [NSQ_del_dec_struct; 4] = [NSQ_del_dec_struct {
        RandState: [0; 32],
        Q_Q10: [0; 32],
        Xq_Q10: [0; 32],
        Pred_Q16: [0; 32],
        Shape_Q10: [0; 32],
        Gain_Q16: [0; 32],
        sAR2_Q14: [0; 16],
        sLPC_Q14: [0; 152],
        LF_AR_Q12: 0,
        Seed: 0,
        SeedInit: 0,
        RD_Q10: 0,
    }; 4];
    let mut psDD: *mut NSQ_del_dec_struct = 0 as *mut NSQ_del_dec_struct;
    subfr_length = (*psEncC).frame_length / 4 as libc::c_int;
    lag = (*NSQ).lagPrev;
    memset(
        psDelDec.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ((*psEncC).nStatesDelayedDecision as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<NSQ_del_dec_struct>() as libc::c_ulong),
    );
    k = 0 as libc::c_int;
    while k < (*psEncC).nStatesDelayedDecision {
        psDD = &mut *psDelDec.as_mut_ptr().offset(k as isize) as *mut NSQ_del_dec_struct;
        (*psDD).Seed = k + (*psEncCtrlC).Seed & 3 as libc::c_int;
        (*psDD).SeedInit = (*psDD).Seed;
        (*psDD).RD_Q10 = 0 as libc::c_int;
        (*psDD).LF_AR_Q12 = (*NSQ).sLF_AR_shp_Q12;
        (*psDD)
            .Shape_Q10[0 as libc::c_int
            as usize] = (*NSQ)
            .sLTP_shp_Q10[((*psEncC).frame_length - 1 as libc::c_int) as usize];
        memcpy(
            ((*psDD).sLPC_Q14).as_mut_ptr() as *mut libc::c_void,
            ((*NSQ).sLPC_Q14).as_mut_ptr() as *const libc::c_void,
            (32 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
        );
        memcpy(
            ((*psDD).sAR2_Q14).as_mut_ptr() as *mut libc::c_void,
            ((*NSQ).sAR2_Q14).as_mut_ptr() as *const libc::c_void,
            ::core::mem::size_of::<[libc::c_int; 16]>() as libc::c_ulong,
        );
        k += 1;
        k;
    }
    offset_Q10 = SKP_Silk_Quantization_Offsets_Q10[(*psEncCtrlC).sigtype
        as usize][(*psEncCtrlC).QuantOffsetType as usize] as libc::c_int;
    smpl_buf_idx = 0 as libc::c_int;
    decisionDelay = SKP_min_int(32 as libc::c_int, subfr_length);
    if (*psEncCtrlC).sigtype == 0 as libc::c_int {
        k = 0 as libc::c_int;
        while k < 4 as libc::c_int {
            decisionDelay = SKP_min_int(
                decisionDelay,
                (*psEncCtrlC).pitchL[k as usize] - 5 as libc::c_int / 2 as libc::c_int
                    - 1 as libc::c_int,
            );
            k += 1;
            k;
        }
    } else if lag > 0 as libc::c_int {
        decisionDelay = SKP_min_int(
            decisionDelay,
            lag - 5 as libc::c_int / 2 as libc::c_int - 1 as libc::c_int,
        );
    }
    if LSFInterpFactor_Q2 == (1 as libc::c_int) << 2 as libc::c_int {
        LSF_interpolation_flag = 0 as libc::c_int;
    } else {
        LSF_interpolation_flag = 1 as libc::c_int;
    }
    pxq = &mut *((*NSQ).xq).as_mut_ptr().offset((*psEncC).frame_length as isize)
        as *mut libc::c_short;
    (*NSQ).sLTP_shp_buf_idx = (*psEncC).frame_length;
    (*NSQ).sLTP_buf_idx = (*psEncC).frame_length;
    subfr = 0 as libc::c_int;
    k = 0 as libc::c_int;
    while k < 4 as libc::c_int {
        A_Q12 = &*PredCoef_Q12
            .offset(
                ((k >> 1 as libc::c_int | 1 as libc::c_int - LSF_interpolation_flag)
                    * 16 as libc::c_int) as isize,
            ) as *const libc::c_short;
        B_Q14 = &*LTPCoef_Q14.offset((k * 5 as libc::c_int) as isize)
            as *const libc::c_short;
        AR_shp_Q13 = &*AR2_Q13.offset((k * 16 as libc::c_int) as isize)
            as *const libc::c_short;
        HarmShapeFIRPacked_Q14 = *HarmShapeGain_Q14.offset(k as isize)
            >> 2 as libc::c_int;
        HarmShapeFIRPacked_Q14
            |= (*HarmShapeGain_Q14.offset(k as isize) >> 1 as libc::c_int)
                << 16 as libc::c_int;
        (*NSQ).rewhite_flag = 0 as libc::c_int;
        if (*psEncCtrlC).sigtype == 0 as libc::c_int {
            lag = (*psEncCtrlC).pitchL[k as usize];
            if k & 3 as libc::c_int - (LSF_interpolation_flag << 1 as libc::c_int)
                == 0 as libc::c_int
            {
                if k == 2 as libc::c_int {
                    RDmin_Q10 = psDelDec[0 as libc::c_int as usize].RD_Q10;
                    Winner_ind = 0 as libc::c_int;
                    i = 1 as libc::c_int;
                    while i < (*psEncC).nStatesDelayedDecision {
                        if psDelDec[i as usize].RD_Q10 < RDmin_Q10 {
                            RDmin_Q10 = psDelDec[i as usize].RD_Q10;
                            Winner_ind = i;
                        }
                        i += 1;
                        i;
                    }
                    i = 0 as libc::c_int;
                    while i < (*psEncC).nStatesDelayedDecision {
                        if i != Winner_ind {
                            psDelDec[i as usize].RD_Q10
                                += 0x7fffffff as libc::c_int >> 4 as libc::c_int;
                        }
                        i += 1;
                        i;
                    }
                    psDD = &mut *psDelDec.as_mut_ptr().offset(Winner_ind as isize)
                        as *mut NSQ_del_dec_struct;
                    last_smple_idx = smpl_buf_idx + decisionDelay;
                    i = 0 as libc::c_int;
                    while i < decisionDelay {
                        last_smple_idx = last_smple_idx - 1 as libc::c_int
                            & 32 as libc::c_int - 1 as libc::c_int;
                        *q
                            .offset(
                                (i - decisionDelay) as isize,
                            ) = ((*psDD).Q_Q10[last_smple_idx as usize]
                            >> 10 as libc::c_int) as libc::c_schar;
                        *pxq
                            .offset(
                                (i - decisionDelay) as isize,
                            ) = (if (if 10 as libc::c_int == 1 as libc::c_int {
                            (((*psDD).Xq_Q10[last_smple_idx as usize]
                                >> 16 as libc::c_int)
                                * (*psDD).Gain_Q16[last_smple_idx as usize] as libc::c_short
                                    as libc::c_int
                                + (((*psDD).Xq_Q10[last_smple_idx as usize]
                                    & 0xffff as libc::c_int)
                                    * (*psDD).Gain_Q16[last_smple_idx as usize] as libc::c_short
                                        as libc::c_int >> 16 as libc::c_int)
                                + (*psDD).Xq_Q10[last_smple_idx as usize]
                                    * (if 16 as libc::c_int == 1 as libc::c_int {
                                        ((*psDD).Gain_Q16[last_smple_idx as usize]
                                            >> 1 as libc::c_int)
                                            + ((*psDD).Gain_Q16[last_smple_idx as usize]
                                                & 1 as libc::c_int)
                                    } else {
                                        ((*psDD).Gain_Q16[last_smple_idx as usize]
                                            >> 16 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                                            >> 1 as libc::c_int
                                    }) >> 1 as libc::c_int)
                                + (((*psDD).Xq_Q10[last_smple_idx as usize]
                                    >> 16 as libc::c_int)
                                    * (*psDD).Gain_Q16[last_smple_idx as usize] as libc::c_short
                                        as libc::c_int
                                    + (((*psDD).Xq_Q10[last_smple_idx as usize]
                                        & 0xffff as libc::c_int)
                                        * (*psDD).Gain_Q16[last_smple_idx as usize] as libc::c_short
                                            as libc::c_int >> 16 as libc::c_int)
                                    + (*psDD).Xq_Q10[last_smple_idx as usize]
                                        * (if 16 as libc::c_int == 1 as libc::c_int {
                                            ((*psDD).Gain_Q16[last_smple_idx as usize]
                                                >> 1 as libc::c_int)
                                                + ((*psDD).Gain_Q16[last_smple_idx as usize]
                                                    & 1 as libc::c_int)
                                        } else {
                                            ((*psDD).Gain_Q16[last_smple_idx as usize]
                                                >> 16 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                                                >> 1 as libc::c_int
                                        }) & 1 as libc::c_int)
                        } else {
                            (((*psDD).Xq_Q10[last_smple_idx as usize]
                                >> 16 as libc::c_int)
                                * (*psDD).Gain_Q16[last_smple_idx as usize] as libc::c_short
                                    as libc::c_int
                                + (((*psDD).Xq_Q10[last_smple_idx as usize]
                                    & 0xffff as libc::c_int)
                                    * (*psDD).Gain_Q16[last_smple_idx as usize] as libc::c_short
                                        as libc::c_int >> 16 as libc::c_int)
                                + (*psDD).Xq_Q10[last_smple_idx as usize]
                                    * (if 16 as libc::c_int == 1 as libc::c_int {
                                        ((*psDD).Gain_Q16[last_smple_idx as usize]
                                            >> 1 as libc::c_int)
                                            + ((*psDD).Gain_Q16[last_smple_idx as usize]
                                                & 1 as libc::c_int)
                                    } else {
                                        ((*psDD).Gain_Q16[last_smple_idx as usize]
                                            >> 16 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                                            >> 1 as libc::c_int
                                    }) >> 10 as libc::c_int - 1 as libc::c_int)
                                + 1 as libc::c_int >> 1 as libc::c_int
                        }) > 0x7fff as libc::c_int
                        {
                            0x7fff as libc::c_int
                        } else if (if 10 as libc::c_int == 1 as libc::c_int {
                            (((*psDD).Xq_Q10[last_smple_idx as usize]
                                >> 16 as libc::c_int)
                                * (*psDD).Gain_Q16[last_smple_idx as usize] as libc::c_short
                                    as libc::c_int
                                + (((*psDD).Xq_Q10[last_smple_idx as usize]
                                    & 0xffff as libc::c_int)
                                    * (*psDD).Gain_Q16[last_smple_idx as usize] as libc::c_short
                                        as libc::c_int >> 16 as libc::c_int)
                                + (*psDD).Xq_Q10[last_smple_idx as usize]
                                    * (if 16 as libc::c_int == 1 as libc::c_int {
                                        ((*psDD).Gain_Q16[last_smple_idx as usize]
                                            >> 1 as libc::c_int)
                                            + ((*psDD).Gain_Q16[last_smple_idx as usize]
                                                & 1 as libc::c_int)
                                    } else {
                                        ((*psDD).Gain_Q16[last_smple_idx as usize]
                                            >> 16 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                                            >> 1 as libc::c_int
                                    }) >> 1 as libc::c_int)
                                + (((*psDD).Xq_Q10[last_smple_idx as usize]
                                    >> 16 as libc::c_int)
                                    * (*psDD).Gain_Q16[last_smple_idx as usize] as libc::c_short
                                        as libc::c_int
                                    + (((*psDD).Xq_Q10[last_smple_idx as usize]
                                        & 0xffff as libc::c_int)
                                        * (*psDD).Gain_Q16[last_smple_idx as usize] as libc::c_short
                                            as libc::c_int >> 16 as libc::c_int)
                                    + (*psDD).Xq_Q10[last_smple_idx as usize]
                                        * (if 16 as libc::c_int == 1 as libc::c_int {
                                            ((*psDD).Gain_Q16[last_smple_idx as usize]
                                                >> 1 as libc::c_int)
                                                + ((*psDD).Gain_Q16[last_smple_idx as usize]
                                                    & 1 as libc::c_int)
                                        } else {
                                            ((*psDD).Gain_Q16[last_smple_idx as usize]
                                                >> 16 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                                                >> 1 as libc::c_int
                                        }) & 1 as libc::c_int)
                        } else {
                            (((*psDD).Xq_Q10[last_smple_idx as usize]
                                >> 16 as libc::c_int)
                                * (*psDD).Gain_Q16[last_smple_idx as usize] as libc::c_short
                                    as libc::c_int
                                + (((*psDD).Xq_Q10[last_smple_idx as usize]
                                    & 0xffff as libc::c_int)
                                    * (*psDD).Gain_Q16[last_smple_idx as usize] as libc::c_short
                                        as libc::c_int >> 16 as libc::c_int)
                                + (*psDD).Xq_Q10[last_smple_idx as usize]
                                    * (if 16 as libc::c_int == 1 as libc::c_int {
                                        ((*psDD).Gain_Q16[last_smple_idx as usize]
                                            >> 1 as libc::c_int)
                                            + ((*psDD).Gain_Q16[last_smple_idx as usize]
                                                & 1 as libc::c_int)
                                    } else {
                                        ((*psDD).Gain_Q16[last_smple_idx as usize]
                                            >> 16 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                                            >> 1 as libc::c_int
                                    }) >> 10 as libc::c_int - 1 as libc::c_int)
                                + 1 as libc::c_int >> 1 as libc::c_int
                        }) < 0x8000 as libc::c_int as libc::c_short as libc::c_int
                        {
                            0x8000 as libc::c_int as libc::c_short as libc::c_int
                        } else if 10 as libc::c_int == 1 as libc::c_int {
                            (((*psDD).Xq_Q10[last_smple_idx as usize]
                                >> 16 as libc::c_int)
                                * (*psDD).Gain_Q16[last_smple_idx as usize] as libc::c_short
                                    as libc::c_int
                                + (((*psDD).Xq_Q10[last_smple_idx as usize]
                                    & 0xffff as libc::c_int)
                                    * (*psDD).Gain_Q16[last_smple_idx as usize] as libc::c_short
                                        as libc::c_int >> 16 as libc::c_int)
                                + (*psDD).Xq_Q10[last_smple_idx as usize]
                                    * (if 16 as libc::c_int == 1 as libc::c_int {
                                        ((*psDD).Gain_Q16[last_smple_idx as usize]
                                            >> 1 as libc::c_int)
                                            + ((*psDD).Gain_Q16[last_smple_idx as usize]
                                                & 1 as libc::c_int)
                                    } else {
                                        ((*psDD).Gain_Q16[last_smple_idx as usize]
                                            >> 16 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                                            >> 1 as libc::c_int
                                    }) >> 1 as libc::c_int)
                                + (((*psDD).Xq_Q10[last_smple_idx as usize]
                                    >> 16 as libc::c_int)
                                    * (*psDD).Gain_Q16[last_smple_idx as usize] as libc::c_short
                                        as libc::c_int
                                    + (((*psDD).Xq_Q10[last_smple_idx as usize]
                                        & 0xffff as libc::c_int)
                                        * (*psDD).Gain_Q16[last_smple_idx as usize] as libc::c_short
                                            as libc::c_int >> 16 as libc::c_int)
                                    + (*psDD).Xq_Q10[last_smple_idx as usize]
                                        * (if 16 as libc::c_int == 1 as libc::c_int {
                                            ((*psDD).Gain_Q16[last_smple_idx as usize]
                                                >> 1 as libc::c_int)
                                                + ((*psDD).Gain_Q16[last_smple_idx as usize]
                                                    & 1 as libc::c_int)
                                        } else {
                                            ((*psDD).Gain_Q16[last_smple_idx as usize]
                                                >> 16 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                                                >> 1 as libc::c_int
                                        }) & 1 as libc::c_int)
                        } else {
                            (((*psDD).Xq_Q10[last_smple_idx as usize]
                                >> 16 as libc::c_int)
                                * (*psDD).Gain_Q16[last_smple_idx as usize] as libc::c_short
                                    as libc::c_int
                                + (((*psDD).Xq_Q10[last_smple_idx as usize]
                                    & 0xffff as libc::c_int)
                                    * (*psDD).Gain_Q16[last_smple_idx as usize] as libc::c_short
                                        as libc::c_int >> 16 as libc::c_int)
                                + (*psDD).Xq_Q10[last_smple_idx as usize]
                                    * (if 16 as libc::c_int == 1 as libc::c_int {
                                        ((*psDD).Gain_Q16[last_smple_idx as usize]
                                            >> 1 as libc::c_int)
                                            + ((*psDD).Gain_Q16[last_smple_idx as usize]
                                                & 1 as libc::c_int)
                                    } else {
                                        ((*psDD).Gain_Q16[last_smple_idx as usize]
                                            >> 16 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                                            >> 1 as libc::c_int
                                    }) >> 10 as libc::c_int - 1 as libc::c_int)
                                + 1 as libc::c_int >> 1 as libc::c_int
                        }) as libc::c_short;
                        (*NSQ)
                            .sLTP_shp_Q10[((*NSQ).sLTP_shp_buf_idx - decisionDelay + i)
                            as usize] = (*psDD).Shape_Q10[last_smple_idx as usize];
                        i += 1;
                        i;
                    }
                    subfr = 0 as libc::c_int;
                }
                start_idx = (*psEncC).frame_length - lag - (*psEncC).predictLPCOrder
                    - 5 as libc::c_int / 2 as libc::c_int;
                memset(
                    FiltState.as_mut_ptr() as *mut libc::c_void,
                    0 as libc::c_int,
                    ((*psEncC).predictLPCOrder as libc::c_ulong)
                        .wrapping_mul(
                            ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                        ),
                );
                SKP_Silk_MA_Prediction(
                    &mut *((*NSQ).xq)
                        .as_mut_ptr()
                        .offset((start_idx + k * (*psEncC).subfr_length) as isize),
                    A_Q12,
                    FiltState.as_mut_ptr(),
                    sLTP.as_mut_ptr().offset(start_idx as isize),
                    (*psEncC).frame_length - start_idx,
                    (*psEncC).predictLPCOrder,
                );
                (*NSQ).sLTP_buf_idx = (*psEncC).frame_length;
                (*NSQ).rewhite_flag = 1 as libc::c_int;
            }
        }
        SKP_Silk_nsq_del_dec_scale_states(
            NSQ,
            psDelDec.as_mut_ptr(),
            x,
            x_sc_Q10.as_mut_ptr(),
            subfr_length,
            sLTP.as_mut_ptr() as *const libc::c_short,
            sLTP_Q16.as_mut_ptr(),
            k,
            (*psEncC).nStatesDelayedDecision,
            smpl_buf_idx,
            LTP_scale_Q14,
            Gains_Q16,
            ((*psEncCtrlC).pitchL).as_mut_ptr() as *const libc::c_int,
        );
        let fresh0 = subfr;
        subfr = subfr + 1;
        SKP_Silk_noise_shape_quantizer_del_dec(
            NSQ,
            psDelDec.as_mut_ptr(),
            (*psEncCtrlC).sigtype,
            x_sc_Q10.as_mut_ptr() as *const libc::c_int,
            q,
            pxq,
            sLTP_Q16.as_mut_ptr(),
            A_Q12,
            B_Q14,
            AR_shp_Q13,
            lag,
            HarmShapeFIRPacked_Q14,
            *Tilt_Q14.offset(k as isize),
            *LF_shp_Q14.offset(k as isize),
            *Gains_Q16.offset(k as isize),
            Lambda_Q10,
            offset_Q10,
            (*psEncC).subfr_length,
            fresh0,
            (*psEncC).shapingLPCOrder,
            (*psEncC).predictLPCOrder,
            (*psEncC).warping_Q16,
            (*psEncC).nStatesDelayedDecision,
            &mut smpl_buf_idx,
            decisionDelay,
        );
        x = x.offset((*psEncC).subfr_length as isize);
        q = q.offset((*psEncC).subfr_length as isize);
        pxq = pxq.offset((*psEncC).subfr_length as isize);
        k += 1;
        k;
    }
    RDmin_Q10 = psDelDec[0 as libc::c_int as usize].RD_Q10;
    Winner_ind = 0 as libc::c_int;
    k = 1 as libc::c_int;
    while k < (*psEncC).nStatesDelayedDecision {
        if psDelDec[k as usize].RD_Q10 < RDmin_Q10 {
            RDmin_Q10 = psDelDec[k as usize].RD_Q10;
            Winner_ind = k;
        }
        k += 1;
        k;
    }
    psDD = &mut *psDelDec.as_mut_ptr().offset(Winner_ind as isize)
        as *mut NSQ_del_dec_struct;
    (*psEncCtrlC).Seed = (*psDD).SeedInit;
    last_smple_idx = smpl_buf_idx + decisionDelay;
    i = 0 as libc::c_int;
    while i < decisionDelay {
        last_smple_idx = last_smple_idx - 1 as libc::c_int
            & 32 as libc::c_int - 1 as libc::c_int;
        *q
            .offset(
                (i - decisionDelay) as isize,
            ) = ((*psDD).Q_Q10[last_smple_idx as usize] >> 10 as libc::c_int)
            as libc::c_schar;
        *pxq
            .offset(
                (i - decisionDelay) as isize,
            ) = (if (if 10 as libc::c_int == 1 as libc::c_int {
            (((*psDD).Xq_Q10[last_smple_idx as usize] >> 16 as libc::c_int)
                * (*psDD).Gain_Q16[last_smple_idx as usize] as libc::c_short
                    as libc::c_int
                + (((*psDD).Xq_Q10[last_smple_idx as usize] & 0xffff as libc::c_int)
                    * (*psDD).Gain_Q16[last_smple_idx as usize] as libc::c_short
                        as libc::c_int >> 16 as libc::c_int)
                + (*psDD).Xq_Q10[last_smple_idx as usize]
                    * (if 16 as libc::c_int == 1 as libc::c_int {
                        ((*psDD).Gain_Q16[last_smple_idx as usize] >> 1 as libc::c_int)
                            + ((*psDD).Gain_Q16[last_smple_idx as usize]
                                & 1 as libc::c_int)
                    } else {
                        ((*psDD).Gain_Q16[last_smple_idx as usize]
                            >> 16 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                            >> 1 as libc::c_int
                    }) >> 1 as libc::c_int)
                + (((*psDD).Xq_Q10[last_smple_idx as usize] >> 16 as libc::c_int)
                    * (*psDD).Gain_Q16[last_smple_idx as usize] as libc::c_short
                        as libc::c_int
                    + (((*psDD).Xq_Q10[last_smple_idx as usize] & 0xffff as libc::c_int)
                        * (*psDD).Gain_Q16[last_smple_idx as usize] as libc::c_short
                            as libc::c_int >> 16 as libc::c_int)
                    + (*psDD).Xq_Q10[last_smple_idx as usize]
                        * (if 16 as libc::c_int == 1 as libc::c_int {
                            ((*psDD).Gain_Q16[last_smple_idx as usize]
                                >> 1 as libc::c_int)
                                + ((*psDD).Gain_Q16[last_smple_idx as usize]
                                    & 1 as libc::c_int)
                        } else {
                            ((*psDD).Gain_Q16[last_smple_idx as usize]
                                >> 16 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                                >> 1 as libc::c_int
                        }) & 1 as libc::c_int)
        } else {
            (((*psDD).Xq_Q10[last_smple_idx as usize] >> 16 as libc::c_int)
                * (*psDD).Gain_Q16[last_smple_idx as usize] as libc::c_short
                    as libc::c_int
                + (((*psDD).Xq_Q10[last_smple_idx as usize] & 0xffff as libc::c_int)
                    * (*psDD).Gain_Q16[last_smple_idx as usize] as libc::c_short
                        as libc::c_int >> 16 as libc::c_int)
                + (*psDD).Xq_Q10[last_smple_idx as usize]
                    * (if 16 as libc::c_int == 1 as libc::c_int {
                        ((*psDD).Gain_Q16[last_smple_idx as usize] >> 1 as libc::c_int)
                            + ((*psDD).Gain_Q16[last_smple_idx as usize]
                                & 1 as libc::c_int)
                    } else {
                        ((*psDD).Gain_Q16[last_smple_idx as usize]
                            >> 16 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                            >> 1 as libc::c_int
                    }) >> 10 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                >> 1 as libc::c_int
        }) > 0x7fff as libc::c_int
        {
            0x7fff as libc::c_int
        } else if (if 10 as libc::c_int == 1 as libc::c_int {
            (((*psDD).Xq_Q10[last_smple_idx as usize] >> 16 as libc::c_int)
                * (*psDD).Gain_Q16[last_smple_idx as usize] as libc::c_short
                    as libc::c_int
                + (((*psDD).Xq_Q10[last_smple_idx as usize] & 0xffff as libc::c_int)
                    * (*psDD).Gain_Q16[last_smple_idx as usize] as libc::c_short
                        as libc::c_int >> 16 as libc::c_int)
                + (*psDD).Xq_Q10[last_smple_idx as usize]
                    * (if 16 as libc::c_int == 1 as libc::c_int {
                        ((*psDD).Gain_Q16[last_smple_idx as usize] >> 1 as libc::c_int)
                            + ((*psDD).Gain_Q16[last_smple_idx as usize]
                                & 1 as libc::c_int)
                    } else {
                        ((*psDD).Gain_Q16[last_smple_idx as usize]
                            >> 16 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                            >> 1 as libc::c_int
                    }) >> 1 as libc::c_int)
                + (((*psDD).Xq_Q10[last_smple_idx as usize] >> 16 as libc::c_int)
                    * (*psDD).Gain_Q16[last_smple_idx as usize] as libc::c_short
                        as libc::c_int
                    + (((*psDD).Xq_Q10[last_smple_idx as usize] & 0xffff as libc::c_int)
                        * (*psDD).Gain_Q16[last_smple_idx as usize] as libc::c_short
                            as libc::c_int >> 16 as libc::c_int)
                    + (*psDD).Xq_Q10[last_smple_idx as usize]
                        * (if 16 as libc::c_int == 1 as libc::c_int {
                            ((*psDD).Gain_Q16[last_smple_idx as usize]
                                >> 1 as libc::c_int)
                                + ((*psDD).Gain_Q16[last_smple_idx as usize]
                                    & 1 as libc::c_int)
                        } else {
                            ((*psDD).Gain_Q16[last_smple_idx as usize]
                                >> 16 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                                >> 1 as libc::c_int
                        }) & 1 as libc::c_int)
        } else {
            (((*psDD).Xq_Q10[last_smple_idx as usize] >> 16 as libc::c_int)
                * (*psDD).Gain_Q16[last_smple_idx as usize] as libc::c_short
                    as libc::c_int
                + (((*psDD).Xq_Q10[last_smple_idx as usize] & 0xffff as libc::c_int)
                    * (*psDD).Gain_Q16[last_smple_idx as usize] as libc::c_short
                        as libc::c_int >> 16 as libc::c_int)
                + (*psDD).Xq_Q10[last_smple_idx as usize]
                    * (if 16 as libc::c_int == 1 as libc::c_int {
                        ((*psDD).Gain_Q16[last_smple_idx as usize] >> 1 as libc::c_int)
                            + ((*psDD).Gain_Q16[last_smple_idx as usize]
                                & 1 as libc::c_int)
                    } else {
                        ((*psDD).Gain_Q16[last_smple_idx as usize]
                            >> 16 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                            >> 1 as libc::c_int
                    }) >> 10 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                >> 1 as libc::c_int
        }) < 0x8000 as libc::c_int as libc::c_short as libc::c_int
        {
            0x8000 as libc::c_int as libc::c_short as libc::c_int
        } else if 10 as libc::c_int == 1 as libc::c_int {
            (((*psDD).Xq_Q10[last_smple_idx as usize] >> 16 as libc::c_int)
                * (*psDD).Gain_Q16[last_smple_idx as usize] as libc::c_short
                    as libc::c_int
                + (((*psDD).Xq_Q10[last_smple_idx as usize] & 0xffff as libc::c_int)
                    * (*psDD).Gain_Q16[last_smple_idx as usize] as libc::c_short
                        as libc::c_int >> 16 as libc::c_int)
                + (*psDD).Xq_Q10[last_smple_idx as usize]
                    * (if 16 as libc::c_int == 1 as libc::c_int {
                        ((*psDD).Gain_Q16[last_smple_idx as usize] >> 1 as libc::c_int)
                            + ((*psDD).Gain_Q16[last_smple_idx as usize]
                                & 1 as libc::c_int)
                    } else {
                        ((*psDD).Gain_Q16[last_smple_idx as usize]
                            >> 16 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                            >> 1 as libc::c_int
                    }) >> 1 as libc::c_int)
                + (((*psDD).Xq_Q10[last_smple_idx as usize] >> 16 as libc::c_int)
                    * (*psDD).Gain_Q16[last_smple_idx as usize] as libc::c_short
                        as libc::c_int
                    + (((*psDD).Xq_Q10[last_smple_idx as usize] & 0xffff as libc::c_int)
                        * (*psDD).Gain_Q16[last_smple_idx as usize] as libc::c_short
                            as libc::c_int >> 16 as libc::c_int)
                    + (*psDD).Xq_Q10[last_smple_idx as usize]
                        * (if 16 as libc::c_int == 1 as libc::c_int {
                            ((*psDD).Gain_Q16[last_smple_idx as usize]
                                >> 1 as libc::c_int)
                                + ((*psDD).Gain_Q16[last_smple_idx as usize]
                                    & 1 as libc::c_int)
                        } else {
                            ((*psDD).Gain_Q16[last_smple_idx as usize]
                                >> 16 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                                >> 1 as libc::c_int
                        }) & 1 as libc::c_int)
        } else {
            (((*psDD).Xq_Q10[last_smple_idx as usize] >> 16 as libc::c_int)
                * (*psDD).Gain_Q16[last_smple_idx as usize] as libc::c_short
                    as libc::c_int
                + (((*psDD).Xq_Q10[last_smple_idx as usize] & 0xffff as libc::c_int)
                    * (*psDD).Gain_Q16[last_smple_idx as usize] as libc::c_short
                        as libc::c_int >> 16 as libc::c_int)
                + (*psDD).Xq_Q10[last_smple_idx as usize]
                    * (if 16 as libc::c_int == 1 as libc::c_int {
                        ((*psDD).Gain_Q16[last_smple_idx as usize] >> 1 as libc::c_int)
                            + ((*psDD).Gain_Q16[last_smple_idx as usize]
                                & 1 as libc::c_int)
                    } else {
                        ((*psDD).Gain_Q16[last_smple_idx as usize]
                            >> 16 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                            >> 1 as libc::c_int
                    }) >> 10 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                >> 1 as libc::c_int
        }) as libc::c_short;
        (*NSQ)
            .sLTP_shp_Q10[((*NSQ).sLTP_shp_buf_idx - decisionDelay + i)
            as usize] = (*psDD).Shape_Q10[last_smple_idx as usize];
        sLTP_Q16[((*NSQ).sLTP_buf_idx - decisionDelay + i)
            as usize] = (*psDD).Pred_Q16[last_smple_idx as usize];
        i += 1;
        i;
    }
    memcpy(
        ((*NSQ).sLPC_Q14).as_mut_ptr() as *mut libc::c_void,
        &mut *((*psDD).sLPC_Q14).as_mut_ptr().offset((*psEncC).subfr_length as isize)
            as *mut libc::c_int as *const libc::c_void,
        (32 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    memcpy(
        ((*NSQ).sAR2_Q14).as_mut_ptr() as *mut libc::c_void,
        ((*psDD).sAR2_Q14).as_mut_ptr() as *const libc::c_void,
        ::core::mem::size_of::<[libc::c_int; 16]>() as libc::c_ulong,
    );
    (*NSQ).sLF_AR_shp_Q12 = (*psDD).LF_AR_Q12;
    (*NSQ)
        .lagPrev = (*psEncCtrlC).pitchL[(4 as libc::c_int - 1 as libc::c_int) as usize];
    memcpy(
        ((*NSQ).xq).as_mut_ptr() as *mut libc::c_void,
        &mut *((*NSQ).xq).as_mut_ptr().offset((*psEncC).frame_length as isize)
            as *mut libc::c_short as *const libc::c_void,
        ((*psEncC).frame_length as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as libc::c_ulong),
    );
    memcpy(
        ((*NSQ).sLTP_shp_Q10).as_mut_ptr() as *mut libc::c_void,
        &mut *((*NSQ).sLTP_shp_Q10).as_mut_ptr().offset((*psEncC).frame_length as isize)
            as *mut libc::c_int as *const libc::c_void,
        ((*psEncC).frame_length as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
}
#[inline]
unsafe extern "C" fn SKP_Silk_noise_shape_quantizer_del_dec(
    mut NSQ: *mut SKP_Silk_nsq_state,
    mut psDelDec: *mut NSQ_del_dec_struct,
    mut sigtype: libc::c_int,
    mut x_Q10: *const libc::c_int,
    mut q: *mut libc::c_schar,
    mut xq: *mut libc::c_short,
    mut sLTP_Q16: *mut libc::c_int,
    mut a_Q12: *const libc::c_short,
    mut b_Q14: *const libc::c_short,
    mut AR_shp_Q13: *const libc::c_short,
    mut lag: libc::c_int,
    mut HarmShapeFIRPacked_Q14: libc::c_int,
    mut Tilt_Q14: libc::c_int,
    mut LF_shp_Q14: libc::c_int,
    mut Gain_Q16: libc::c_int,
    mut Lambda_Q10: libc::c_int,
    mut offset_Q10: libc::c_int,
    mut length: libc::c_int,
    mut subfr: libc::c_int,
    mut shapingLPCOrder: libc::c_int,
    mut predictLPCOrder: libc::c_int,
    mut warping_Q16: libc::c_int,
    mut nStatesDelayedDecision: libc::c_int,
    mut smpl_buf_idx: *mut libc::c_int,
    mut decisionDelay: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut Winner_ind: libc::c_int = 0;
    let mut RDmin_ind: libc::c_int = 0;
    let mut RDmax_ind: libc::c_int = 0;
    let mut last_smple_idx: libc::c_int = 0;
    let mut Winner_rand_state: libc::c_int = 0;
    let mut LTP_pred_Q14: libc::c_int = 0;
    let mut LPC_pred_Q10: libc::c_int = 0;
    let mut n_AR_Q10: libc::c_int = 0;
    let mut n_LTP_Q14: libc::c_int = 0;
    let mut n_LF_Q10: libc::c_int = 0;
    let mut r_Q10: libc::c_int = 0;
    let mut rr_Q20: libc::c_int = 0;
    let mut rd1_Q10: libc::c_int = 0;
    let mut rd2_Q10: libc::c_int = 0;
    let mut RDmin_Q10: libc::c_int = 0;
    let mut RDmax_Q10: libc::c_int = 0;
    let mut q1_Q10: libc::c_int = 0;
    let mut q2_Q10: libc::c_int = 0;
    let mut dither: libc::c_int = 0;
    let mut exc_Q10: libc::c_int = 0;
    let mut LPC_exc_Q10: libc::c_int = 0;
    let mut xq_Q10: libc::c_int = 0;
    let mut tmp1: libc::c_int = 0;
    let mut tmp2: libc::c_int = 0;
    let mut sLF_AR_shp_Q10: libc::c_int = 0;
    let mut pred_lag_ptr: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut shp_lag_ptr: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut psLPC_Q14: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut psSampleState: [[NSQ_sample_struct; 2]; 4] = [[NSQ_sample_struct {
        Q_Q10: 0,
        RD_Q10: 0,
        xq_Q14: 0,
        LF_AR_Q12: 0,
        sLTP_shp_Q10: 0,
        LPC_exc_Q16: 0,
    }; 2]; 4];
    let mut psDD: *mut NSQ_del_dec_struct = 0 as *mut NSQ_del_dec_struct;
    let mut psSS: *mut NSQ_sample_struct = 0 as *mut NSQ_sample_struct;
    let mut a_Q12_tmp: [libc::c_int; 8] = [0; 8];
    let mut Atmp: libc::c_int = 0;
    memcpy(
        a_Q12_tmp.as_mut_ptr() as *mut libc::c_void,
        a_Q12 as *const libc::c_void,
        (predictLPCOrder as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as libc::c_ulong),
    );
    shp_lag_ptr = &mut *((*NSQ).sLTP_shp_Q10)
        .as_mut_ptr()
        .offset(
            ((*NSQ).sLTP_shp_buf_idx - lag + 3 as libc::c_int / 2 as libc::c_int)
                as isize,
        ) as *mut libc::c_int;
    pred_lag_ptr = &mut *sLTP_Q16
        .offset(
            ((*NSQ).sLTP_buf_idx - lag + 5 as libc::c_int / 2 as libc::c_int) as isize,
        ) as *mut libc::c_int;
    i = 0 as libc::c_int;
    while i < length {
        if sigtype == 0 as libc::c_int {
            LTP_pred_Q14 = (*pred_lag_ptr.offset(0 as libc::c_int as isize)
                >> 16 as libc::c_int)
                * *b_Q14.offset(0 as libc::c_int as isize) as libc::c_int
                + ((*pred_lag_ptr.offset(0 as libc::c_int as isize)
                    & 0xffff as libc::c_int)
                    * *b_Q14.offset(0 as libc::c_int as isize) as libc::c_int
                    >> 16 as libc::c_int);
            LTP_pred_Q14 = LTP_pred_Q14
                + ((*pred_lag_ptr.offset(-(1 as libc::c_int) as isize)
                    >> 16 as libc::c_int)
                    * *b_Q14.offset(1 as libc::c_int as isize) as libc::c_int
                    + ((*pred_lag_ptr.offset(-(1 as libc::c_int) as isize)
                        & 0xffff as libc::c_int)
                        * *b_Q14.offset(1 as libc::c_int as isize) as libc::c_int
                        >> 16 as libc::c_int));
            LTP_pred_Q14 = LTP_pred_Q14
                + ((*pred_lag_ptr.offset(-(2 as libc::c_int) as isize)
                    >> 16 as libc::c_int)
                    * *b_Q14.offset(2 as libc::c_int as isize) as libc::c_int
                    + ((*pred_lag_ptr.offset(-(2 as libc::c_int) as isize)
                        & 0xffff as libc::c_int)
                        * *b_Q14.offset(2 as libc::c_int as isize) as libc::c_int
                        >> 16 as libc::c_int));
            LTP_pred_Q14 = LTP_pred_Q14
                + ((*pred_lag_ptr.offset(-(3 as libc::c_int) as isize)
                    >> 16 as libc::c_int)
                    * *b_Q14.offset(3 as libc::c_int as isize) as libc::c_int
                    + ((*pred_lag_ptr.offset(-(3 as libc::c_int) as isize)
                        & 0xffff as libc::c_int)
                        * *b_Q14.offset(3 as libc::c_int as isize) as libc::c_int
                        >> 16 as libc::c_int));
            LTP_pred_Q14 = LTP_pred_Q14
                + ((*pred_lag_ptr.offset(-(4 as libc::c_int) as isize)
                    >> 16 as libc::c_int)
                    * *b_Q14.offset(4 as libc::c_int as isize) as libc::c_int
                    + ((*pred_lag_ptr.offset(-(4 as libc::c_int) as isize)
                        & 0xffff as libc::c_int)
                        * *b_Q14.offset(4 as libc::c_int as isize) as libc::c_int
                        >> 16 as libc::c_int));
            pred_lag_ptr = pred_lag_ptr.offset(1);
            pred_lag_ptr;
        } else {
            LTP_pred_Q14 = 0 as libc::c_int;
        }
        if lag > 0 as libc::c_int {
            n_LTP_Q14 = (*shp_lag_ptr.offset(0 as libc::c_int as isize)
                + *shp_lag_ptr.offset(-(2 as libc::c_int) as isize) >> 16 as libc::c_int)
                * HarmShapeFIRPacked_Q14 as libc::c_short as libc::c_int
                + ((*shp_lag_ptr.offset(0 as libc::c_int as isize)
                    + *shp_lag_ptr.offset(-(2 as libc::c_int) as isize)
                    & 0xffff as libc::c_int)
                    * HarmShapeFIRPacked_Q14 as libc::c_short as libc::c_int
                    >> 16 as libc::c_int);
            n_LTP_Q14 = n_LTP_Q14
                + (*shp_lag_ptr.offset(-(1 as libc::c_int) as isize)
                    >> 16 as libc::c_int) * (HarmShapeFIRPacked_Q14 >> 16 as libc::c_int)
                + ((*shp_lag_ptr.offset(-(1 as libc::c_int) as isize)
                    & 0xffff as libc::c_int)
                    * (HarmShapeFIRPacked_Q14 >> 16 as libc::c_int)
                    >> 16 as libc::c_int);
            n_LTP_Q14 = n_LTP_Q14 << 6 as libc::c_int;
            shp_lag_ptr = shp_lag_ptr.offset(1);
            shp_lag_ptr;
        } else {
            n_LTP_Q14 = 0 as libc::c_int;
        }
        k = 0 as libc::c_int;
        while k < nStatesDelayedDecision {
            psDD = &mut *psDelDec.offset(k as isize) as *mut NSQ_del_dec_struct;
            psSS = (psSampleState[k as usize]).as_mut_ptr();
            (*psDD)
                .Seed = (907633515 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    ((*psDD).Seed as libc::c_uint)
                        .wrapping_mul(196314165 as libc::c_int as libc::c_uint),
                ) as libc::c_int;
            dither = (*psDD).Seed >> 31 as libc::c_int;
            psLPC_Q14 = &mut *((*psDD).sLPC_Q14)
                .as_mut_ptr()
                .offset((32 as libc::c_int - 1 as libc::c_int + i) as isize)
                as *mut libc::c_int;
            Atmp = a_Q12_tmp[0 as libc::c_int as usize];
            LPC_pred_Q10 = (*psLPC_Q14.offset(0 as libc::c_int as isize)
                >> 16 as libc::c_int) * Atmp as libc::c_short as libc::c_int
                + ((*psLPC_Q14.offset(0 as libc::c_int as isize) & 0xffff as libc::c_int)
                    * Atmp as libc::c_short as libc::c_int >> 16 as libc::c_int);
            LPC_pred_Q10 = LPC_pred_Q10
                + (*psLPC_Q14.offset(-(1 as libc::c_int) as isize) >> 16 as libc::c_int)
                    * (Atmp >> 16 as libc::c_int)
                + ((*psLPC_Q14.offset(-(1 as libc::c_int) as isize)
                    & 0xffff as libc::c_int) * (Atmp >> 16 as libc::c_int)
                    >> 16 as libc::c_int);
            Atmp = a_Q12_tmp[1 as libc::c_int as usize];
            LPC_pred_Q10 = LPC_pred_Q10
                + ((*psLPC_Q14.offset(-(2 as libc::c_int) as isize) >> 16 as libc::c_int)
                    * Atmp as libc::c_short as libc::c_int
                    + ((*psLPC_Q14.offset(-(2 as libc::c_int) as isize)
                        & 0xffff as libc::c_int) * Atmp as libc::c_short as libc::c_int
                        >> 16 as libc::c_int));
            LPC_pred_Q10 = LPC_pred_Q10
                + (*psLPC_Q14.offset(-(3 as libc::c_int) as isize) >> 16 as libc::c_int)
                    * (Atmp >> 16 as libc::c_int)
                + ((*psLPC_Q14.offset(-(3 as libc::c_int) as isize)
                    & 0xffff as libc::c_int) * (Atmp >> 16 as libc::c_int)
                    >> 16 as libc::c_int);
            Atmp = a_Q12_tmp[2 as libc::c_int as usize];
            LPC_pred_Q10 = LPC_pred_Q10
                + ((*psLPC_Q14.offset(-(4 as libc::c_int) as isize) >> 16 as libc::c_int)
                    * Atmp as libc::c_short as libc::c_int
                    + ((*psLPC_Q14.offset(-(4 as libc::c_int) as isize)
                        & 0xffff as libc::c_int) * Atmp as libc::c_short as libc::c_int
                        >> 16 as libc::c_int));
            LPC_pred_Q10 = LPC_pred_Q10
                + (*psLPC_Q14.offset(-(5 as libc::c_int) as isize) >> 16 as libc::c_int)
                    * (Atmp >> 16 as libc::c_int)
                + ((*psLPC_Q14.offset(-(5 as libc::c_int) as isize)
                    & 0xffff as libc::c_int) * (Atmp >> 16 as libc::c_int)
                    >> 16 as libc::c_int);
            Atmp = a_Q12_tmp[3 as libc::c_int as usize];
            LPC_pred_Q10 = LPC_pred_Q10
                + ((*psLPC_Q14.offset(-(6 as libc::c_int) as isize) >> 16 as libc::c_int)
                    * Atmp as libc::c_short as libc::c_int
                    + ((*psLPC_Q14.offset(-(6 as libc::c_int) as isize)
                        & 0xffff as libc::c_int) * Atmp as libc::c_short as libc::c_int
                        >> 16 as libc::c_int));
            LPC_pred_Q10 = LPC_pred_Q10
                + (*psLPC_Q14.offset(-(7 as libc::c_int) as isize) >> 16 as libc::c_int)
                    * (Atmp >> 16 as libc::c_int)
                + ((*psLPC_Q14.offset(-(7 as libc::c_int) as isize)
                    & 0xffff as libc::c_int) * (Atmp >> 16 as libc::c_int)
                    >> 16 as libc::c_int);
            Atmp = a_Q12_tmp[4 as libc::c_int as usize];
            LPC_pred_Q10 = LPC_pred_Q10
                + ((*psLPC_Q14.offset(-(8 as libc::c_int) as isize) >> 16 as libc::c_int)
                    * Atmp as libc::c_short as libc::c_int
                    + ((*psLPC_Q14.offset(-(8 as libc::c_int) as isize)
                        & 0xffff as libc::c_int) * Atmp as libc::c_short as libc::c_int
                        >> 16 as libc::c_int));
            LPC_pred_Q10 = LPC_pred_Q10
                + (*psLPC_Q14.offset(-(9 as libc::c_int) as isize) >> 16 as libc::c_int)
                    * (Atmp >> 16 as libc::c_int)
                + ((*psLPC_Q14.offset(-(9 as libc::c_int) as isize)
                    & 0xffff as libc::c_int) * (Atmp >> 16 as libc::c_int)
                    >> 16 as libc::c_int);
            j = 10 as libc::c_int;
            while j < predictLPCOrder {
                Atmp = a_Q12_tmp[(j >> 1 as libc::c_int) as usize];
                LPC_pred_Q10 = LPC_pred_Q10
                    + ((*psLPC_Q14.offset(-j as isize) >> 16 as libc::c_int)
                        * Atmp as libc::c_short as libc::c_int
                        + ((*psLPC_Q14.offset(-j as isize) & 0xffff as libc::c_int)
                            * Atmp as libc::c_short as libc::c_int
                            >> 16 as libc::c_int));
                LPC_pred_Q10 = LPC_pred_Q10
                    + (*psLPC_Q14.offset((-j - 1 as libc::c_int) as isize)
                        >> 16 as libc::c_int) * (Atmp >> 16 as libc::c_int)
                    + ((*psLPC_Q14.offset((-j - 1 as libc::c_int) as isize)
                        & 0xffff as libc::c_int) * (Atmp >> 16 as libc::c_int)
                        >> 16 as libc::c_int);
                j += 2 as libc::c_int;
            }
            tmp2 = *psLPC_Q14.offset(0 as libc::c_int as isize)
                + (((*psDD).sAR2_Q14[0 as libc::c_int as usize] >> 16 as libc::c_int)
                    * warping_Q16 as libc::c_short as libc::c_int
                    + (((*psDD).sAR2_Q14[0 as libc::c_int as usize]
                        & 0xffff as libc::c_int)
                        * warping_Q16 as libc::c_short as libc::c_int
                        >> 16 as libc::c_int));
            tmp1 = (*psDD).sAR2_Q14[0 as libc::c_int as usize]
                + (((*psDD).sAR2_Q14[1 as libc::c_int as usize] - tmp2
                    >> 16 as libc::c_int) * warping_Q16 as libc::c_short as libc::c_int
                    + (((*psDD).sAR2_Q14[1 as libc::c_int as usize] - tmp2
                        & 0xffff as libc::c_int)
                        * warping_Q16 as libc::c_short as libc::c_int
                        >> 16 as libc::c_int));
            (*psDD).sAR2_Q14[0 as libc::c_int as usize] = tmp2;
            n_AR_Q10 = (tmp2 >> 16 as libc::c_int)
                * *AR_shp_Q13.offset(0 as libc::c_int as isize) as libc::c_int
                + ((tmp2 & 0xffff as libc::c_int)
                    * *AR_shp_Q13.offset(0 as libc::c_int as isize) as libc::c_int
                    >> 16 as libc::c_int);
            j = 2 as libc::c_int;
            while j < shapingLPCOrder {
                tmp2 = (*psDD).sAR2_Q14[(j - 1 as libc::c_int) as usize]
                    + (((*psDD).sAR2_Q14[(j + 0 as libc::c_int) as usize] - tmp1
                        >> 16 as libc::c_int)
                        * warping_Q16 as libc::c_short as libc::c_int
                        + (((*psDD).sAR2_Q14[(j + 0 as libc::c_int) as usize] - tmp1
                            & 0xffff as libc::c_int)
                            * warping_Q16 as libc::c_short as libc::c_int
                            >> 16 as libc::c_int));
                (*psDD).sAR2_Q14[(j - 1 as libc::c_int) as usize] = tmp1;
                n_AR_Q10 = n_AR_Q10
                    + ((tmp1 >> 16 as libc::c_int)
                        * *AR_shp_Q13.offset((j - 1 as libc::c_int) as isize)
                            as libc::c_int
                        + ((tmp1 & 0xffff as libc::c_int)
                            * *AR_shp_Q13.offset((j - 1 as libc::c_int) as isize)
                                as libc::c_int >> 16 as libc::c_int));
                tmp1 = (*psDD).sAR2_Q14[(j + 0 as libc::c_int) as usize]
                    + (((*psDD).sAR2_Q14[(j + 1 as libc::c_int) as usize] - tmp2
                        >> 16 as libc::c_int)
                        * warping_Q16 as libc::c_short as libc::c_int
                        + (((*psDD).sAR2_Q14[(j + 1 as libc::c_int) as usize] - tmp2
                            & 0xffff as libc::c_int)
                            * warping_Q16 as libc::c_short as libc::c_int
                            >> 16 as libc::c_int));
                (*psDD).sAR2_Q14[(j + 0 as libc::c_int) as usize] = tmp2;
                n_AR_Q10 = n_AR_Q10
                    + ((tmp2 >> 16 as libc::c_int)
                        * *AR_shp_Q13.offset(j as isize) as libc::c_int
                        + ((tmp2 & 0xffff as libc::c_int)
                            * *AR_shp_Q13.offset(j as isize) as libc::c_int
                            >> 16 as libc::c_int));
                j += 2 as libc::c_int;
            }
            (*psDD).sAR2_Q14[(shapingLPCOrder - 1 as libc::c_int) as usize] = tmp1;
            n_AR_Q10 = n_AR_Q10
                + ((tmp1 >> 16 as libc::c_int)
                    * *AR_shp_Q13.offset((shapingLPCOrder - 1 as libc::c_int) as isize)
                        as libc::c_int
                    + ((tmp1 & 0xffff as libc::c_int)
                        * *AR_shp_Q13
                            .offset((shapingLPCOrder - 1 as libc::c_int) as isize)
                            as libc::c_int >> 16 as libc::c_int));
            n_AR_Q10 = n_AR_Q10 >> 1 as libc::c_int;
            n_AR_Q10 = n_AR_Q10
                + (((*psDD).LF_AR_Q12 >> 16 as libc::c_int)
                    * Tilt_Q14 as libc::c_short as libc::c_int
                    + (((*psDD).LF_AR_Q12 & 0xffff as libc::c_int)
                        * Tilt_Q14 as libc::c_short as libc::c_int
                        >> 16 as libc::c_int));
            n_LF_Q10 = ((*psDD).Shape_Q10[*smpl_buf_idx as usize] >> 16 as libc::c_int)
                * LF_shp_Q14 as libc::c_short as libc::c_int
                + (((*psDD).Shape_Q10[*smpl_buf_idx as usize] & 0xffff as libc::c_int)
                    * LF_shp_Q14 as libc::c_short as libc::c_int >> 16 as libc::c_int)
                << 2 as libc::c_int;
            n_LF_Q10 = n_LF_Q10
                + ((*psDD).LF_AR_Q12 >> 16 as libc::c_int)
                    * (LF_shp_Q14 >> 16 as libc::c_int)
                + (((*psDD).LF_AR_Q12 & 0xffff as libc::c_int)
                    * (LF_shp_Q14 >> 16 as libc::c_int) >> 16 as libc::c_int);
            tmp1 = LTP_pred_Q14 - n_LTP_Q14;
            tmp1 = tmp1 >> 4 as libc::c_int;
            tmp1 = tmp1 + LPC_pred_Q10;
            tmp1 = tmp1 - n_AR_Q10;
            tmp1 = tmp1 - n_LF_Q10;
            r_Q10 = *x_Q10.offset(i as isize) - tmp1;
            r_Q10 = (r_Q10 ^ dither) - dither;
            r_Q10 = r_Q10 - offset_Q10;
            r_Q10 = if -((64 as libc::c_int) << 10 as libc::c_int)
                > (64 as libc::c_int) << 10 as libc::c_int
            {
                if r_Q10 > -((64 as libc::c_int) << 10 as libc::c_int) {
                    -((64 as libc::c_int) << 10 as libc::c_int)
                } else if r_Q10 < (64 as libc::c_int) << 10 as libc::c_int {
                    (64 as libc::c_int) << 10 as libc::c_int
                } else {
                    r_Q10
                }
            } else if r_Q10 > (64 as libc::c_int) << 10 as libc::c_int {
                (64 as libc::c_int) << 10 as libc::c_int
            } else if r_Q10 < -((64 as libc::c_int) << 10 as libc::c_int) {
                -((64 as libc::c_int) << 10 as libc::c_int)
            } else {
                r_Q10
            };
            if r_Q10 < -(1536 as libc::c_int) {
                q1_Q10 = (if 10 as libc::c_int == 1 as libc::c_int {
                    (r_Q10 >> 1 as libc::c_int) + (r_Q10 & 1 as libc::c_int)
                } else {
                    (r_Q10 >> 10 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                        >> 1 as libc::c_int
                }) << 10 as libc::c_int;
                r_Q10 = r_Q10 - q1_Q10;
                rd1_Q10 = -(q1_Q10 + offset_Q10) * Lambda_Q10
                    + r_Q10 as libc::c_short as libc::c_int
                        * r_Q10 as libc::c_short as libc::c_int >> 10 as libc::c_int;
                rd2_Q10 = rd1_Q10 + 1024 as libc::c_int;
                rd2_Q10 = rd2_Q10 - (Lambda_Q10 + (r_Q10 << 1 as libc::c_int));
                q2_Q10 = q1_Q10 + 1024 as libc::c_int;
            } else if r_Q10 > 512 as libc::c_int {
                q1_Q10 = (if 10 as libc::c_int == 1 as libc::c_int {
                    (r_Q10 >> 1 as libc::c_int) + (r_Q10 & 1 as libc::c_int)
                } else {
                    (r_Q10 >> 10 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                        >> 1 as libc::c_int
                }) << 10 as libc::c_int;
                r_Q10 = r_Q10 - q1_Q10;
                rd1_Q10 = (q1_Q10 + offset_Q10) * Lambda_Q10
                    + r_Q10 as libc::c_short as libc::c_int
                        * r_Q10 as libc::c_short as libc::c_int >> 10 as libc::c_int;
                rd2_Q10 = rd1_Q10 + 1024 as libc::c_int;
                rd2_Q10 = rd2_Q10 - (Lambda_Q10 - (r_Q10 << 1 as libc::c_int));
                q2_Q10 = q1_Q10 - 1024 as libc::c_int;
            } else {
                rr_Q20 = offset_Q10 as libc::c_short as libc::c_int
                    * Lambda_Q10 as libc::c_short as libc::c_int;
                rd2_Q10 = rr_Q20
                    + r_Q10 as libc::c_short as libc::c_int
                        * r_Q10 as libc::c_short as libc::c_int >> 10 as libc::c_int;
                rd1_Q10 = rd2_Q10 + 1024 as libc::c_int;
                rd1_Q10 = rd1_Q10
                    + (Lambda_Q10 + (r_Q10 << 1 as libc::c_int)
                        - (rr_Q20 >> 9 as libc::c_int));
                q1_Q10 = -(1024 as libc::c_int);
                q2_Q10 = 0 as libc::c_int;
            }
            if rd1_Q10 < rd2_Q10 {
                (*psSS.offset(0 as libc::c_int as isize))
                    .RD_Q10 = (*psDD).RD_Q10 + rd1_Q10;
                (*psSS.offset(1 as libc::c_int as isize))
                    .RD_Q10 = (*psDD).RD_Q10 + rd2_Q10;
                (*psSS.offset(0 as libc::c_int as isize)).Q_Q10 = q1_Q10;
                (*psSS.offset(1 as libc::c_int as isize)).Q_Q10 = q2_Q10;
            } else {
                (*psSS.offset(0 as libc::c_int as isize))
                    .RD_Q10 = (*psDD).RD_Q10 + rd2_Q10;
                (*psSS.offset(1 as libc::c_int as isize))
                    .RD_Q10 = (*psDD).RD_Q10 + rd1_Q10;
                (*psSS.offset(0 as libc::c_int as isize)).Q_Q10 = q2_Q10;
                (*psSS.offset(1 as libc::c_int as isize)).Q_Q10 = q1_Q10;
            }
            exc_Q10 = offset_Q10 + (*psSS.offset(0 as libc::c_int as isize)).Q_Q10;
            exc_Q10 = (exc_Q10 ^ dither) - dither;
            LPC_exc_Q10 = exc_Q10
                + (if 4 as libc::c_int == 1 as libc::c_int {
                    (LTP_pred_Q14 >> 1 as libc::c_int)
                        + (LTP_pred_Q14 & 1 as libc::c_int)
                } else {
                    (LTP_pred_Q14 >> 4 as libc::c_int - 1 as libc::c_int)
                        + 1 as libc::c_int >> 1 as libc::c_int
                });
            xq_Q10 = LPC_exc_Q10 + LPC_pred_Q10;
            sLF_AR_shp_Q10 = xq_Q10 - n_AR_Q10;
            (*psSS.offset(0 as libc::c_int as isize))
                .sLTP_shp_Q10 = sLF_AR_shp_Q10 - n_LF_Q10;
            (*psSS.offset(0 as libc::c_int as isize))
                .LF_AR_Q12 = sLF_AR_shp_Q10 << 2 as libc::c_int;
            (*psSS.offset(0 as libc::c_int as isize))
                .xq_Q14 = xq_Q10 << 4 as libc::c_int;
            (*psSS.offset(0 as libc::c_int as isize))
                .LPC_exc_Q16 = LPC_exc_Q10 << 6 as libc::c_int;
            exc_Q10 = offset_Q10 + (*psSS.offset(1 as libc::c_int as isize)).Q_Q10;
            exc_Q10 = (exc_Q10 ^ dither) - dither;
            LPC_exc_Q10 = exc_Q10
                + (if 4 as libc::c_int == 1 as libc::c_int {
                    (LTP_pred_Q14 >> 1 as libc::c_int)
                        + (LTP_pred_Q14 & 1 as libc::c_int)
                } else {
                    (LTP_pred_Q14 >> 4 as libc::c_int - 1 as libc::c_int)
                        + 1 as libc::c_int >> 1 as libc::c_int
                });
            xq_Q10 = LPC_exc_Q10 + LPC_pred_Q10;
            sLF_AR_shp_Q10 = xq_Q10 - n_AR_Q10;
            (*psSS.offset(1 as libc::c_int as isize))
                .sLTP_shp_Q10 = sLF_AR_shp_Q10 - n_LF_Q10;
            (*psSS.offset(1 as libc::c_int as isize))
                .LF_AR_Q12 = sLF_AR_shp_Q10 << 2 as libc::c_int;
            (*psSS.offset(1 as libc::c_int as isize))
                .xq_Q14 = xq_Q10 << 4 as libc::c_int;
            (*psSS.offset(1 as libc::c_int as isize))
                .LPC_exc_Q16 = LPC_exc_Q10 << 6 as libc::c_int;
            k += 1;
            k;
        }
        *smpl_buf_idx = *smpl_buf_idx - 1 as libc::c_int
            & 32 as libc::c_int - 1 as libc::c_int;
        last_smple_idx = *smpl_buf_idx + decisionDelay
            & 32 as libc::c_int - 1 as libc::c_int;
        RDmin_Q10 = psSampleState[0 as libc::c_int as usize][0 as libc::c_int as usize]
            .RD_Q10;
        Winner_ind = 0 as libc::c_int;
        k = 1 as libc::c_int;
        while k < nStatesDelayedDecision {
            if psSampleState[k as usize][0 as libc::c_int as usize].RD_Q10 < RDmin_Q10 {
                RDmin_Q10 = psSampleState[k as usize][0 as libc::c_int as usize].RD_Q10;
                Winner_ind = k;
            }
            k += 1;
            k;
        }
        Winner_rand_state = (*psDelDec.offset(Winner_ind as isize))
            .RandState[last_smple_idx as usize];
        k = 0 as libc::c_int;
        while k < nStatesDelayedDecision {
            if (*psDelDec.offset(k as isize)).RandState[last_smple_idx as usize]
                != Winner_rand_state
            {
                psSampleState[k as usize][0 as libc::c_int as usize]
                    .RD_Q10 = psSampleState[k as usize][0 as libc::c_int as usize].RD_Q10
                    + (0x7fffffff as libc::c_int >> 4 as libc::c_int);
                psSampleState[k as usize][1 as libc::c_int as usize]
                    .RD_Q10 = psSampleState[k as usize][1 as libc::c_int as usize].RD_Q10
                    + (0x7fffffff as libc::c_int >> 4 as libc::c_int);
            }
            k += 1;
            k;
        }
        RDmax_Q10 = psSampleState[0 as libc::c_int as usize][0 as libc::c_int as usize]
            .RD_Q10;
        RDmin_Q10 = psSampleState[0 as libc::c_int as usize][1 as libc::c_int as usize]
            .RD_Q10;
        RDmax_ind = 0 as libc::c_int;
        RDmin_ind = 0 as libc::c_int;
        k = 1 as libc::c_int;
        while k < nStatesDelayedDecision {
            if psSampleState[k as usize][0 as libc::c_int as usize].RD_Q10 > RDmax_Q10 {
                RDmax_Q10 = psSampleState[k as usize][0 as libc::c_int as usize].RD_Q10;
                RDmax_ind = k;
            }
            if psSampleState[k as usize][1 as libc::c_int as usize].RD_Q10 < RDmin_Q10 {
                RDmin_Q10 = psSampleState[k as usize][1 as libc::c_int as usize].RD_Q10;
                RDmin_ind = k;
            }
            k += 1;
            k;
        }
        if RDmin_Q10 < RDmax_Q10 {
            SKP_Silk_copy_del_dec_state(
                &mut *psDelDec.offset(RDmax_ind as isize),
                &mut *psDelDec.offset(RDmin_ind as isize),
                i,
            );
            memcpy(
                &mut *(*psSampleState.as_mut_ptr().offset(RDmax_ind as isize))
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize) as *mut NSQ_sample_struct
                    as *mut libc::c_void,
                &mut *(*psSampleState.as_mut_ptr().offset(RDmin_ind as isize))
                    .as_mut_ptr()
                    .offset(1 as libc::c_int as isize) as *mut NSQ_sample_struct
                    as *const libc::c_void,
                ::core::mem::size_of::<NSQ_sample_struct>() as libc::c_ulong,
            );
        }
        psDD = &mut *psDelDec.offset(Winner_ind as isize) as *mut NSQ_del_dec_struct;
        if subfr > 0 as libc::c_int || i >= decisionDelay {
            *q
                .offset(
                    (i - decisionDelay) as isize,
                ) = ((*psDD).Q_Q10[last_smple_idx as usize] >> 10 as libc::c_int)
                as libc::c_schar;
            *xq
                .offset(
                    (i - decisionDelay) as isize,
                ) = (if (if 10 as libc::c_int == 1 as libc::c_int {
                (((*psDD).Xq_Q10[last_smple_idx as usize] >> 16 as libc::c_int)
                    * (*psDD).Gain_Q16[last_smple_idx as usize] as libc::c_short
                        as libc::c_int
                    + (((*psDD).Xq_Q10[last_smple_idx as usize] & 0xffff as libc::c_int)
                        * (*psDD).Gain_Q16[last_smple_idx as usize] as libc::c_short
                            as libc::c_int >> 16 as libc::c_int)
                    + (*psDD).Xq_Q10[last_smple_idx as usize]
                        * (if 16 as libc::c_int == 1 as libc::c_int {
                            ((*psDD).Gain_Q16[last_smple_idx as usize]
                                >> 1 as libc::c_int)
                                + ((*psDD).Gain_Q16[last_smple_idx as usize]
                                    & 1 as libc::c_int)
                        } else {
                            ((*psDD).Gain_Q16[last_smple_idx as usize]
                                >> 16 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                                >> 1 as libc::c_int
                        }) >> 1 as libc::c_int)
                    + (((*psDD).Xq_Q10[last_smple_idx as usize] >> 16 as libc::c_int)
                        * (*psDD).Gain_Q16[last_smple_idx as usize] as libc::c_short
                            as libc::c_int
                        + (((*psDD).Xq_Q10[last_smple_idx as usize]
                            & 0xffff as libc::c_int)
                            * (*psDD).Gain_Q16[last_smple_idx as usize] as libc::c_short
                                as libc::c_int >> 16 as libc::c_int)
                        + (*psDD).Xq_Q10[last_smple_idx as usize]
                            * (if 16 as libc::c_int == 1 as libc::c_int {
                                ((*psDD).Gain_Q16[last_smple_idx as usize]
                                    >> 1 as libc::c_int)
                                    + ((*psDD).Gain_Q16[last_smple_idx as usize]
                                        & 1 as libc::c_int)
                            } else {
                                ((*psDD).Gain_Q16[last_smple_idx as usize]
                                    >> 16 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                                    >> 1 as libc::c_int
                            }) & 1 as libc::c_int)
            } else {
                (((*psDD).Xq_Q10[last_smple_idx as usize] >> 16 as libc::c_int)
                    * (*psDD).Gain_Q16[last_smple_idx as usize] as libc::c_short
                        as libc::c_int
                    + (((*psDD).Xq_Q10[last_smple_idx as usize] & 0xffff as libc::c_int)
                        * (*psDD).Gain_Q16[last_smple_idx as usize] as libc::c_short
                            as libc::c_int >> 16 as libc::c_int)
                    + (*psDD).Xq_Q10[last_smple_idx as usize]
                        * (if 16 as libc::c_int == 1 as libc::c_int {
                            ((*psDD).Gain_Q16[last_smple_idx as usize]
                                >> 1 as libc::c_int)
                                + ((*psDD).Gain_Q16[last_smple_idx as usize]
                                    & 1 as libc::c_int)
                        } else {
                            ((*psDD).Gain_Q16[last_smple_idx as usize]
                                >> 16 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                                >> 1 as libc::c_int
                        }) >> 10 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                    >> 1 as libc::c_int
            }) > 0x7fff as libc::c_int
            {
                0x7fff as libc::c_int
            } else if (if 10 as libc::c_int == 1 as libc::c_int {
                (((*psDD).Xq_Q10[last_smple_idx as usize] >> 16 as libc::c_int)
                    * (*psDD).Gain_Q16[last_smple_idx as usize] as libc::c_short
                        as libc::c_int
                    + (((*psDD).Xq_Q10[last_smple_idx as usize] & 0xffff as libc::c_int)
                        * (*psDD).Gain_Q16[last_smple_idx as usize] as libc::c_short
                            as libc::c_int >> 16 as libc::c_int)
                    + (*psDD).Xq_Q10[last_smple_idx as usize]
                        * (if 16 as libc::c_int == 1 as libc::c_int {
                            ((*psDD).Gain_Q16[last_smple_idx as usize]
                                >> 1 as libc::c_int)
                                + ((*psDD).Gain_Q16[last_smple_idx as usize]
                                    & 1 as libc::c_int)
                        } else {
                            ((*psDD).Gain_Q16[last_smple_idx as usize]
                                >> 16 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                                >> 1 as libc::c_int
                        }) >> 1 as libc::c_int)
                    + (((*psDD).Xq_Q10[last_smple_idx as usize] >> 16 as libc::c_int)
                        * (*psDD).Gain_Q16[last_smple_idx as usize] as libc::c_short
                            as libc::c_int
                        + (((*psDD).Xq_Q10[last_smple_idx as usize]
                            & 0xffff as libc::c_int)
                            * (*psDD).Gain_Q16[last_smple_idx as usize] as libc::c_short
                                as libc::c_int >> 16 as libc::c_int)
                        + (*psDD).Xq_Q10[last_smple_idx as usize]
                            * (if 16 as libc::c_int == 1 as libc::c_int {
                                ((*psDD).Gain_Q16[last_smple_idx as usize]
                                    >> 1 as libc::c_int)
                                    + ((*psDD).Gain_Q16[last_smple_idx as usize]
                                        & 1 as libc::c_int)
                            } else {
                                ((*psDD).Gain_Q16[last_smple_idx as usize]
                                    >> 16 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                                    >> 1 as libc::c_int
                            }) & 1 as libc::c_int)
            } else {
                (((*psDD).Xq_Q10[last_smple_idx as usize] >> 16 as libc::c_int)
                    * (*psDD).Gain_Q16[last_smple_idx as usize] as libc::c_short
                        as libc::c_int
                    + (((*psDD).Xq_Q10[last_smple_idx as usize] & 0xffff as libc::c_int)
                        * (*psDD).Gain_Q16[last_smple_idx as usize] as libc::c_short
                            as libc::c_int >> 16 as libc::c_int)
                    + (*psDD).Xq_Q10[last_smple_idx as usize]
                        * (if 16 as libc::c_int == 1 as libc::c_int {
                            ((*psDD).Gain_Q16[last_smple_idx as usize]
                                >> 1 as libc::c_int)
                                + ((*psDD).Gain_Q16[last_smple_idx as usize]
                                    & 1 as libc::c_int)
                        } else {
                            ((*psDD).Gain_Q16[last_smple_idx as usize]
                                >> 16 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                                >> 1 as libc::c_int
                        }) >> 10 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                    >> 1 as libc::c_int
            }) < 0x8000 as libc::c_int as libc::c_short as libc::c_int
            {
                0x8000 as libc::c_int as libc::c_short as libc::c_int
            } else if 10 as libc::c_int == 1 as libc::c_int {
                (((*psDD).Xq_Q10[last_smple_idx as usize] >> 16 as libc::c_int)
                    * (*psDD).Gain_Q16[last_smple_idx as usize] as libc::c_short
                        as libc::c_int
                    + (((*psDD).Xq_Q10[last_smple_idx as usize] & 0xffff as libc::c_int)
                        * (*psDD).Gain_Q16[last_smple_idx as usize] as libc::c_short
                            as libc::c_int >> 16 as libc::c_int)
                    + (*psDD).Xq_Q10[last_smple_idx as usize]
                        * (if 16 as libc::c_int == 1 as libc::c_int {
                            ((*psDD).Gain_Q16[last_smple_idx as usize]
                                >> 1 as libc::c_int)
                                + ((*psDD).Gain_Q16[last_smple_idx as usize]
                                    & 1 as libc::c_int)
                        } else {
                            ((*psDD).Gain_Q16[last_smple_idx as usize]
                                >> 16 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                                >> 1 as libc::c_int
                        }) >> 1 as libc::c_int)
                    + (((*psDD).Xq_Q10[last_smple_idx as usize] >> 16 as libc::c_int)
                        * (*psDD).Gain_Q16[last_smple_idx as usize] as libc::c_short
                            as libc::c_int
                        + (((*psDD).Xq_Q10[last_smple_idx as usize]
                            & 0xffff as libc::c_int)
                            * (*psDD).Gain_Q16[last_smple_idx as usize] as libc::c_short
                                as libc::c_int >> 16 as libc::c_int)
                        + (*psDD).Xq_Q10[last_smple_idx as usize]
                            * (if 16 as libc::c_int == 1 as libc::c_int {
                                ((*psDD).Gain_Q16[last_smple_idx as usize]
                                    >> 1 as libc::c_int)
                                    + ((*psDD).Gain_Q16[last_smple_idx as usize]
                                        & 1 as libc::c_int)
                            } else {
                                ((*psDD).Gain_Q16[last_smple_idx as usize]
                                    >> 16 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                                    >> 1 as libc::c_int
                            }) & 1 as libc::c_int)
            } else {
                (((*psDD).Xq_Q10[last_smple_idx as usize] >> 16 as libc::c_int)
                    * (*psDD).Gain_Q16[last_smple_idx as usize] as libc::c_short
                        as libc::c_int
                    + (((*psDD).Xq_Q10[last_smple_idx as usize] & 0xffff as libc::c_int)
                        * (*psDD).Gain_Q16[last_smple_idx as usize] as libc::c_short
                            as libc::c_int >> 16 as libc::c_int)
                    + (*psDD).Xq_Q10[last_smple_idx as usize]
                        * (if 16 as libc::c_int == 1 as libc::c_int {
                            ((*psDD).Gain_Q16[last_smple_idx as usize]
                                >> 1 as libc::c_int)
                                + ((*psDD).Gain_Q16[last_smple_idx as usize]
                                    & 1 as libc::c_int)
                        } else {
                            ((*psDD).Gain_Q16[last_smple_idx as usize]
                                >> 16 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                                >> 1 as libc::c_int
                        }) >> 10 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                    >> 1 as libc::c_int
            }) as libc::c_short;
            (*NSQ)
                .sLTP_shp_Q10[((*NSQ).sLTP_shp_buf_idx - decisionDelay)
                as usize] = (*psDD).Shape_Q10[last_smple_idx as usize];
            *sLTP_Q16
                .offset(
                    ((*NSQ).sLTP_buf_idx - decisionDelay) as isize,
                ) = (*psDD).Pred_Q16[last_smple_idx as usize];
        }
        (*NSQ).sLTP_shp_buf_idx += 1;
        (*NSQ).sLTP_shp_buf_idx;
        (*NSQ).sLTP_buf_idx += 1;
        (*NSQ).sLTP_buf_idx;
        k = 0 as libc::c_int;
        while k < nStatesDelayedDecision {
            psDD = &mut *psDelDec.offset(k as isize) as *mut NSQ_del_dec_struct;
            psSS = &mut *(*psSampleState.as_mut_ptr().offset(k as isize))
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize) as *mut NSQ_sample_struct;
            (*psDD).LF_AR_Q12 = (*psSS).LF_AR_Q12;
            (*psDD).sLPC_Q14[(32 as libc::c_int + i) as usize] = (*psSS).xq_Q14;
            (*psDD).Xq_Q10[*smpl_buf_idx as usize] = (*psSS).xq_Q14 >> 4 as libc::c_int;
            (*psDD).Q_Q10[*smpl_buf_idx as usize] = (*psSS).Q_Q10;
            (*psDD).Pred_Q16[*smpl_buf_idx as usize] = (*psSS).LPC_exc_Q16;
            (*psDD).Shape_Q10[*smpl_buf_idx as usize] = (*psSS).sLTP_shp_Q10;
            (*psDD).Seed = (*psDD).Seed + ((*psSS).Q_Q10 >> 10 as libc::c_int);
            (*psDD).RandState[*smpl_buf_idx as usize] = (*psDD).Seed;
            (*psDD).RD_Q10 = (*psSS).RD_Q10;
            (*psDD).Gain_Q16[*smpl_buf_idx as usize] = Gain_Q16;
            k += 1;
            k;
        }
        i += 1;
        i;
    }
    k = 0 as libc::c_int;
    while k < nStatesDelayedDecision {
        psDD = &mut *psDelDec.offset(k as isize) as *mut NSQ_del_dec_struct;
        memcpy(
            ((*psDD).sLPC_Q14).as_mut_ptr() as *mut libc::c_void,
            &mut *((*psDD).sLPC_Q14).as_mut_ptr().offset(length as isize)
                as *mut libc::c_int as *const libc::c_void,
            (32 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
        );
        k += 1;
        k;
    }
}
#[inline]
unsafe extern "C" fn SKP_Silk_nsq_del_dec_scale_states(
    mut NSQ: *mut SKP_Silk_nsq_state,
    mut psDelDec: *mut NSQ_del_dec_struct,
    mut x: *const libc::c_short,
    mut x_sc_Q10: *mut libc::c_int,
    mut subfr_length: libc::c_int,
    mut sLTP: *const libc::c_short,
    mut sLTP_Q16: *mut libc::c_int,
    mut subfr: libc::c_int,
    mut nStatesDelayedDecision: libc::c_int,
    mut smpl_buf_idx: libc::c_int,
    LTP_scale_Q14: libc::c_int,
    mut Gains_Q16: *const libc::c_int,
    mut pitchL: *const libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut lag: libc::c_int = 0;
    let mut inv_gain_Q16: libc::c_int = 0;
    let mut gain_adj_Q16: libc::c_int = 0;
    let mut inv_gain_Q32: libc::c_int = 0;
    let mut psDD: *mut NSQ_del_dec_struct = 0 as *mut NSQ_del_dec_struct;
    inv_gain_Q16 = SKP_INVERSE32_varQ(
        if *Gains_Q16.offset(subfr as isize) > 1 as libc::c_int {
            *Gains_Q16.offset(subfr as isize)
        } else {
            1 as libc::c_int
        },
        32 as libc::c_int,
    );
    inv_gain_Q16 = if inv_gain_Q16 < 0x7fff as libc::c_int {
        inv_gain_Q16
    } else {
        0x7fff as libc::c_int
    };
    lag = *pitchL.offset(subfr as isize);
    if (*NSQ).rewhite_flag != 0 {
        inv_gain_Q32 = inv_gain_Q16 << 16 as libc::c_int;
        if subfr == 0 as libc::c_int {
            inv_gain_Q32 = (inv_gain_Q32 >> 16 as libc::c_int)
                * LTP_scale_Q14 as libc::c_short as libc::c_int
                + ((inv_gain_Q32 & 0xffff as libc::c_int)
                    * LTP_scale_Q14 as libc::c_short as libc::c_int >> 16 as libc::c_int)
                << 2 as libc::c_int;
        }
        i = (*NSQ).sLTP_buf_idx - lag - 5 as libc::c_int / 2 as libc::c_int;
        while i < (*NSQ).sLTP_buf_idx {
            *sLTP_Q16
                .offset(
                    i as isize,
                ) = (inv_gain_Q32 >> 16 as libc::c_int)
                * *sLTP.offset(i as isize) as libc::c_int
                + ((inv_gain_Q32 & 0xffff as libc::c_int)
                    * *sLTP.offset(i as isize) as libc::c_int >> 16 as libc::c_int);
            i += 1;
            i;
        }
    }
    if inv_gain_Q16 != (*NSQ).prev_inv_gain_Q16 {
        gain_adj_Q16 = SKP_DIV32_varQ(
            inv_gain_Q16,
            (*NSQ).prev_inv_gain_Q16,
            16 as libc::c_int,
        );
        i = (*NSQ).sLTP_shp_buf_idx - subfr_length * 4 as libc::c_int;
        while i < (*NSQ).sLTP_shp_buf_idx {
            (*NSQ)
                .sLTP_shp_Q10[i
                as usize] = (gain_adj_Q16 >> 16 as libc::c_int)
                * (*NSQ).sLTP_shp_Q10[i as usize] as libc::c_short as libc::c_int
                + ((gain_adj_Q16 & 0xffff as libc::c_int)
                    * (*NSQ).sLTP_shp_Q10[i as usize] as libc::c_short as libc::c_int
                    >> 16 as libc::c_int)
                + gain_adj_Q16
                    * (if 16 as libc::c_int == 1 as libc::c_int {
                        ((*NSQ).sLTP_shp_Q10[i as usize] >> 1 as libc::c_int)
                            + ((*NSQ).sLTP_shp_Q10[i as usize] & 1 as libc::c_int)
                    } else {
                        ((*NSQ).sLTP_shp_Q10[i as usize]
                            >> 16 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                            >> 1 as libc::c_int
                    });
            i += 1;
            i;
        }
        if (*NSQ).rewhite_flag == 0 as libc::c_int {
            i = (*NSQ).sLTP_buf_idx - lag - 5 as libc::c_int / 2 as libc::c_int;
            while i < (*NSQ).sLTP_buf_idx {
                *sLTP_Q16
                    .offset(
                        i as isize,
                    ) = (gain_adj_Q16 >> 16 as libc::c_int)
                    * *sLTP_Q16.offset(i as isize) as libc::c_short as libc::c_int
                    + ((gain_adj_Q16 & 0xffff as libc::c_int)
                        * *sLTP_Q16.offset(i as isize) as libc::c_short as libc::c_int
                        >> 16 as libc::c_int)
                    + gain_adj_Q16
                        * (if 16 as libc::c_int == 1 as libc::c_int {
                            (*sLTP_Q16.offset(i as isize) >> 1 as libc::c_int)
                                + (*sLTP_Q16.offset(i as isize) & 1 as libc::c_int)
                        } else {
                            (*sLTP_Q16.offset(i as isize)
                                >> 16 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                                >> 1 as libc::c_int
                        });
                i += 1;
                i;
            }
        }
        k = 0 as libc::c_int;
        while k < nStatesDelayedDecision {
            psDD = &mut *psDelDec.offset(k as isize) as *mut NSQ_del_dec_struct;
            (*psDD)
                .LF_AR_Q12 = (gain_adj_Q16 >> 16 as libc::c_int)
                * (*psDD).LF_AR_Q12 as libc::c_short as libc::c_int
                + ((gain_adj_Q16 & 0xffff as libc::c_int)
                    * (*psDD).LF_AR_Q12 as libc::c_short as libc::c_int
                    >> 16 as libc::c_int)
                + gain_adj_Q16
                    * (if 16 as libc::c_int == 1 as libc::c_int {
                        ((*psDD).LF_AR_Q12 >> 1 as libc::c_int)
                            + ((*psDD).LF_AR_Q12 & 1 as libc::c_int)
                    } else {
                        ((*psDD).LF_AR_Q12 >> 16 as libc::c_int - 1 as libc::c_int)
                            + 1 as libc::c_int >> 1 as libc::c_int
                    });
            i = 0 as libc::c_int;
            while i < 32 as libc::c_int {
                (*psDD)
                    .sLPC_Q14[i
                    as usize] = (gain_adj_Q16 >> 16 as libc::c_int)
                    * (*psDD).sLPC_Q14[i as usize] as libc::c_short as libc::c_int
                    + ((gain_adj_Q16 & 0xffff as libc::c_int)
                        * (*psDD).sLPC_Q14[i as usize] as libc::c_short as libc::c_int
                        >> 16 as libc::c_int)
                    + gain_adj_Q16
                        * (if 16 as libc::c_int == 1 as libc::c_int {
                            ((*psDD).sLPC_Q14[i as usize] >> 1 as libc::c_int)
                                + ((*psDD).sLPC_Q14[i as usize] & 1 as libc::c_int)
                        } else {
                            ((*psDD).sLPC_Q14[i as usize]
                                >> 16 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                                >> 1 as libc::c_int
                        });
                i += 1;
                i;
            }
            i = 0 as libc::c_int;
            while i < 16 as libc::c_int {
                (*psDD)
                    .sAR2_Q14[i
                    as usize] = (gain_adj_Q16 >> 16 as libc::c_int)
                    * (*psDD).sAR2_Q14[i as usize] as libc::c_short as libc::c_int
                    + ((gain_adj_Q16 & 0xffff as libc::c_int)
                        * (*psDD).sAR2_Q14[i as usize] as libc::c_short as libc::c_int
                        >> 16 as libc::c_int)
                    + gain_adj_Q16
                        * (if 16 as libc::c_int == 1 as libc::c_int {
                            ((*psDD).sAR2_Q14[i as usize] >> 1 as libc::c_int)
                                + ((*psDD).sAR2_Q14[i as usize] & 1 as libc::c_int)
                        } else {
                            ((*psDD).sAR2_Q14[i as usize]
                                >> 16 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                                >> 1 as libc::c_int
                        });
                i += 1;
                i;
            }
            i = 0 as libc::c_int;
            while i < 32 as libc::c_int {
                (*psDD)
                    .Pred_Q16[i
                    as usize] = (gain_adj_Q16 >> 16 as libc::c_int)
                    * (*psDD).Pred_Q16[i as usize] as libc::c_short as libc::c_int
                    + ((gain_adj_Q16 & 0xffff as libc::c_int)
                        * (*psDD).Pred_Q16[i as usize] as libc::c_short as libc::c_int
                        >> 16 as libc::c_int)
                    + gain_adj_Q16
                        * (if 16 as libc::c_int == 1 as libc::c_int {
                            ((*psDD).Pred_Q16[i as usize] >> 1 as libc::c_int)
                                + ((*psDD).Pred_Q16[i as usize] & 1 as libc::c_int)
                        } else {
                            ((*psDD).Pred_Q16[i as usize]
                                >> 16 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                                >> 1 as libc::c_int
                        });
                (*psDD)
                    .Shape_Q10[i
                    as usize] = (gain_adj_Q16 >> 16 as libc::c_int)
                    * (*psDD).Shape_Q10[i as usize] as libc::c_short as libc::c_int
                    + ((gain_adj_Q16 & 0xffff as libc::c_int)
                        * (*psDD).Shape_Q10[i as usize] as libc::c_short as libc::c_int
                        >> 16 as libc::c_int)
                    + gain_adj_Q16
                        * (if 16 as libc::c_int == 1 as libc::c_int {
                            ((*psDD).Shape_Q10[i as usize] >> 1 as libc::c_int)
                                + ((*psDD).Shape_Q10[i as usize] & 1 as libc::c_int)
                        } else {
                            ((*psDD).Shape_Q10[i as usize]
                                >> 16 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                                >> 1 as libc::c_int
                        });
                i += 1;
                i;
            }
            k += 1;
            k;
        }
    }
    i = 0 as libc::c_int;
    while i < subfr_length {
        *x_sc_Q10
            .offset(
                i as isize,
            ) = *x.offset(i as isize) as libc::c_int
            * inv_gain_Q16 as libc::c_short as libc::c_int >> 6 as libc::c_int;
        i += 1;
        i;
    }
    (*NSQ).prev_inv_gain_Q16 = inv_gain_Q16;
}
#[inline]
unsafe extern "C" fn SKP_Silk_copy_del_dec_state(
    mut DD_dst: *mut NSQ_del_dec_struct,
    mut DD_src: *mut NSQ_del_dec_struct,
    mut LPC_state_idx: libc::c_int,
) {
    memcpy(
        ((*DD_dst).RandState).as_mut_ptr() as *mut libc::c_void,
        ((*DD_src).RandState).as_mut_ptr() as *const libc::c_void,
        ::core::mem::size_of::<[libc::c_int; 32]>() as libc::c_ulong,
    );
    memcpy(
        ((*DD_dst).Q_Q10).as_mut_ptr() as *mut libc::c_void,
        ((*DD_src).Q_Q10).as_mut_ptr() as *const libc::c_void,
        ::core::mem::size_of::<[libc::c_int; 32]>() as libc::c_ulong,
    );
    memcpy(
        ((*DD_dst).Pred_Q16).as_mut_ptr() as *mut libc::c_void,
        ((*DD_src).Pred_Q16).as_mut_ptr() as *const libc::c_void,
        ::core::mem::size_of::<[libc::c_int; 32]>() as libc::c_ulong,
    );
    memcpy(
        ((*DD_dst).Shape_Q10).as_mut_ptr() as *mut libc::c_void,
        ((*DD_src).Shape_Q10).as_mut_ptr() as *const libc::c_void,
        ::core::mem::size_of::<[libc::c_int; 32]>() as libc::c_ulong,
    );
    memcpy(
        ((*DD_dst).Xq_Q10).as_mut_ptr() as *mut libc::c_void,
        ((*DD_src).Xq_Q10).as_mut_ptr() as *const libc::c_void,
        ::core::mem::size_of::<[libc::c_int; 32]>() as libc::c_ulong,
    );
    memcpy(
        ((*DD_dst).sAR2_Q14).as_mut_ptr() as *mut libc::c_void,
        ((*DD_src).sAR2_Q14).as_mut_ptr() as *const libc::c_void,
        ::core::mem::size_of::<[libc::c_int; 16]>() as libc::c_ulong,
    );
    memcpy(
        &mut *((*DD_dst).sLPC_Q14).as_mut_ptr().offset(LPC_state_idx as isize)
            as *mut libc::c_int as *mut libc::c_void,
        &mut *((*DD_src).sLPC_Q14).as_mut_ptr().offset(LPC_state_idx as isize)
            as *mut libc::c_int as *const libc::c_void,
        (32 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    (*DD_dst).LF_AR_Q12 = (*DD_src).LF_AR_Q12;
    (*DD_dst).Seed = (*DD_src).Seed;
    (*DD_dst).SeedInit = (*DD_src).SeedInit;
    (*DD_dst).RD_Q10 = (*DD_src).RD_Q10;
}
