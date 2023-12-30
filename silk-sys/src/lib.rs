pub mod skp_silk_dec_api;
pub mod SKP_Silk_range_coder;
pub mod SKP_Silk_gain_quant;
pub mod SKP_Silk_lin2log;


pub mod SKP_Silk_code_signs;
pub mod SKP_Silk_MA;

pub mod skp_silk_resampler;
pub mod skp_silk_resampler_up2;
pub mod skp_silk_resampler_down2;
pub mod skp_silk_resampler_private_up4;
pub mod skp_silk_resampler_private_up2_hq;
pub mod skp_silk_resampler_private_iir_fir;
pub mod skp_silk_resampler_private_down_fir;
pub mod skp_silk_resampler_private_copy;
pub mod skp_silk_resampler_private_arma4;
pub mod skp_silk_resampler_private_down4;
pub mod skp_silk_resampler_private_ar2;
pub mod skp_silk_cng;
pub mod skp_silk_decode_pitch;
pub mod skp_silk_decoder_set_fs;
pub mod skp_silk_decode_frame;
pub mod skp_silk_biquad;
pub mod skp_silk_lpc_synthesis_filter;
pub mod skp_silk_lpc_synthesis_order16;
pub mod skp_silk_plc;
pub mod skp_silk_sum_sqr_shift;
pub mod skp_silk_shell_coder;
pub mod skp_silk_tables_ltp;
pub mod skp_silk_decode_core;
pub mod skp_silk_decode_parameters;
pub mod skp_silk_nlsf2a_stable;
pub mod skp_silk_lpc_inv_pred_gain;
pub mod skp_silk_log2lin;
pub mod skp_silk_nlsf2a;
pub mod skp_silk_decode_pulses;
pub mod skp_silk_nlsf_stabilize;
pub mod skp_silk_bwexpander;
pub mod skp_silk_nlsf_msvq_decode;
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
pub mod skp_utils;

pub mod error;

use bytes::Buf;

use crate::{error::SilkError, skp_silk_dec_api::{SkpSilkSdkDecControlStruct, skp_silk_sdk_decode, SkpSilkDecoderStruct }};

pub fn decode_silk(src: impl AsRef<[u8]>, sample_rate: u32) -> Result<Vec<i16>, SilkError> {
    unsafe { _decode_silk(src.as_ref(), sample_rate as i32) }
}

unsafe fn _decode_silk(mut src: &[u8], sample_rate: i32) -> Result<Vec<i16>, SilkError> {

    const SILK_HEADER: &[u8] = b"#!SILK_V3";
    if src.starts_with(SILK_HEADER) {
        src.advance(SILK_HEADER.len());
    } else {
        return Err(SilkError::Invalid);
    };

    let mut dec_control = SkpSilkSdkDecControlStruct {
        api_sample_rate: sample_rate,
        frame_size: 0,
        frames_per_packet: 1,
        more_internal_decoder_frames: 0,
        in_band_fec_offset: 0,
    };

    let mut decoder = SkpSilkDecoderStruct::default();
    decoder.init();
    let mut result = vec![];
    let frame_size = 960;
    let mut buf = vec![0i16; frame_size];
//    let mut aaa = 0;
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

        let mut output_size = 0;
        let r = skp_silk_sdk_decode(
            &mut decoder,
            &mut dec_control,
            0,
            input,
            input_size as i32,
            &mut buf,
            &mut output_size,
        );
        //println!("data: {aaa} \n{:?}", buf);
        //aaa += 1;
        // println!("output_size : {}, buf : {:?}", output_size, &buf[..i16::min(50,output_size) as usize]);
        if r != 0 {
            return Err(r.into());
        }
        result.extend_from_slice(&buf[0..output_size])
    }
    Ok(result)
}