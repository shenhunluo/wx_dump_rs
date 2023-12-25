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
}

use crate::SKP_Silk_CNG::SKP_Silk_CNG_struct;
use crate::SKP_Silk_PLC::SKP_Silk_PLC_struct;
use crate::SKP_Silk_NLSF_MSVQ_decode::SKP_Silk_NLSF_CB_struct;
use super::SKP_Silk_create_init_destroy::SKP_Silk_init_decoder;
use super::SKP_Silk_resampler::{
    SKP_Silk_resampler,
    SKP_Silk_resampler_init,
    SKP_Silk_resampler_state_struct,
};
use super::SKP_Silk_decode_frame::SKP_Silk_decode_frame;
use super::SKP_Silk_decode_parameters::SKP_Silk_decode_parameters;
use super::SKP_Silk_range_coder::{
    SKP_Silk_range_dec_init,
    SKP_Silk_range_coder_state,
};

#[derive(Copy, Clone)]
#[repr(C)]
pub struct SKP_Silk_decoder_state {
    pub sRC: SKP_Silk_range_coder_state,
    pub prev_inv_gain_Q16: libc::c_int,
    pub sLTP_Q16: [libc::c_int; 960],
    pub sLPC_Q14: [libc::c_int; 136],
    pub exc_Q10: [libc::c_int; 480],
    pub res_Q10: [libc::c_int; 480],
    pub outBuf: [libc::c_short; 960],
    pub lagPrev: libc::c_int,
    pub LastGainIndex: libc::c_int,
    pub LastGainIndex_EnhLayer: libc::c_int,
    pub typeOffsetPrev: libc::c_int,
    pub HPState: [libc::c_int; 2],
    pub HP_A: *const libc::c_short,
    pub HP_B: *const libc::c_short,
    pub fs_kHz: libc::c_int,
    pub prev_API_sampleRate: libc::c_int,
    pub frame_length: libc::c_int,
    pub subfr_length: libc::c_int,
    pub LPC_order: libc::c_int,
    pub prevNLSF_Q15: [libc::c_int; 16],
    pub first_frame_after_reset: libc::c_int,
    pub nBytesLeft: libc::c_int,
    pub nFramesDecoded: libc::c_int,
    pub nFramesInPacket: libc::c_int,
    pub moreInternalDecoderFrames: libc::c_int,
    pub FrameTermination: libc::c_int,
    pub resampler_state: SKP_Silk_resampler_state_struct,
    pub psNLSF_CB: [Option<&'static SKP_Silk_NLSF_CB_struct>; 2],
    pub vadFlag: libc::c_int,
    pub no_FEC_counter: libc::c_int,
    pub inband_FEC_offset: libc::c_int,
    pub sCNG: SKP_Silk_CNG_struct,
    pub lossCnt: libc::c_int,
    pub prev_sigtype: libc::c_int,
    pub sPLC: SKP_Silk_PLC_struct,
}

#[derive(Copy, Clone,Debug)]
#[repr(C)]
pub struct SKP_SILK_SDK_DecControlStruct {
    pub API_sampleRate: libc::c_int,
    pub frameSize: libc::c_int,
    pub framesPerPacket: libc::c_int,
    pub moreInternalDecoderFrames: libc::c_int,
    pub inBandFECOffset: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SKP_Silk_TOC_struct {
    pub framesInPacket: libc::c_int,
    pub fs_kHz: libc::c_int,
    pub inbandLBRR: libc::c_int,
    pub corrupt: libc::c_int,
    pub vadFlags: [libc::c_int; 5],
    pub sigtypeFlags: [libc::c_int; 5],
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
pub struct SKP_Silk_decoder_control {
    pub pitchL: [libc::c_int; 4],
    pub Gains_Q16: [libc::c_int; 4],
    pub Seed: libc::c_int,
    pub PredCoef_Q12: [[libc::c_short; 16]; 2],
    pub LTPCoef_Q14: [libc::c_short; 20],
    pub LTP_scale_Q14: libc::c_int,
    pub PERIndex: libc::c_int,
    pub RateLevelIndex: libc::c_int,
    pub QuantOffsetType: libc::c_int,
    pub sigtype: libc::c_int,
    pub NLSFInterpCoef_Q2: libc::c_int,
}
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_SDK_Get_Decoder_Size(
    mut decSizeBytes: *mut libc::c_int,
) -> libc::c_int {
    let mut ret: libc::c_int = 0 as libc::c_int;
    *decSizeBytes = ::core::mem::size_of::<SKP_Silk_decoder_state>() as libc::c_ulong
        as libc::c_int;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_SDK_InitDecoder(
    mut decState: *mut libc::c_void,
) -> libc::c_int {
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut struc: *mut SKP_Silk_decoder_state = 0 as *mut SKP_Silk_decoder_state;
    struc = decState as *mut SKP_Silk_decoder_state;
    ret = SKP_Silk_init_decoder(struc);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_SDK_Decode(
    mut decState: *mut libc::c_void,
    mut decControl: *mut SKP_SILK_SDK_DecControlStruct,
    mut lostFlag: libc::c_int,
    mut inData: *const libc::c_uchar,
    nBytesIn: libc::c_int,
    mut samplesOut: *mut libc::c_short,
    mut nSamplesOut: *mut libc::c_short,
) -> libc::c_int {
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut used_bytes: libc::c_int = 0;
    let mut prev_fs_kHz: libc::c_int = 0;
    let mut psDec: *mut SKP_Silk_decoder_state = 0 as *mut SKP_Silk_decoder_state;
    let mut samplesOutInternal: [libc::c_short; 960] = [0; 960];
    let mut pSamplesOutInternal: *mut libc::c_short = 0 as *mut libc::c_short;
    psDec = decState as *mut SKP_Silk_decoder_state;
    pSamplesOutInternal = samplesOut;
    if (*psDec).fs_kHz * 1000 as libc::c_int > (*decControl).API_sampleRate {
        pSamplesOutInternal = samplesOutInternal.as_mut_ptr();
    }
    if (*psDec).moreInternalDecoderFrames == 0 as libc::c_int {
        (*psDec).nFramesDecoded = 0 as libc::c_int;
    }
    if (*psDec).moreInternalDecoderFrames == 0 as libc::c_int
        && lostFlag == 0 as libc::c_int && nBytesIn > 1024 as libc::c_int
    {
        lostFlag = 1 as libc::c_int;
        ret = -(11 as libc::c_int);
    }
    prev_fs_kHz = (*psDec).fs_kHz;
    ret
        += SKP_Silk_decode_frame(
            psDec,
            pSamplesOutInternal,
            nSamplesOut,
            inData,
            nBytesIn,
            lostFlag,
            &mut used_bytes,
        );
    if used_bytes != 0 {
        if (*psDec).nBytesLeft > 0 as libc::c_int
            && (*psDec).FrameTermination == 1 as libc::c_int
            && (*psDec).nFramesDecoded < 5 as libc::c_int
        {
            (*psDec).moreInternalDecoderFrames = 1 as libc::c_int;
        } else {
            (*psDec).moreInternalDecoderFrames = 0 as libc::c_int;
            (*psDec).nFramesInPacket = (*psDec).nFramesDecoded;
            if (*psDec).vadFlag == 1 as libc::c_int {
                if (*psDec).FrameTermination == 0 as libc::c_int {
                    (*psDec).no_FEC_counter += 1;
                    (*psDec).no_FEC_counter;
                    if (*psDec).no_FEC_counter > 10 as libc::c_int {
                        (*psDec).inband_FEC_offset = 0 as libc::c_int;
                    }
                } else if (*psDec).FrameTermination == 2 as libc::c_int {
                    (*psDec).inband_FEC_offset = 1 as libc::c_int;
                    (*psDec).no_FEC_counter = 0 as libc::c_int;
                } else if (*psDec).FrameTermination == 3 as libc::c_int {
                    (*psDec).inband_FEC_offset = 2 as libc::c_int;
                    (*psDec).no_FEC_counter = 0 as libc::c_int;
                }
            }
        }
    }
    if (48 as libc::c_int * 1000 as libc::c_int) < (*decControl).API_sampleRate
        || 8000 as libc::c_int > (*decControl).API_sampleRate
    {
        ret = -(10 as libc::c_int);
        return ret;
    }
    if (*psDec).fs_kHz * 1000 as libc::c_int != (*decControl).API_sampleRate {
        let mut samplesOut_tmp: [libc::c_short; 960] = [0; 960];
        memcpy(
            samplesOut_tmp.as_mut_ptr() as *mut libc::c_void,
            pSamplesOutInternal as *const libc::c_void,
            (*nSamplesOut as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_short>() as libc::c_ulong),
        );
        if prev_fs_kHz != (*psDec).fs_kHz
            || (*psDec).prev_API_sampleRate != (*decControl).API_sampleRate
        {
            ret = SKP_Silk_resampler_init(
                &mut (*psDec).resampler_state,
                (*psDec).fs_kHz as libc::c_short as libc::c_int
                    * 1000 as libc::c_int as libc::c_short as libc::c_int,
                (*decControl).API_sampleRate,
            );
        }
        ret
            += SKP_Silk_resampler(
                &mut (*psDec).resampler_state,
                samplesOut,
                samplesOut_tmp.as_mut_ptr() as *const libc::c_short,
                *nSamplesOut as libc::c_int,
            );
        *nSamplesOut = (*nSamplesOut as libc::c_int * (*decControl).API_sampleRate
            / ((*psDec).fs_kHz * 1000 as libc::c_int)) as libc::c_short;
    } else if prev_fs_kHz * 1000 as libc::c_int > (*decControl).API_sampleRate {
        memcpy(
            samplesOut as *mut libc::c_void,
            pSamplesOutInternal as *const libc::c_void,
            (*nSamplesOut as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_short>() as libc::c_ulong),
        );
    }
    (*psDec).prev_API_sampleRate = (*decControl).API_sampleRate;
    (*decControl)
        .frameSize = ((*decControl).API_sampleRate / 50 as libc::c_int) as libc::c_ushort
        as libc::c_int;
    (*decControl).framesPerPacket = (*psDec).nFramesInPacket;
    (*decControl).inBandFECOffset = (*psDec).inband_FEC_offset;
    (*decControl).moreInternalDecoderFrames = (*psDec).moreInternalDecoderFrames;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_SDK_search_for_LBRR(
    mut inData: *const libc::c_uchar,
    nBytesIn: libc::c_int,
    mut lost_offset: libc::c_int,
    mut LBRRData: *mut libc::c_uchar,
    mut nLBRRBytes: *mut libc::c_short,
) {
    let mut sDec: SKP_Silk_decoder_state = SKP_Silk_decoder_state {
        sRC: SKP_Silk_range_coder_state {
            bufferLength: 0,
            bufferIx: 0,
            base_Q32: 0,
            range_Q16: 0,
            error: 0,
            buffer: [0; 1024],
        },
        prev_inv_gain_Q16: 0,
        sLTP_Q16: [0; 960],
        sLPC_Q14: [0; 136],
        exc_Q10: [0; 480],
        res_Q10: [0; 480],
        outBuf: [0; 960],
        lagPrev: 0,
        LastGainIndex: 0,
        LastGainIndex_EnhLayer: 0,
        typeOffsetPrev: 0,
        HPState: [0; 2],
        HP_A: 0 as *const libc::c_short,
        HP_B: 0 as *const libc::c_short,
        fs_kHz: 0,
        prev_API_sampleRate: 0,
        frame_length: 0,
        subfr_length: 0,
        LPC_order: 0,
        prevNLSF_Q15: [0; 16],
        first_frame_after_reset: 0,
        nBytesLeft: 0,
        nFramesDecoded: 0,
        nFramesInPacket: 0,
        moreInternalDecoderFrames: 0,
        FrameTermination: 0,
        resampler_state: SKP_Silk_resampler_state_struct {
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
        },
        psNLSF_CB: [None; 2],
        vadFlag: 0,
        no_FEC_counter: 0,
        inband_FEC_offset: 0,
        sCNG: SKP_Silk_CNG_struct {
            CNG_exc_buf_Q10: [0; 480],
            CNG_smth_NLSF_Q15: [0; 16],
            CNG_synth_state: [0; 16],
            CNG_smth_Gain_Q16: 0,
            rand_seed: 0,
            fs_kHz: 0,
        },
        lossCnt: 0,
        prev_sigtype: 0,
        sPLC: SKP_Silk_PLC_struct {
            pitchL_Q8: 0,
            LTPCoef_Q14: [0; 5],
            prevLPC_Q12: [0; 16],
            last_frame_lost: 0,
            rand_seed: 0,
            randScale_Q14: 0,
            conc_energy: 0,
            conc_energy_shift: 0,
            prevLTP_scale_Q14: 0,
            prevGain_Q16: [0; 4],
            fs_kHz: 0,
        },
    };
    let mut sDecCtrl: SKP_Silk_decoder_control = SKP_Silk_decoder_control {
        pitchL: [0; 4],
        Gains_Q16: [0; 4],
        Seed: 0,
        PredCoef_Q12: [[0; 16]; 2],
        LTPCoef_Q14: [0; 20],
        LTP_scale_Q14: 0,
        PERIndex: 0,
        RateLevelIndex: 0,
        QuantOffsetType: 0,
        sigtype: 0,
        NLSFInterpCoef_Q2: 0,
    };
    let mut TempQ: [libc::c_int; 480] = [0; 480];
    if lost_offset < 1 as libc::c_int || lost_offset > 2 as libc::c_int {
        *nLBRRBytes = 0 as libc::c_int as libc::c_short;
        return;
    }
    sDec.nFramesDecoded = 0 as libc::c_int;
    sDec.fs_kHz = 0 as libc::c_int;
    sDec.lossCnt = 0 as libc::c_int;
    memset(
        (sDec.prevNLSF_Q15).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        (16 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    SKP_Silk_range_dec_init(&mut sDec.sRC, inData, nBytesIn);
    loop {
        SKP_Silk_decode_parameters(
            &mut sDec,
            &mut sDecCtrl,
            TempQ.as_mut_ptr(),
            0 as libc::c_int,
        );
        if sDec.sRC.error != 0 {
            *nLBRRBytes = 0 as libc::c_int as libc::c_short;
            return;
        }
        if sDec.FrameTermination - 1 as libc::c_int & lost_offset != 0
            && sDec.FrameTermination > 0 as libc::c_int
            && sDec.nBytesLeft >= 0 as libc::c_int
        {
            *nLBRRBytes = sDec.nBytesLeft as libc::c_short;
            memcpy(
                LBRRData as *mut libc::c_void,
                &*inData.offset((nBytesIn - sDec.nBytesLeft) as isize)
                    as *const libc::c_uchar as *const libc::c_void,
                (sDec.nBytesLeft as libc::c_ulong)
                    .wrapping_mul(
                        ::core::mem::size_of::<libc::c_uchar>() as libc::c_ulong,
                    ),
            );
            break;
        } else if sDec.nBytesLeft > 0 as libc::c_int
            && sDec.FrameTermination == 1 as libc::c_int
        {
            sDec.nFramesDecoded += 1;
            sDec.nFramesDecoded;
        } else {
            *nLBRRBytes = 0 as libc::c_int as libc::c_short;
            break;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_SDK_get_TOC(
    mut inData: *const libc::c_uchar,
    nBytesIn: libc::c_int,
    mut Silk_TOC: *mut SKP_Silk_TOC_struct,
) {
    let mut sDec: SKP_Silk_decoder_state = SKP_Silk_decoder_state {
        sRC: SKP_Silk_range_coder_state {
            bufferLength: 0,
            bufferIx: 0,
            base_Q32: 0,
            range_Q16: 0,
            error: 0,
            buffer: [0; 1024],
        },
        prev_inv_gain_Q16: 0,
        sLTP_Q16: [0; 960],
        sLPC_Q14: [0; 136],
        exc_Q10: [0; 480],
        res_Q10: [0; 480],
        outBuf: [0; 960],
        lagPrev: 0,
        LastGainIndex: 0,
        LastGainIndex_EnhLayer: 0,
        typeOffsetPrev: 0,
        HPState: [0; 2],
        HP_A: 0 as *const libc::c_short,
        HP_B: 0 as *const libc::c_short,
        fs_kHz: 0,
        prev_API_sampleRate: 0,
        frame_length: 0,
        subfr_length: 0,
        LPC_order: 0,
        prevNLSF_Q15: [0; 16],
        first_frame_after_reset: 0,
        nBytesLeft: 0,
        nFramesDecoded: 0,
        nFramesInPacket: 0,
        moreInternalDecoderFrames: 0,
        FrameTermination: 0,
        resampler_state: SKP_Silk_resampler_state_struct {
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
        },
        psNLSF_CB: [None; 2],
        vadFlag: 0,
        no_FEC_counter: 0,
        inband_FEC_offset: 0,
        sCNG: SKP_Silk_CNG_struct {
            CNG_exc_buf_Q10: [0; 480],
            CNG_smth_NLSF_Q15: [0; 16],
            CNG_synth_state: [0; 16],
            CNG_smth_Gain_Q16: 0,
            rand_seed: 0,
            fs_kHz: 0,
        },
        lossCnt: 0,
        prev_sigtype: 0,
        sPLC: SKP_Silk_PLC_struct {
            pitchL_Q8: 0,
            LTPCoef_Q14: [0; 5],
            prevLPC_Q12: [0; 16],
            last_frame_lost: 0,
            rand_seed: 0,
            randScale_Q14: 0,
            conc_energy: 0,
            conc_energy_shift: 0,
            prevLTP_scale_Q14: 0,
            prevGain_Q16: [0; 4],
            fs_kHz: 0,
        },
    };
    let mut sDecCtrl: SKP_Silk_decoder_control = SKP_Silk_decoder_control {
        pitchL: [0; 4],
        Gains_Q16: [0; 4],
        Seed: 0,
        PredCoef_Q12: [[0; 16]; 2],
        LTPCoef_Q14: [0; 20],
        LTP_scale_Q14: 0,
        PERIndex: 0,
        RateLevelIndex: 0,
        QuantOffsetType: 0,
        sigtype: 0,
        NLSFInterpCoef_Q2: 0,
    };
    let mut TempQ: [libc::c_int; 480] = [0; 480];
    sDec.nFramesDecoded = 0 as libc::c_int;
    sDec.fs_kHz = 0 as libc::c_int;
    SKP_Silk_range_dec_init(&mut sDec.sRC, inData, nBytesIn);
    (*Silk_TOC).corrupt = 0 as libc::c_int;
    loop {
        SKP_Silk_decode_parameters(
            &mut sDec,
            &mut sDecCtrl,
            TempQ.as_mut_ptr(),
            0 as libc::c_int,
        );
        (*Silk_TOC).vadFlags[sDec.nFramesDecoded as usize] = sDec.vadFlag;
        (*Silk_TOC).sigtypeFlags[sDec.nFramesDecoded as usize] = sDecCtrl.sigtype;
        if sDec.sRC.error != 0 {
            (*Silk_TOC).corrupt = 1 as libc::c_int;
            break;
        } else {
            if !(sDec.nBytesLeft > 0 as libc::c_int
                && sDec.FrameTermination == 1 as libc::c_int)
            {
                break;
            }
            sDec.nFramesDecoded += 1;
            sDec.nFramesDecoded;
        }
    }
    if (*Silk_TOC).corrupt != 0 || sDec.FrameTermination == 1 as libc::c_int
        || sDec.nFramesInPacket > 5 as libc::c_int
    {
        memset(
            Silk_TOC as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<SKP_Silk_TOC_struct>() as libc::c_ulong,
        );
        (*Silk_TOC).corrupt = 1 as libc::c_int;
    } else {
        (*Silk_TOC).framesInPacket = sDec.nFramesDecoded + 1 as libc::c_int;
        (*Silk_TOC).fs_kHz = sDec.fs_kHz;
        if sDec.FrameTermination == 0 as libc::c_int {
            (*Silk_TOC).inbandLBRR = sDec.FrameTermination;
        } else {
            (*Silk_TOC).inbandLBRR = sDec.FrameTermination - 1 as libc::c_int;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_SDK_get_version() -> *const libc::c_char {
    static mut version: [libc::c_char; 8] = unsafe {
        *::core::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"1.0.9.6\0")
    };
    return version.as_ptr();
}
