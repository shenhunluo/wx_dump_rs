#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    static SKP_Silk_FrameTermination_CDF: [libc::c_ushort; 5];
    fn SKP_Silk_range_encoder(
        psRC: *mut SKP_Silk_range_coder_state,
        data: libc::c_int,
        prob: *const libc::c_ushort,
    );
    fn SKP_Silk_range_enc_init(psRC: *mut SKP_Silk_range_coder_state);
    fn SKP_Silk_range_coder_get_length(
        psRC: *const SKP_Silk_range_coder_state,
        nBytes: *mut libc::c_int,
    ) -> libc::c_int;
    fn SKP_Silk_range_enc_wrap_up(psRC: *mut SKP_Silk_range_coder_state);
    fn SKP_Silk_gains_dequant(
        gain_Q16: *mut libc::c_int,
        ind: *const libc::c_int,
        prev_ind: *mut libc::c_int,
        conditional: libc::c_int,
    );
    fn SKP_Silk_NSQ(
        psEncC: *mut SKP_Silk_encoder_state,
        psEncCtrlC: *mut SKP_Silk_encoder_control,
        NSQ: *mut SKP_Silk_nsq_state,
        x: *const libc::c_short,
        q: *mut libc::c_schar,
        LSFInterpFactor_Q2: libc::c_int,
        PredCoef_Q12: *const libc::c_short,
        LTPCoef_Q14: *const libc::c_short,
        AR2_Q13: *const libc::c_short,
        HarmShapeGain_Q14: *const libc::c_int,
        Tilt_Q14: *const libc::c_int,
        LF_shp_Q14: *const libc::c_int,
        Gains_Q16: *const libc::c_int,
        Lambda_Q10: libc::c_int,
        LTP_scale_Q14: libc::c_int,
    );
    fn SKP_Silk_NSQ_del_dec(
        psEncC: *mut SKP_Silk_encoder_state,
        psEncCtrlC: *mut SKP_Silk_encoder_control,
        NSQ: *mut SKP_Silk_nsq_state,
        x: *const libc::c_short,
        q: *mut libc::c_schar,
        LSFInterpFactor_Q2: libc::c_int,
        PredCoef_Q12: *const libc::c_short,
        LTPCoef_Q14: *const libc::c_short,
        AR2_Q13: *const libc::c_short,
        HarmShapeGain_Q14: *const libc::c_int,
        Tilt_Q14: *const libc::c_int,
        LF_shp_Q14: *const libc::c_int,
        Gains_Q16: *const libc::c_int,
        Lambda_Q10: libc::c_int,
        LTP_scale_Q14: libc::c_int,
    );
    fn SKP_Silk_VAD_GetSA_Q8(
        psSilk_VAD: *mut SKP_Silk_VAD_state,
        pSA_Q8: *mut libc::c_int,
        pSNR_dB_Q7: *mut libc::c_int,
        pQuality_Q15: *mut libc::c_int,
        pTilt_Q15: *mut libc::c_int,
        pIn: *const libc::c_short,
        framelength: libc::c_int,
    ) -> libc::c_int;
    fn SKP_Silk_LP_variable_cutoff(
        psLP: *mut SKP_Silk_LP_state,
        out: *mut libc::c_short,
        in_0: *const libc::c_short,
        frame_length: libc::c_int,
    );
    fn SKP_Silk_encode_parameters(
        psEncC: *mut SKP_Silk_encoder_state,
        psEncCtrlC: *mut SKP_Silk_encoder_control,
        psRC: *mut SKP_Silk_range_coder_state,
        q: *const libc::c_schar,
    );
    fn SKP_Silk_LBRR_ctrl_FIX(
        psEnc: *mut SKP_Silk_encoder_state_FIX,
        psEncCtrlC: *mut SKP_Silk_encoder_control,
    );
    fn SKP_Silk_process_gains_FIX(
        psEnc: *mut SKP_Silk_encoder_state_FIX,
        psEncCtrl: *mut SKP_Silk_encoder_control_FIX,
    );
    fn SKP_Silk_find_pred_coefs_FIX(
        psEnc: *mut SKP_Silk_encoder_state_FIX,
        psEncCtrl: *mut SKP_Silk_encoder_control_FIX,
        res_pitch: *const libc::c_short,
    );
    fn SKP_Silk_prefilter_FIX(
        psEnc: *mut SKP_Silk_encoder_state_FIX,
        psEncCtrl: *const SKP_Silk_encoder_control_FIX,
        xw: *mut libc::c_short,
        x: *const libc::c_short,
    );
    fn SKP_Silk_noise_shape_analysis_FIX(
        psEnc: *mut SKP_Silk_encoder_state_FIX,
        psEncCtrl: *mut SKP_Silk_encoder_control_FIX,
        pitch_res: *const libc::c_short,
        x: *const libc::c_short,
    );
    fn SKP_Silk_find_pitch_lags_FIX(
        psEnc: *mut SKP_Silk_encoder_state_FIX,
        psEncCtrl: *mut SKP_Silk_encoder_control_FIX,
        res: *mut libc::c_short,
        x: *const libc::c_short,
    );
    fn SKP_Silk_HP_variable_cutoff_FIX(
        psEnc: *mut SKP_Silk_encoder_state_FIX,
        psEncCtrl: *mut SKP_Silk_encoder_control_FIX,
        out: *mut libc::c_short,
        in_0: *const libc::c_short,
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
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_encode_frame_FIX(
    mut psEnc: *mut SKP_Silk_encoder_state_FIX,
    mut pCode: *mut libc::c_uchar,
    mut pnBytesOut: *mut libc::c_short,
    mut pIn: *const libc::c_short,
) -> libc::c_int {
    let mut sEncCtrl: SKP_Silk_encoder_control_FIX = SKP_Silk_encoder_control_FIX {
        sCmn: SKP_Silk_encoder_control {
            lagIndex: 0,
            contourIndex: 0,
            PERIndex: 0,
            LTPIndex: [0; 4],
            NLSFIndices: [0; 10],
            NLSFInterpCoef_Q2: 0,
            GainsIndices: [0; 4],
            Seed: 0,
            LTP_scaleIndex: 0,
            RateLevelIndex: 0,
            QuantOffsetType: 0,
            sigtype: 0,
            pitchL: [0; 4],
            LBRR_usage: 0,
        },
        Gains_Q16: [0; 4],
        PredCoef_Q12: [[0; 16]; 2],
        LTPCoef_Q14: [0; 20],
        LTP_scale_Q14: 0,
        AR1_Q13: [0; 64],
        AR2_Q13: [0; 64],
        LF_shp_Q14: [0; 4],
        GainsPre_Q14: [0; 4],
        HarmBoost_Q14: [0; 4],
        Tilt_Q14: [0; 4],
        HarmShapeGain_Q14: [0; 4],
        Lambda_Q10: 0,
        input_quality_Q14: 0,
        coding_quality_Q14: 0,
        pitch_freq_low_Hz: 0,
        current_SNR_dB_Q7: 0,
        sparseness_Q8: 0,
        predGain_Q16: 0,
        LTPredCodGain_Q7: 0,
        input_quality_bands_Q15: [0; 4],
        input_tilt_Q15: 0,
        ResNrg: [0; 4],
        ResNrgQ: [0; 4],
    };
    let mut nBytes: libc::c_int = 0;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut x_frame: *mut libc::c_short = 0 as *mut libc::c_short;
    let mut res_pitch_frame: *mut libc::c_short = 0 as *mut libc::c_short;
    let mut xfw: [libc::c_short; 480] = [0; 480];
    let mut pIn_HP: [libc::c_short; 480] = [0; 480];
    let mut res_pitch: [libc::c_short; 1008] = [0; 1008];
    let mut LBRR_idx: libc::c_int = 0;
    let mut frame_terminator: libc::c_int = 0;
    let mut SNR_dB_Q7: libc::c_int = 0;
    let mut FrameTermination_CDF: *const libc::c_ushort = 0 as *const libc::c_ushort;
    let mut LBRRpayload: [libc::c_uchar; 1024] = [0; 1024];
    let mut nBytesLBRR: libc::c_short = 0;
    let fresh0 = (*psEnc).sCmn.frameCounter;
    (*psEnc).sCmn.frameCounter = (*psEnc).sCmn.frameCounter + 1;
    sEncCtrl.sCmn.Seed = fresh0 & 3 as libc::c_int;
    x_frame = ((*psEnc).x_buf).as_mut_ptr().offset((*psEnc).sCmn.frame_length as isize);
    res_pitch_frame = res_pitch.as_mut_ptr().offset((*psEnc).sCmn.frame_length as isize);
    ret = SKP_Silk_VAD_GetSA_Q8(
        &mut (*psEnc).sCmn.sVAD,
        &mut (*psEnc).speech_activity_Q8,
        &mut SNR_dB_Q7,
        (sEncCtrl.input_quality_bands_Q15).as_mut_ptr(),
        &mut sEncCtrl.input_tilt_Q15,
        pIn,
        (*psEnc).sCmn.frame_length,
    );
    SKP_Silk_HP_variable_cutoff_FIX(psEnc, &mut sEncCtrl, pIn_HP.as_mut_ptr(), pIn);
    SKP_Silk_LP_variable_cutoff(
        &mut (*psEnc).sCmn.sLP,
        x_frame.offset((5 as libc::c_int * (*psEnc).sCmn.fs_kHz) as isize),
        pIn_HP.as_mut_ptr(),
        (*psEnc).sCmn.frame_length,
    );
    SKP_Silk_find_pitch_lags_FIX(
        psEnc,
        &mut sEncCtrl,
        res_pitch.as_mut_ptr(),
        x_frame as *const libc::c_short,
    );
    SKP_Silk_noise_shape_analysis_FIX(psEnc, &mut sEncCtrl, res_pitch_frame, x_frame);
    SKP_Silk_prefilter_FIX(
        psEnc,
        &mut sEncCtrl,
        xfw.as_mut_ptr(),
        x_frame as *const libc::c_short,
    );
    SKP_Silk_find_pred_coefs_FIX(
        psEnc,
        &mut sEncCtrl,
        res_pitch.as_mut_ptr() as *const libc::c_short,
    );
    SKP_Silk_process_gains_FIX(psEnc, &mut sEncCtrl);
    nBytesLBRR = 1024 as libc::c_int as libc::c_short;
    SKP_Silk_LBRR_encode_FIX(
        psEnc,
        &mut sEncCtrl,
        LBRRpayload.as_mut_ptr(),
        &mut nBytesLBRR,
        xfw.as_mut_ptr(),
    );
    if (*psEnc).sCmn.nStatesDelayedDecision > 1 as libc::c_int
        || (*psEnc).sCmn.warping_Q16 > 0 as libc::c_int
    {
        SKP_Silk_NSQ_del_dec(
            &mut (*psEnc).sCmn,
            &mut sEncCtrl.sCmn,
            &mut (*psEnc).sCmn.sNSQ,
            xfw.as_mut_ptr() as *const libc::c_short,
            ((*psEnc).sCmn.q).as_mut_ptr(),
            sEncCtrl.sCmn.NLSFInterpCoef_Q2,
            (sEncCtrl.PredCoef_Q12[0 as libc::c_int as usize]).as_mut_ptr()
                as *const libc::c_short,
            (sEncCtrl.LTPCoef_Q14).as_mut_ptr() as *const libc::c_short,
            (sEncCtrl.AR2_Q13).as_mut_ptr() as *const libc::c_short,
            (sEncCtrl.HarmShapeGain_Q14).as_mut_ptr() as *const libc::c_int,
            (sEncCtrl.Tilt_Q14).as_mut_ptr() as *const libc::c_int,
            (sEncCtrl.LF_shp_Q14).as_mut_ptr() as *const libc::c_int,
            (sEncCtrl.Gains_Q16).as_mut_ptr() as *const libc::c_int,
            sEncCtrl.Lambda_Q10,
            sEncCtrl.LTP_scale_Q14,
        );
    } else {
        SKP_Silk_NSQ(
            &mut (*psEnc).sCmn,
            &mut sEncCtrl.sCmn,
            &mut (*psEnc).sCmn.sNSQ,
            xfw.as_mut_ptr() as *const libc::c_short,
            ((*psEnc).sCmn.q).as_mut_ptr(),
            sEncCtrl.sCmn.NLSFInterpCoef_Q2,
            (sEncCtrl.PredCoef_Q12[0 as libc::c_int as usize]).as_mut_ptr()
                as *const libc::c_short,
            (sEncCtrl.LTPCoef_Q14).as_mut_ptr() as *const libc::c_short,
            (sEncCtrl.AR2_Q13).as_mut_ptr() as *const libc::c_short,
            (sEncCtrl.HarmShapeGain_Q14).as_mut_ptr() as *const libc::c_int,
            (sEncCtrl.Tilt_Q14).as_mut_ptr() as *const libc::c_int,
            (sEncCtrl.LF_shp_Q14).as_mut_ptr() as *const libc::c_int,
            (sEncCtrl.Gains_Q16).as_mut_ptr() as *const libc::c_int,
            sEncCtrl.Lambda_Q10,
            sEncCtrl.LTP_scale_Q14,
        );
    }
    if (*psEnc).speech_activity_Q8
        < ((0.1f32
            * ((1 as libc::c_int as int64_t) << 8 as libc::c_int) as libc::c_float)
            as libc::c_double + 0.5f64) as libc::c_int
    {
        (*psEnc).sCmn.vadFlag = 0 as libc::c_int;
        (*psEnc).sCmn.noSpeechCounter += 1;
        (*psEnc).sCmn.noSpeechCounter;
        if (*psEnc).sCmn.noSpeechCounter > 5 as libc::c_int {
            (*psEnc).sCmn.inDTX = 1 as libc::c_int;
        }
        if (*psEnc).sCmn.noSpeechCounter > 20 as libc::c_int + 5 as libc::c_int {
            (*psEnc).sCmn.noSpeechCounter = 5 as libc::c_int;
            (*psEnc).sCmn.inDTX = 0 as libc::c_int;
        }
    } else {
        (*psEnc).sCmn.noSpeechCounter = 0 as libc::c_int;
        (*psEnc).sCmn.inDTX = 0 as libc::c_int;
        (*psEnc).sCmn.vadFlag = 1 as libc::c_int;
    }
    if (*psEnc).sCmn.nFramesInPayloadBuf == 0 as libc::c_int {
        SKP_Silk_range_enc_init(&mut (*psEnc).sCmn.sRC);
        (*psEnc).sCmn.nBytesInPayloadBuf = 0 as libc::c_int;
    }
    SKP_Silk_encode_parameters(
        &mut (*psEnc).sCmn,
        &mut sEncCtrl.sCmn,
        &mut (*psEnc).sCmn.sRC,
        ((*psEnc).sCmn.q).as_mut_ptr(),
    );
    FrameTermination_CDF = SKP_Silk_FrameTermination_CDF.as_ptr();
    memmove(
        ((*psEnc).x_buf).as_mut_ptr() as *mut libc::c_void,
        &mut *((*psEnc).x_buf).as_mut_ptr().offset((*psEnc).sCmn.frame_length as isize)
            as *mut libc::c_short as *const libc::c_void,
        (((*psEnc).sCmn.frame_length + 5 as libc::c_int * (*psEnc).sCmn.fs_kHz)
            as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as libc::c_ulong),
    );
    (*psEnc).sCmn.prev_sigtype = sEncCtrl.sCmn.sigtype;
    (*psEnc)
        .sCmn
        .prevLag = sEncCtrl.sCmn.pitchL[(4 as libc::c_int - 1 as libc::c_int) as usize];
    (*psEnc).sCmn.first_frame_after_reset = 0 as libc::c_int;
    if (*psEnc).sCmn.sRC.error != 0 {
        (*psEnc).sCmn.nFramesInPayloadBuf = 0 as libc::c_int;
    } else {
        (*psEnc).sCmn.nFramesInPayloadBuf += 1;
        (*psEnc).sCmn.nFramesInPayloadBuf;
    }
    if (*psEnc).sCmn.nFramesInPayloadBuf * 20 as libc::c_int
        >= (*psEnc).sCmn.PacketSize_ms
    {
        LBRR_idx = (*psEnc).sCmn.oldest_LBRR_idx + 1 as libc::c_int & 1 as libc::c_int;
        frame_terminator = 0 as libc::c_int;
        if (*psEnc).sCmn.LBRR_buffer[LBRR_idx as usize].usage == 1 as libc::c_int {
            frame_terminator = 2 as libc::c_int;
        }
        if (*psEnc).sCmn.LBRR_buffer[(*psEnc).sCmn.oldest_LBRR_idx as usize].usage
            == 2 as libc::c_int
        {
            frame_terminator = 3 as libc::c_int;
            LBRR_idx = (*psEnc).sCmn.oldest_LBRR_idx;
        }
        SKP_Silk_range_encoder(
            &mut (*psEnc).sCmn.sRC,
            frame_terminator,
            FrameTermination_CDF,
        );
        SKP_Silk_range_coder_get_length(&mut (*psEnc).sCmn.sRC, &mut nBytes);
        if *pnBytesOut as libc::c_int >= nBytes {
            SKP_Silk_range_enc_wrap_up(&mut (*psEnc).sCmn.sRC);
            memcpy(
                pCode as *mut libc::c_void,
                ((*psEnc).sCmn.sRC.buffer).as_mut_ptr() as *const libc::c_void,
                (nBytes as libc::c_ulong)
                    .wrapping_mul(
                        ::core::mem::size_of::<libc::c_uchar>() as libc::c_ulong,
                    ),
            );
            if frame_terminator > 1 as libc::c_int
                && *pnBytesOut as libc::c_int
                    >= nBytes + (*psEnc).sCmn.LBRR_buffer[LBRR_idx as usize].nBytes
            {
                memcpy(
                    &mut *pCode.offset(nBytes as isize) as *mut libc::c_uchar
                        as *mut libc::c_void,
                    ((*psEnc).sCmn.LBRR_buffer[LBRR_idx as usize].payload).as_mut_ptr()
                        as *const libc::c_void,
                    ((*psEnc).sCmn.LBRR_buffer[LBRR_idx as usize].nBytes
                        as libc::c_ulong)
                        .wrapping_mul(
                            ::core::mem::size_of::<libc::c_uchar>() as libc::c_ulong,
                        ),
                );
                nBytes += (*psEnc).sCmn.LBRR_buffer[LBRR_idx as usize].nBytes;
            }
            *pnBytesOut = nBytes as libc::c_short;
            memcpy(
                ((*psEnc)
                    .sCmn
                    .LBRR_buffer[(*psEnc).sCmn.oldest_LBRR_idx as usize]
                    .payload)
                    .as_mut_ptr() as *mut libc::c_void,
                LBRRpayload.as_mut_ptr() as *const libc::c_void,
                (nBytesLBRR as libc::c_ulong)
                    .wrapping_mul(
                        ::core::mem::size_of::<libc::c_uchar>() as libc::c_ulong,
                    ),
            );
            (*psEnc)
                .sCmn
                .LBRR_buffer[(*psEnc).sCmn.oldest_LBRR_idx as usize]
                .nBytes = nBytesLBRR as libc::c_int;
            (*psEnc)
                .sCmn
                .LBRR_buffer[(*psEnc).sCmn.oldest_LBRR_idx as usize]
                .usage = sEncCtrl.sCmn.LBRR_usage;
            (*psEnc)
                .sCmn
                .oldest_LBRR_idx = (*psEnc).sCmn.oldest_LBRR_idx + 1 as libc::c_int
                & 1 as libc::c_int;
        } else {
            *pnBytesOut = 0 as libc::c_int as libc::c_short;
            nBytes = 0 as libc::c_int;
            ret = -(4 as libc::c_int);
        }
        (*psEnc).sCmn.nFramesInPayloadBuf = 0 as libc::c_int;
    } else {
        *pnBytesOut = 0 as libc::c_int as libc::c_short;
        frame_terminator = 1 as libc::c_int;
        SKP_Silk_range_encoder(
            &mut (*psEnc).sCmn.sRC,
            frame_terminator,
            FrameTermination_CDF,
        );
        SKP_Silk_range_coder_get_length(&mut (*psEnc).sCmn.sRC, &mut nBytes);
    }
    if (*psEnc).sCmn.sRC.error != 0 {
        ret = -(9 as libc::c_int);
    }
    (*psEnc).BufferedInChannel_ms
        += 8 as libc::c_int * 1000 as libc::c_int
            * (nBytes - (*psEnc).sCmn.nBytesInPayloadBuf) / (*psEnc).sCmn.TargetRate_bps;
    (*psEnc).BufferedInChannel_ms -= 20 as libc::c_int;
    (*psEnc)
        .BufferedInChannel_ms = if 0 as libc::c_int > 100 as libc::c_int {
        if (*psEnc).BufferedInChannel_ms > 0 as libc::c_int {
            0 as libc::c_int
        } else if (*psEnc).BufferedInChannel_ms < 100 as libc::c_int {
            100 as libc::c_int
        } else {
            (*psEnc).BufferedInChannel_ms
        }
    } else if (*psEnc).BufferedInChannel_ms > 100 as libc::c_int {
        100 as libc::c_int
    } else if (*psEnc).BufferedInChannel_ms < 0 as libc::c_int {
        0 as libc::c_int
    } else {
        (*psEnc).BufferedInChannel_ms
    };
    (*psEnc).sCmn.nBytesInPayloadBuf = nBytes;
    if (*psEnc).speech_activity_Q8
        > ((0.7f32
            * ((1 as libc::c_int as int64_t) << 8 as libc::c_int) as libc::c_float)
            as libc::c_double + 0.5f64) as libc::c_int
    {
        (*psEnc)
            .sCmn
            .sSWBdetect
            .ActiveSpeech_ms = if ((*psEnc).sCmn.sSWBdetect.ActiveSpeech_ms
            + 20 as libc::c_int) as libc::c_uint & 0x80000000 as libc::c_uint != 0
        {
            0x7fffffff as libc::c_int
        } else {
            (*psEnc).sCmn.sSWBdetect.ActiveSpeech_ms + 20 as libc::c_int
        };
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_LBRR_encode_FIX(
    mut psEnc: *mut SKP_Silk_encoder_state_FIX,
    mut psEncCtrl: *mut SKP_Silk_encoder_control_FIX,
    mut pCode: *mut libc::c_uchar,
    mut pnBytesOut: *mut libc::c_short,
    mut xfw: *mut libc::c_short,
) {
    let mut TempGainsIndices: [libc::c_int; 4] = [0; 4];
    let mut frame_terminator: libc::c_int = 0;
    let mut nBytes: libc::c_int = 0;
    let mut nFramesInPayloadBuf: libc::c_int = 0;
    let mut TempGains_Q16: [libc::c_int; 4] = [0; 4];
    let mut typeOffset: libc::c_int = 0;
    let mut LTP_scaleIndex: libc::c_int = 0;
    let mut Rate_only_parameters: libc::c_int = 0 as libc::c_int;
    SKP_Silk_LBRR_ctrl_FIX(psEnc, &mut (*psEncCtrl).sCmn);
    if (*psEnc).sCmn.LBRR_enabled != 0 {
        memcpy(
            TempGainsIndices.as_mut_ptr() as *mut libc::c_void,
            ((*psEncCtrl).sCmn.GainsIndices).as_mut_ptr() as *const libc::c_void,
            (4 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
        );
        memcpy(
            TempGains_Q16.as_mut_ptr() as *mut libc::c_void,
            ((*psEncCtrl).Gains_Q16).as_mut_ptr() as *const libc::c_void,
            (4 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
        );
        typeOffset = (*psEnc).sCmn.typeOffsetPrev;
        LTP_scaleIndex = (*psEncCtrl).sCmn.LTP_scaleIndex;
        if (*psEnc).sCmn.fs_kHz == 8 as libc::c_int {
            Rate_only_parameters = 13500 as libc::c_int;
        } else if (*psEnc).sCmn.fs_kHz == 12 as libc::c_int {
            Rate_only_parameters = 15500 as libc::c_int;
        } else if (*psEnc).sCmn.fs_kHz == 16 as libc::c_int {
            Rate_only_parameters = 17500 as libc::c_int;
        } else if (*psEnc).sCmn.fs_kHz == 24 as libc::c_int {
            Rate_only_parameters = 19500 as libc::c_int;
        }
        if (*psEnc).sCmn.Complexity > 0 as libc::c_int
            && (*psEnc).sCmn.TargetRate_bps > Rate_only_parameters
        {
            if (*psEnc).sCmn.nFramesInPayloadBuf == 0 as libc::c_int {
                memcpy(
                    &mut (*psEnc).sCmn.sNSQ_LBRR as *mut SKP_Silk_nsq_state
                        as *mut libc::c_void,
                    &mut (*psEnc).sCmn.sNSQ as *mut SKP_Silk_nsq_state
                        as *const libc::c_void,
                    ::core::mem::size_of::<SKP_Silk_nsq_state>() as libc::c_ulong,
                );
                (*psEnc).sCmn.LBRRprevLastGainIndex = (*psEnc).sShape.LastGainIndex;
                (*psEncCtrl)
                    .sCmn
                    .GainsIndices[0 as libc::c_int
                    as usize] = (*psEncCtrl).sCmn.GainsIndices[0 as libc::c_int as usize]
                    + (*psEnc).sCmn.LBRR_GainIncreases;
                (*psEncCtrl)
                    .sCmn
                    .GainsIndices[0 as libc::c_int
                    as usize] = if 0 as libc::c_int
                    > 64 as libc::c_int - 1 as libc::c_int
                {
                    if (*psEncCtrl).sCmn.GainsIndices[0 as libc::c_int as usize]
                        > 0 as libc::c_int
                    {
                        0 as libc::c_int
                    } else if (*psEncCtrl).sCmn.GainsIndices[0 as libc::c_int as usize]
                        < 64 as libc::c_int - 1 as libc::c_int
                    {
                        64 as libc::c_int - 1 as libc::c_int
                    } else {
                        (*psEncCtrl).sCmn.GainsIndices[0 as libc::c_int as usize]
                    }
                } else if (*psEncCtrl).sCmn.GainsIndices[0 as libc::c_int as usize]
                    > 64 as libc::c_int - 1 as libc::c_int
                {
                    64 as libc::c_int - 1 as libc::c_int
                } else if (*psEncCtrl).sCmn.GainsIndices[0 as libc::c_int as usize]
                    < 0 as libc::c_int
                {
                    0 as libc::c_int
                } else {
                    (*psEncCtrl).sCmn.GainsIndices[0 as libc::c_int as usize]
                };
            }
            SKP_Silk_gains_dequant(
                ((*psEncCtrl).Gains_Q16).as_mut_ptr(),
                ((*psEncCtrl).sCmn.GainsIndices).as_mut_ptr() as *const libc::c_int,
                &mut (*psEnc).sCmn.LBRRprevLastGainIndex,
                (*psEnc).sCmn.nFramesInPayloadBuf,
            );
            if (*psEnc).sCmn.nStatesDelayedDecision > 1 as libc::c_int
                || (*psEnc).sCmn.warping_Q16 > 0 as libc::c_int
            {
                SKP_Silk_NSQ_del_dec(
                    &mut (*psEnc).sCmn,
                    &mut (*psEncCtrl).sCmn,
                    &mut (*psEnc).sCmn.sNSQ_LBRR,
                    xfw as *const libc::c_short,
                    ((*psEnc).sCmn.q_LBRR).as_mut_ptr(),
                    (*psEncCtrl).sCmn.NLSFInterpCoef_Q2,
                    ((*psEncCtrl).PredCoef_Q12[0 as libc::c_int as usize]).as_mut_ptr()
                        as *const libc::c_short,
                    ((*psEncCtrl).LTPCoef_Q14).as_mut_ptr() as *const libc::c_short,
                    ((*psEncCtrl).AR2_Q13).as_mut_ptr() as *const libc::c_short,
                    ((*psEncCtrl).HarmShapeGain_Q14).as_mut_ptr() as *const libc::c_int,
                    ((*psEncCtrl).Tilt_Q14).as_mut_ptr() as *const libc::c_int,
                    ((*psEncCtrl).LF_shp_Q14).as_mut_ptr() as *const libc::c_int,
                    ((*psEncCtrl).Gains_Q16).as_mut_ptr() as *const libc::c_int,
                    (*psEncCtrl).Lambda_Q10,
                    (*psEncCtrl).LTP_scale_Q14,
                );
            } else {
                SKP_Silk_NSQ(
                    &mut (*psEnc).sCmn,
                    &mut (*psEncCtrl).sCmn,
                    &mut (*psEnc).sCmn.sNSQ_LBRR,
                    xfw as *const libc::c_short,
                    ((*psEnc).sCmn.q_LBRR).as_mut_ptr(),
                    (*psEncCtrl).sCmn.NLSFInterpCoef_Q2,
                    ((*psEncCtrl).PredCoef_Q12[0 as libc::c_int as usize]).as_mut_ptr()
                        as *const libc::c_short,
                    ((*psEncCtrl).LTPCoef_Q14).as_mut_ptr() as *const libc::c_short,
                    ((*psEncCtrl).AR2_Q13).as_mut_ptr() as *const libc::c_short,
                    ((*psEncCtrl).HarmShapeGain_Q14).as_mut_ptr() as *const libc::c_int,
                    ((*psEncCtrl).Tilt_Q14).as_mut_ptr() as *const libc::c_int,
                    ((*psEncCtrl).LF_shp_Q14).as_mut_ptr() as *const libc::c_int,
                    ((*psEncCtrl).Gains_Q16).as_mut_ptr() as *const libc::c_int,
                    (*psEncCtrl).Lambda_Q10,
                    (*psEncCtrl).LTP_scale_Q14,
                );
            }
        } else {
            memset(
                ((*psEnc).sCmn.q_LBRR).as_mut_ptr() as *mut libc::c_void,
                0 as libc::c_int,
                ((*psEnc).sCmn.frame_length as libc::c_ulong)
                    .wrapping_mul(
                        ::core::mem::size_of::<libc::c_schar>() as libc::c_ulong,
                    ),
            );
            (*psEncCtrl).sCmn.LTP_scaleIndex = 0 as libc::c_int;
        }
        if (*psEnc).sCmn.nFramesInPayloadBuf == 0 as libc::c_int {
            SKP_Silk_range_enc_init(&mut (*psEnc).sCmn.sRC_LBRR);
            (*psEnc).sCmn.nBytesInPayloadBuf = 0 as libc::c_int;
        }
        SKP_Silk_encode_parameters(
            &mut (*psEnc).sCmn,
            &mut (*psEncCtrl).sCmn,
            &mut (*psEnc).sCmn.sRC_LBRR,
            ((*psEnc).sCmn.q_LBRR).as_mut_ptr(),
        );
        if (*psEnc).sCmn.sRC_LBRR.error != 0 {
            nFramesInPayloadBuf = 0 as libc::c_int;
        } else {
            nFramesInPayloadBuf = (*psEnc).sCmn.nFramesInPayloadBuf + 1 as libc::c_int;
        }
        if nFramesInPayloadBuf as libc::c_short as libc::c_int
            * 20 as libc::c_int as libc::c_short as libc::c_int
            >= (*psEnc).sCmn.PacketSize_ms
        {
            frame_terminator = 0 as libc::c_int;
            SKP_Silk_range_encoder(
                &mut (*psEnc).sCmn.sRC_LBRR,
                frame_terminator,
                SKP_Silk_FrameTermination_CDF.as_ptr(),
            );
            SKP_Silk_range_coder_get_length(&mut (*psEnc).sCmn.sRC_LBRR, &mut nBytes);
            if *pnBytesOut as libc::c_int >= nBytes {
                SKP_Silk_range_enc_wrap_up(&mut (*psEnc).sCmn.sRC_LBRR);
                memcpy(
                    pCode as *mut libc::c_void,
                    ((*psEnc).sCmn.sRC_LBRR.buffer).as_mut_ptr() as *const libc::c_void,
                    (nBytes as libc::c_ulong)
                        .wrapping_mul(
                            ::core::mem::size_of::<libc::c_uchar>() as libc::c_ulong,
                        ),
                );
                *pnBytesOut = nBytes as libc::c_short;
            } else {
                *pnBytesOut = 0 as libc::c_int as libc::c_short;
            }
        } else {
            *pnBytesOut = 0 as libc::c_int as libc::c_short;
            frame_terminator = 1 as libc::c_int;
            SKP_Silk_range_encoder(
                &mut (*psEnc).sCmn.sRC_LBRR,
                frame_terminator,
                SKP_Silk_FrameTermination_CDF.as_ptr(),
            );
        }
        memcpy(
            ((*psEncCtrl).sCmn.GainsIndices).as_mut_ptr() as *mut libc::c_void,
            TempGainsIndices.as_mut_ptr() as *const libc::c_void,
            (4 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
        );
        memcpy(
            ((*psEncCtrl).Gains_Q16).as_mut_ptr() as *mut libc::c_void,
            TempGains_Q16.as_mut_ptr() as *const libc::c_void,
            (4 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
        );
        (*psEncCtrl).sCmn.LTP_scaleIndex = LTP_scaleIndex;
        (*psEnc).sCmn.typeOffsetPrev = typeOffset;
    }
}
