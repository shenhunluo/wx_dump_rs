use super::skp_silk_decode_frame::skp_silk_decode_frame;
use super::skp_silk_decode_parameters::skp_silk_decode_parameters;
use super::skp_silk_resampler::{
    skp_silk_resampler, skp_silk_resampler_init, SkpSilkResamplerStateStruct,
};
use super::SKP_Silk_range_coder::{SKP_Silk_range_coder_state, SKP_Silk_range_dec_init};
use crate::skp_silk_cng::{skp_silk_cng_reset, SkpSilkCngStruct};
use crate::skp_silk_decoder_set_fs::skp_silk_decoder_set_fs;
use crate::skp_silk_nlsf_msvq_decode::SkpSilkNlsfCbStruct;
use crate::skp_silk_plc::{skp_silk_plc_reset, SkpSilkPlcStruct};

#[derive(Clone)]
pub struct SkpSilkDecoderStruct {
    pub s_r_c: SKP_Silk_range_coder_state,
    pub prev_inv_gain_q16: i32,
    pub s_ltp_q16: [i32; 960],
    pub s_lpc_q14: [i32; 136],
    pub exc_q10: [i32; 480],
    pub res_q10: [i32; 480],
    pub out_buf: [i16; 960],
    pub lag_prev: i32,
    pub last_gain_index: i32,
    pub last_gain_index_enh_layer: i32,
    pub type_offset_prev: i32,
    pub hp_state: [i32; 2],
    pub hp_a: Option<&'static [i16]>,
    pub hp_b: Option<&'static [i16]>,
    pub fs_k_hz: i32,
    pub prev_api_sample_rate: i32,
    pub frame_length: i32,
    pub subfr_length: i32,
    pub lpc_order: i32,
    pub prev_nlsf_q15: [i32; 16],
    pub first_frame_after_reset: i32,
    pub n_bytes_left: i32,
    pub n_frames_decoded: i32,
    pub n_frames_in_packet: i32,
    pub more_internal_decoder_frames: i32,
    pub frame_termination: i32,
    pub resampler_state: SkpSilkResamplerStateStruct,
    pub ps_nlsf_cb: [Option<&'static SkpSilkNlsfCbStruct>; 2],
    pub vad_flag: i32,
    pub no_fec_counter: i32,
    pub inband_fec_offset: i32,
    pub s_cng: SkpSilkCngStruct,
    pub loss_cnt: i32,
    pub prev_sig_type: i32,
    pub s_plc: SkpSilkPlcStruct,
}

impl Default for SkpSilkDecoderStruct {
    fn default() -> Self {
        Self {
            s_r_c: Default::default(),
            prev_inv_gain_q16: Default::default(),
            s_ltp_q16: [0; 960],
            s_lpc_q14: [0; 136],
            exc_q10: [0; 480],
            res_q10: [0; 480],
            out_buf: [0; 960],
            lag_prev: Default::default(),
            last_gain_index: Default::default(),
            last_gain_index_enh_layer: Default::default(),
            type_offset_prev: Default::default(),
            hp_state: Default::default(),
            hp_a: Default::default(),
            hp_b: None,
            fs_k_hz: Default::default(),
            prev_api_sample_rate: Default::default(),
            frame_length: Default::default(),
            subfr_length: Default::default(),
            lpc_order: Default::default(),
            prev_nlsf_q15: Default::default(),
            first_frame_after_reset: Default::default(),
            n_bytes_left: Default::default(),
            n_frames_decoded: Default::default(),
            n_frames_in_packet: Default::default(),
            more_internal_decoder_frames: Default::default(),
            frame_termination: Default::default(),
            resampler_state: Default::default(),
            ps_nlsf_cb: Default::default(),
            vad_flag: Default::default(),
            no_fec_counter: Default::default(),
            inband_fec_offset: Default::default(),
            s_cng: Default::default(),
            loss_cnt: Default::default(),
            prev_sig_type: Default::default(),
            s_plc: Default::default(),
        }
    }
}

impl SkpSilkDecoderStruct {
    pub fn init(&mut self) {
        skp_silk_decoder_set_fs(self, 24);
        self.first_frame_after_reset = 1;
        self.prev_inv_gain_q16 = 65536;
        skp_silk_cng_reset(self);
        skp_silk_plc_reset(self);
    }
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct SkpSilkSdkDecControlStruct {
    pub api_sample_rate: i32,
    pub frame_size: i32,
    pub frames_per_packet: i32,
    pub more_internal_decoder_frames: i32,
    pub in_band_fec_offset: i32,
}

impl SkpSilkSdkDecControlStruct {
    pub fn get_control_by_api_sample_rate(sample_rate: i32) -> Self {
        Self {
            api_sample_rate: sample_rate.into(),
            frame_size: 0,
            frames_per_packet: 1,
            more_internal_decoder_frames: 0,
            in_band_fec_offset: 0,
        }
    }
}

#[derive(Clone, Default)]
pub struct SkpSilkTocStruct {
    pub frames_in_packet: i32,
    pub fs_k_hz: i32,
    pub inband_lbrr: i32,
    pub corrupt: i32,
    pub vad_flags: [i32; 5],
    pub sig_type_flags: [i32; 5],
}

#[derive(Clone, Default)]
#[repr(C)]
pub struct SkpSilkDecoderControl {
    pub pitch_l: [i32; 4],
    pub gains_q16: [i32; 4],
    pub seed: i32,
    pub pred_coef_q12: [[i16; 16]; 2],
    pub ltp_coef_q14: [i16; 20],
    pub ltp_scale_q14: i32,
    pub per_index: i32,
    pub rate_level_index: i32,
    pub quant_offset_type: i32,
    pub sig_type: i32,
    pub nlsf_interp_coef_q2: i32,
}

#[no_mangle]
pub fn skp_silk_sdk_decode(
    dec_state: &mut SkpSilkDecoderStruct,
    dec_control: &mut SkpSilkSdkDecControlStruct,
    mut lost_flag: i32,
    in_data: &[u8],
    n_bytes_in: i32,
    samples_out: &mut [i16],
    n_samples_out: &mut usize,
) -> i32 {
    let mut ret = 0;
    let mut used_bytes = 0;
    let p_s_dec = dec_state;
    if p_s_dec.more_internal_decoder_frames == 0 {
        p_s_dec.n_frames_decoded = 0;
    }
    if p_s_dec.more_internal_decoder_frames == 0 && lost_flag == 0 && n_bytes_in > 1024 {
        lost_flag = 1;
        ret = -11;
    }
    let prev_fs_k_hz = p_s_dec.fs_k_hz;
    ret += skp_silk_decode_frame(
        p_s_dec,
        samples_out,
        n_samples_out,
        in_data,
        n_bytes_in,
        lost_flag,
        &mut used_bytes,
    );
    if used_bytes != 0 {
        if p_s_dec.n_bytes_left > 0
            && p_s_dec.frame_termination == 1
            && p_s_dec.n_frames_decoded < 5
        {
            p_s_dec.more_internal_decoder_frames = 1;
        } else {
            p_s_dec.more_internal_decoder_frames = 0;
            p_s_dec.n_frames_in_packet = p_s_dec.n_frames_decoded;
            if p_s_dec.vad_flag == 1 {
                if p_s_dec.frame_termination == 0 {
                    p_s_dec.no_fec_counter += 1;
                    p_s_dec.no_fec_counter;
                    if p_s_dec.no_fec_counter > 10 {
                        p_s_dec.inband_fec_offset = 0;
                    }
                } else if p_s_dec.frame_termination == 2 {
                    p_s_dec.inband_fec_offset = 1;
                    p_s_dec.no_fec_counter = 0;
                } else if p_s_dec.frame_termination == 3 {
                    p_s_dec.inband_fec_offset = 2;
                    p_s_dec.no_fec_counter = 0;
                }
            }
        }
    }
    if (48 * 1000) < dec_control.api_sample_rate || 8000 > dec_control.api_sample_rate {
        ret = -10;
        return ret;
    }
    if p_s_dec.fs_k_hz * 1000 != dec_control.api_sample_rate {
        let mut samples_out_tmp: [i16; 960] = [0; 960];
        for i in 0..*n_samples_out {
            samples_out_tmp[i] = samples_out[i];
        }
        for i in 0..samples_out.len() {
            samples_out[i] = 0;
        }
        if prev_fs_k_hz != p_s_dec.fs_k_hz
            || p_s_dec.prev_api_sample_rate != dec_control.api_sample_rate
        {
            ret = skp_silk_resampler_init(
                &mut p_s_dec.resampler_state,
                p_s_dec.fs_k_hz as i16 as i32 * 1000,
                dec_control.api_sample_rate,
            );
        }
        ret += skp_silk_resampler(
            &mut p_s_dec.resampler_state,
            samples_out,
            &samples_out_tmp,
            *n_samples_out,
        );
        *n_samples_out = *n_samples_out * dec_control.api_sample_rate as usize
            / (p_s_dec.fs_k_hz as usize * 1000);
    }
    p_s_dec.prev_api_sample_rate = dec_control.api_sample_rate;
    dec_control.frame_size = (dec_control.api_sample_rate / 50) as i16 as i32;
    dec_control.frames_per_packet = p_s_dec.n_frames_in_packet;
    dec_control.in_band_fec_offset = p_s_dec.inband_fec_offset;
    dec_control.more_internal_decoder_frames = p_s_dec.more_internal_decoder_frames;
    return ret;
}

pub fn skp_silk_sdk_search_for_lbrr(
    in_data: &[u8],
    n_bytes_in: i32,
    lost_offset: i32,
    lbrr_data: &mut [u8],
    n_lbrr_bytes: &mut i16,
) {
    let mut s_dec = SkpSilkDecoderStruct::default();
    let mut s_dec_ctrl: SkpSilkDecoderControl = SkpSilkDecoderControl::default();
    let mut temp_q = [0; 480];
    if lost_offset < 1 || lost_offset > 2 {
        *n_lbrr_bytes = 0;
        return;
    }
    s_dec.n_frames_decoded = 0;
    s_dec.fs_k_hz = 0;
    s_dec.loss_cnt = 0;
    for i in 0..16 {
        s_dec.prev_nlsf_q15[i] = 0;
    }
    SKP_Silk_range_dec_init(&mut s_dec.s_r_c, in_data, n_bytes_in);
    loop {
        skp_silk_decode_parameters(&mut s_dec, &mut s_dec_ctrl, &mut temp_q, 0);
        if s_dec.s_r_c.error != 0 {
            *n_lbrr_bytes = 0;
            return;
        }
        if s_dec.frame_termination - 1 & lost_offset != 0
            && s_dec.frame_termination > 0
            && s_dec.n_bytes_left >= 0
        {
            *n_lbrr_bytes = s_dec.n_bytes_left as i16;
            for i in 0..s_dec.n_bytes_left as usize {
                lbrr_data[i] = in_data[(n_bytes_in - s_dec.n_bytes_left) as usize + i];
            }
            break;
        } else if s_dec.n_bytes_left > 0 && s_dec.frame_termination == 1 {
            s_dec.n_frames_decoded += 1;
            s_dec.n_frames_decoded;
        } else {
            *n_lbrr_bytes = 0;
            break;
        }
    }
}

pub fn skp_silk_sdk_get_toc(in_data: &[u8], n_bytes_in: i32, silk_toc: &mut SkpSilkTocStruct) {
    let mut s_dec = SkpSilkDecoderStruct::default();
    let mut s_dec_ctrl = SkpSilkDecoderControl::default();
    let mut temp_q = [0; 480];
    s_dec.n_frames_decoded = 0;
    s_dec.fs_k_hz = 0;
    SKP_Silk_range_dec_init(&mut s_dec.s_r_c, in_data, n_bytes_in);
    silk_toc.corrupt = 0;
    loop {
        skp_silk_decode_parameters(&mut s_dec, &mut s_dec_ctrl, &mut temp_q, 0);
        silk_toc.vad_flags[s_dec.n_frames_decoded as usize] = s_dec.vad_flag;
        silk_toc.sig_type_flags[s_dec.n_frames_decoded as usize] = s_dec_ctrl.sig_type;
        if s_dec.s_r_c.error != 0 {
            silk_toc.corrupt = 1;
            break;
        } else {
            if !(s_dec.n_bytes_left > 0 && s_dec.frame_termination == 1) {
                break;
            }
            s_dec.n_frames_decoded += 1;
            s_dec.n_frames_decoded;
        }
    }
    if silk_toc.corrupt != 0 || s_dec.frame_termination == 1 || s_dec.n_frames_in_packet > 5 {
        *silk_toc = SkpSilkTocStruct::default();
        silk_toc.corrupt = 1;
    } else {
        silk_toc.frames_in_packet = s_dec.n_frames_decoded + 1;
        silk_toc.fs_k_hz = s_dec.fs_k_hz;
        if s_dec.frame_termination == 0 {
            silk_toc.inband_lbrr = s_dec.frame_termination;
        } else {
            silk_toc.inband_lbrr = s_dec.frame_termination - 1;
        }
    };
}

pub fn skp_silk_sdk_get_version() -> &'static str {
    "1.0.9.6"
}
