use crate::{
    skp_silk_bwexpander::skp_silk_bwexpander,
    skp_silk_decode_pitch::skp_silk_decode_pitch,
    skp_silk_decode_pulses::skp_silk_decode_pulses,
    skp_silk_nlsf2a_stable::skp_silk_nlsf2a_stable,
    skp_silk_nlsf_msvq_decode::{skp_silk_nlsf_msvq_decode, SkpSilkNlsfCbStruct},
    skp_silk_tables_gain::{
        SKP_SILK_DELTA_GAIN_CDF, SKP_SILK_DELTA_GAIN_CDF_OFFSET, SKP_SILK_GAIN_CDF,
        SKP_SILK_GAIN_CDF_OFFSET,
    },
    skp_silk_tables_other::{
        SKP_SILK_FRAME_TERMINATION_CDF, SKP_SILK_FRAME_TERMINATION_OFFSET,
        SKP_SILK_LTP_SCALES_TABLE_Q14, SKP_SILK_LTP_SCALE_CDF, SKP_SILK_LTP_SCALE_OFFSET,
        SKP_SILK_NLSF_INTERPOLATION_FACTOR_CDF, SKP_SILK_NLSF_INTERPOLATION_FACTOR_OFFSET,
        SKP_SILK_SAMPLING_RATES_CDF, SKP_SILK_SAMPLING_RATES_OFFSET, SKP_SILK_SAMPLING_RATES_TABLE,
        SKP_SILK_SEED_CDF, SKP_SILK_SEED_OFFSET, SKP_SILK_VAD_FLAG_CDF, SKP_SILK_VAD_FLAG_OFFSET,
    },
    skp_silk_tables_pitch_lag::{
        SKP_SILK_PITCH_CONTOUR_CDF, SKP_SILK_PITCH_CONTOUR_CDF_OFFSET,
        SKP_SILK_PITCH_CONTOUR_NB_CDF, SKP_SILK_PITCH_CONTOUR_NB_CDF_OFFSET,
        SKP_SILK_PITCH_LAG_MB_CDF, SKP_SILK_PITCH_LAG_MB_CDF_OFFSET, SKP_SILK_PITCH_LAG_NB_CDF,
        SKP_SILK_PITCH_LAG_NB_CDF_OFFSET, SKP_SILK_PITCH_LAG_SWB_CDF,
        SKP_SILK_PITCH_LAG_SWB_CDF_OFFSET, SKP_SILK_PITCH_LAG_WB_CDF,
        SKP_SILK_PITCH_LAG_WB_CDF_OFFSET,
    },
    skp_silk_tables_type_offset::{
        SKP_SILK_TYPE_OFFSET_CDF, SKP_SILK_TYPE_OFFSET_CDF_OFFSET, SKP_SILK_TYPE_OFFSET_JOINT_CDF,
    },
    SKP_Silk_dec_API::{SKP_Silk_decoder_control, SKP_Silk_decoder_state},
    SKP_Silk_decoder_set_fs::SKP_Silk_decoder_set_fs,
    SKP_Silk_gain_quant::skp_silk_gains_dequant,
    SKP_Silk_range_coder::{
        skp_silk_range_coder_check_after_decoding, skp_silk_range_coder_get_length,
        skp_silk_range_decoder_multi, SKP_Silk_range_decoder,
    },
    SKP_Silk_tables_LTP::{
        SKP_Silk_LTP_gain_CDF_offsets, SKP_Silk_LTP_gain_CDF_ptrs, SKP_Silk_LTP_per_index_CDF,
        SKP_Silk_LTP_per_index_CDF_offset, SKP_Silk_LTP_vq_ptrs_Q14,
    },
};

#[no_mangle]
pub fn skp_silk_decode_parameters(
    ps_dec: &mut SKP_Silk_decoder_state,
    ps_dec_ctrl: &mut SKP_Silk_decoder_control,
    q: &mut [i32],
    full_decoding: i32,
) {
    let mut ix;
    let mut p_nlsf0_q15 = [0; 16];
    let ps_r_c = &mut ps_dec.sRC;
    if ps_dec.nFramesDecoded == 0 {
        ix = SKP_Silk_range_decoder(
            ps_r_c,
            &SKP_SILK_SAMPLING_RATES_CDF,
            SKP_SILK_SAMPLING_RATES_OFFSET,
        );
        if ix < 0 || ix > 3 {
            ps_r_c.error = -7;
            return;
        }
        let fs_k_hz_dec = SKP_SILK_SAMPLING_RATES_TABLE[ix as usize];
        SKP_Silk_decoder_set_fs(ps_dec, fs_k_hz_dec);
    }
    let ps_r_c = &mut ps_dec.sRC;
    if ps_dec.nFramesDecoded == 0 {
        ix = SKP_Silk_range_decoder(
            ps_r_c,
            &SKP_SILK_TYPE_OFFSET_CDF,
            SKP_SILK_TYPE_OFFSET_CDF_OFFSET,
        );
    } else {
        ix = SKP_Silk_range_decoder(
            ps_r_c,
            &SKP_SILK_TYPE_OFFSET_JOINT_CDF[ps_dec.typeOffsetPrev as usize],
            SKP_SILK_TYPE_OFFSET_CDF_OFFSET,
        );
    }
    ps_dec_ctrl.sig_type = ix >> 1;
    ps_dec_ctrl.QuantOffsetType = ix & 1;
    ps_dec.typeOffsetPrev = ix;
    let mut gains_indices = [0; 4];
    if ps_dec.nFramesDecoded == 0 {
        gains_indices[0] = SKP_Silk_range_decoder(
            ps_r_c,
            &SKP_SILK_GAIN_CDF[ps_dec_ctrl.sig_type as usize],
            SKP_SILK_GAIN_CDF_OFFSET,
        );
    } else {
        gains_indices[0] = SKP_Silk_range_decoder(
            ps_r_c,
            &SKP_SILK_DELTA_GAIN_CDF,
            SKP_SILK_DELTA_GAIN_CDF_OFFSET,
        );
    }

    for i in 1..4 {
        gains_indices[i] = SKP_Silk_range_decoder(
            ps_r_c,
            &SKP_SILK_DELTA_GAIN_CDF,
            SKP_SILK_DELTA_GAIN_CDF_OFFSET,
        );
    }
    skp_silk_gains_dequant(
        &mut ps_dec_ctrl.Gains_Q16,
        &gains_indices,
        &mut ps_dec.LastGainIndex,
        ps_dec.nFramesDecoded,
    );
    let ps_nlsf_cb = ps_dec.psNLSF_CB[ps_dec_ctrl.sig_type as usize].unwrap();
    let mut nlsf_indices = [0; 10];
    skp_silk_range_decoder_multi(
        &mut nlsf_indices,
        ps_r_c,
        ps_nlsf_cb.start_ptr,
        ps_nlsf_cb.middle_ix,
        ps_nlsf_cb.n_stages as usize,
    );
    let mut p_nlsf_q15 = [0; 16];
    skp_silk_nlsf_msvq_decode(&mut p_nlsf_q15, ps_nlsf_cb, &nlsf_indices, ps_dec.LPC_order);
    ps_dec_ctrl.NLSFInterpCoef_Q2 = SKP_Silk_range_decoder(
        ps_r_c,
        &SKP_SILK_NLSF_INTERPOLATION_FACTOR_CDF,
        SKP_SILK_NLSF_INTERPOLATION_FACTOR_OFFSET,
    );
    if ps_dec.first_frame_after_reset == 1 {
        ps_dec_ctrl.NLSFInterpCoef_Q2 = 4;
    }
    if full_decoding != 0 {
        skp_silk_nlsf2a_stable(
            &mut ps_dec_ctrl.PredCoef_Q12[1],
            &p_nlsf_q15,
            ps_dec.LPC_order as usize,
        );
        if ps_dec_ctrl.NLSFInterpCoef_Q2 < 4 {
            for i in 0..ps_dec.LPC_order as usize {
                p_nlsf0_q15[i] = ps_dec.prevNLSF_Q15[i]
                    + (ps_dec_ctrl.NLSFInterpCoef_Q2 * (p_nlsf_q15[i] - ps_dec.prevNLSF_Q15[i])
                        >> 2);
            }
            skp_silk_nlsf2a_stable(
                &mut (ps_dec_ctrl.PredCoef_Q12[0]),
                &p_nlsf0_q15,
                ps_dec.LPC_order as usize,
            );
        } else {
            for i in 0..ps_dec.LPC_order as usize {
                ps_dec_ctrl.PredCoef_Q12[0][i] = ps_dec_ctrl.PredCoef_Q12[1][i];
            }
        }
    }
    for i in 0..ps_dec.LPC_order as usize {
        ps_dec.prevNLSF_Q15[i] = p_nlsf_q15[i];
    }
    if ps_dec.lossCnt != 0 {
        skp_silk_bwexpander(
            &mut (ps_dec_ctrl.PredCoef_Q12[0]),
            ps_dec.LPC_order as usize,
            63570,
        );
        skp_silk_bwexpander(
            &mut (ps_dec_ctrl.PredCoef_Q12[1]),
            ps_dec.LPC_order as usize,
            63570,
        );
    }
    let mut ixs = [0; 4];
    if ps_dec_ctrl.sig_type == 0 {
        if ps_dec.fs_kHz == 8 {
            ixs[0] = SKP_Silk_range_decoder(
                ps_r_c,
                &SKP_SILK_PITCH_LAG_NB_CDF,
                SKP_SILK_PITCH_LAG_NB_CDF_OFFSET,
            );
        } else if ps_dec.fs_kHz == 12 {
            ixs[0] = SKP_Silk_range_decoder(
                ps_r_c,
                &SKP_SILK_PITCH_LAG_MB_CDF,
                SKP_SILK_PITCH_LAG_MB_CDF_OFFSET,
            );
        } else if ps_dec.fs_kHz == 16 {
            ixs[0] = SKP_Silk_range_decoder(
                ps_r_c,
                &SKP_SILK_PITCH_LAG_WB_CDF,
                SKP_SILK_PITCH_LAG_WB_CDF_OFFSET,
            );
        } else {
            ixs[0] = SKP_Silk_range_decoder(
                ps_r_c,
                &SKP_SILK_PITCH_LAG_SWB_CDF,
                SKP_SILK_PITCH_LAG_SWB_CDF_OFFSET,
            );
        }
        if ps_dec.fs_kHz == 8 {
            ixs[1] = SKP_Silk_range_decoder(
                ps_r_c,
                &SKP_SILK_PITCH_CONTOUR_NB_CDF,
                SKP_SILK_PITCH_CONTOUR_NB_CDF_OFFSET,
            );
        } else {
            ixs[1] = SKP_Silk_range_decoder(
                ps_r_c,
                &SKP_SILK_PITCH_CONTOUR_CDF,
                SKP_SILK_PITCH_CONTOUR_CDF_OFFSET,
            );
        }
        skp_silk_decode_pitch(
            ixs[0] as usize,
            ixs[1] as usize,
            &mut ps_dec_ctrl.pitchL,
            ps_dec.fs_kHz,
        );
        ps_dec_ctrl.PERIndex = SKP_Silk_range_decoder(
            ps_r_c,
            &SKP_Silk_LTP_per_index_CDF,
            SKP_Silk_LTP_per_index_CDF_offset,
        );
        let cbk_ptr_q14 = SKP_Silk_LTP_vq_ptrs_Q14[ps_dec_ctrl.PERIndex as usize];
        for k in 0..4 {
            ix = SKP_Silk_range_decoder(
                ps_r_c,
                SKP_Silk_LTP_gain_CDF_ptrs[ps_dec_ctrl.PERIndex as usize],
                SKP_Silk_LTP_gain_CDF_offsets[ps_dec_ctrl.PERIndex as usize],
            );
            for i in 0..5 {
                ps_dec_ctrl.LTPCoef_Q14[k * 5 + i] = cbk_ptr_q14[ix as usize][i];
            }
        }
        ix = SKP_Silk_range_decoder(ps_r_c, &SKP_SILK_LTP_SCALE_CDF, SKP_SILK_LTP_SCALE_OFFSET);
        ps_dec_ctrl.LTP_scale_Q14 = SKP_SILK_LTP_SCALES_TABLE_Q14[ix as usize] as i32;
    } else {
        for i in 0..4 {
            ps_dec_ctrl.pitchL[i] = 0;
        }
        for i in 0..5 * 4 {
            ps_dec_ctrl.LTPCoef_Q14[i] = 0;
        }
        ps_dec_ctrl.PERIndex = 0;
        ps_dec_ctrl.LTP_scale_Q14 = 0;
    }
    ix = SKP_Silk_range_decoder(ps_r_c, &SKP_SILK_SEED_CDF, SKP_SILK_SEED_OFFSET);
    ps_dec_ctrl.Seed = ix;
    skp_silk_decode_pulses(ps_r_c, ps_dec_ctrl, q, ps_dec.frame_length);
    ps_dec.vadFlag =
        SKP_Silk_range_decoder(ps_r_c, &SKP_SILK_VAD_FLAG_CDF, SKP_SILK_VAD_FLAG_OFFSET);
    ps_dec.FrameTermination = SKP_Silk_range_decoder(
        ps_r_c,
        &SKP_SILK_FRAME_TERMINATION_CDF,
        SKP_SILK_FRAME_TERMINATION_OFFSET,
    );
    let mut n_bytes_used = 0;
    skp_silk_range_coder_get_length(ps_r_c, &mut n_bytes_used);
    ps_dec.nBytesLeft = ps_r_c.bufferLength - n_bytes_used;
    if ps_dec.nBytesLeft < 0 {
        ps_r_c.error = -6;
    }
    if ps_dec.nBytesLeft == 0 {
        skp_silk_range_coder_check_after_decoding(ps_r_c);
    }
}
