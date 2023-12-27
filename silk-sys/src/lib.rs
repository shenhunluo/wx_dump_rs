pub mod SKP_Silk_create_init_destroy;
pub mod SKP_Silk_dec_API;
pub mod SKP_Silk_resampler;
pub mod SKP_Silk_resampler_private_up2_HQ;
pub mod SKP_Silk_resampler_up2;
pub mod SKP_Silk_resampler_down2;
pub mod SKP_Silk_resampler_private_up4;
pub mod SKP_Silk_resampler_private_down4;
pub mod SKP_Silk_resampler_private_down_FIR;
pub mod SKP_Silk_resampler_private_IIR_FIR;
pub mod SKP_Silk_resampler_private_AR2;
pub mod SKP_Silk_resampler_private_ARMA4;
pub mod SKP_Silk_resampler_private_copy;
pub mod SKP_Silk_decode_frame;
pub mod SKP_Silk_decode_parameters;
pub mod SKP_Silk_range_coder;
pub mod SKP_Silk_decoder_set_fs;
pub mod SKP_Silk_CNG;
pub mod SKP_Silk_PLC;
pub mod SKP_Silk_biquad;
pub mod SKP_Silk_decode_core;
pub mod SKP_Silk_NLSF_MSVQ_decode;
pub mod SKP_Silk_bwexpander;
pub mod SKP_Silk_decode_pitch;
pub mod SKP_Silk_gain_quant;
pub mod SKP_Silk_NLSF2A_stable;
pub mod skp_silk_decode_pulses;
pub mod SKP_Silk_LPC_synthesis_order16;
pub mod SKP_Silk_LPC_synthesis_filter;
pub mod SKP_Silk_sum_sqr_shift;
pub mod SKP_Silk_LPC_inv_pred_gain;
pub mod SKP_Silk_MA;
pub mod skp_silk_nlsf_stabilize;
pub mod SKP_Silk_lin2log;
pub mod skp_silk_log2lin;
pub mod SKP_Silk_NLSF2A;
pub mod SKP_Silk_shell_coder;
pub mod SKP_Silk_code_signs;

pub mod SKP_Silk_tables_LTP;


pub mod SKP_Silk_sort;
pub mod skp_silk_bwexpander_32;
pub mod skp_silk_resampler_rom;
pub mod skp_silk_tables_other;
pub mod skp_silk_tables_type_offset;
pub mod skp_silk_tables_gain;
pub mod skp_silk_tables_pitch_lag;
pub mod skp_silk_tables_nlsf_cb0_10;
pub mod skp_silk_tables_nlsf_cb1_10;
pub mod skp_silk_tables_nlsf_cb0_16;
pub mod skp_silk_tables_nlsf_cb1_16;
pub mod skp_silk_pitch_est_tables;
pub mod skp_silk_tables_pulses_per_block;
pub mod skp_silk_tables_sign;
pub mod skp_silk_lsf_cos_table;

pub mod skp_silk_macro;

pub mod error;

use bytes::Buf;

use crate::{error::SilkError, SKP_Silk_dec_API::{SKP_SILK_SDK_DecControlStruct, SKP_Silk_SDK_Decode, SKP_Silk_decoder_state}};

pub fn decode_silk(src: impl AsRef<[u8]>, sample_rate: i32) -> Result<Vec<u8>, SilkError> {
    unsafe { _decode_silk(src.as_ref(), sample_rate) }
}

unsafe fn _decode_silk(mut src: &[u8], sample_rate: i32) -> Result<Vec<u8>, SilkError> {
    // skip tencent flag
    if src.starts_with(b"\x02") {
        src.advance(1);
    };

    const SILK_HEADER: &[u8] = b"#!SILK_V3";
    if src.starts_with(SILK_HEADER) {
        src.advance(SILK_HEADER.len());
    } else {
        return Err(SilkError::Invalid);
    };

    let mut dec_control = SKP_SILK_SDK_DecControlStruct {
        API_sampleRate: sample_rate,
        frameSize: 0,
        framesPerPacket: 1,
        moreInternalDecoderFrames: 0,
        inBandFECOffset: 0,
    };

    let mut decoder = SKP_Silk_decoder_state::default();
    let mut result = vec![];
    let frame_size = sample_rate as usize / 1000 * 40;
    let mut buf = vec![0u8; frame_size];
    loop {
        if src.remaining() < 2 {
            break;
        }
        let input_size = src.get_i16_le();
        if input_size > frame_size as i16 {
            return Err(SilkError::Invalid);
        }
        if src.remaining() < input_size as usize {
            return Err(SilkError::Invalid);
        }

        let input;
        (input, src) = src.split_at(input_size as usize);

        let mut output_size = 0i16;
        let r = SKP_Silk_SDK_Decode(
            &mut decoder,
            &mut dec_control,
            0,
            input,
            input_size as i32,
            buf.as_mut_ptr() as *mut i16,
            &mut output_size,
        );
        if r != 0 {
            return Err(r.into());
        }

        result.extend_from_slice(&buf[0..output_size as usize * 2])
    }
    Ok(result)
}