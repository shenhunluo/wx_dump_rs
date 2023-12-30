use crate::skp_silk_resampler_down2::skp_silk_resampler_down2;
use crate::skp_silk_resampler_private_copy::skp_silk_resampler_private_copy;
use crate::skp_silk_resampler_private_down4::skp_silk_resampler_private_down4;
use crate::skp_silk_resampler_private_down_fir::skp_silk_resampler_private_down_fir;
use crate::skp_silk_resampler_private_iir_fir::skp_silk_resampler_private_iir_fir;
use crate::skp_silk_resampler_private_up2_hq::{
    skp_silk_resampler_private_up2_hq, skp_silk_resampler_private_up2_hq_wrapper,
};
use crate::skp_silk_resampler_private_up4::skp_silk_resampler_private_up4;
use crate::skp_silk_resampler_rom::{
    SKP_SILK_RESAMPLER_120_441_ARMA4_COEFS, SKP_SILK_RESAMPLER_160_441_ARMA4_COEFS,
    SKP_SILK_RESAMPLER_1_2_COEFS, SKP_SILK_RESAMPLER_1_3_COEFS,
    SKP_SILK_RESAMPLER_240_441_ARMA4_COEFS, SKP_SILK_RESAMPLER_2_3_COEFS,
    SKP_SILK_RESAMPLER_320_441_ARMA4_COEFS, SKP_SILK_RESAMPLER_3_4_COEFS,
    SKP_SILK_RESAMPLER_3_8_COEFS, SKP_SILK_RESAMPLER_80_441_ARMA4_COEFS,
};
use crate::skp_silk_resampler_up2::skp_silk_resampler_up2;
use crate::{skp_l_shift, skp_s_mul_w_w};

#[derive(Clone)]
pub struct SkpSilkResamplerStateStruct {
    pub s_iir: [i32; 6],
    pub s_fir: [i32; 16],
    pub s_down2: [i32; 2],
    pub resampler_function:
        Option<fn(&mut SkpSilkResamplerStateStruct, &mut [i16], &[i16], usize) -> ()>,
    pub up2_function: Option<fn(&mut [i32], &mut [i16], &[i16], usize) -> ()>,
    pub batch_size: i32,
    pub inv_ratio_q16: i32,
    pub fir_fracs: i32,
    pub input2x: i32,
    pub coefs: Option<&'static [i16]>,
    pub s_down_pre: [i32; 2],
    pub s_up_post: [i32; 2],
    pub down_pre_function: Option<fn(&mut [i32], &mut [i16], &[i16], usize) -> ()>,
    pub up_post_function: Option<fn(&mut [i32], &mut [i16], &[i16], usize) -> ()>,
    pub batch_size_pre_post: i32,
    pub ratio_q16: i32,
    pub n_pre_down_samplers: i32,
    pub n_post_up_samplers: i32,
    pub magic_number: i32,
}

impl Default for SkpSilkResamplerStateStruct {
    fn default() -> Self {
        Self {
            s_iir: Default::default(),
            s_fir: Default::default(),
            s_down2: Default::default(),
            resampler_function: Default::default(),
            up2_function: Default::default(),
            batch_size: Default::default(),
            inv_ratio_q16: Default::default(),
            fir_fracs: Default::default(),
            input2x: Default::default(),
            coefs: Default::default(),
            s_down_pre: Default::default(),
            s_up_post: Default::default(),
            down_pre_function: Default::default(),
            up_post_function: Default::default(),
            batch_size_pre_post: Default::default(),
            ratio_q16: Default::default(),
            n_pre_down_samplers: Default::default(),
            n_post_up_samplers: Default::default(),
            magic_number: Default::default(),
        }
    }
}

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b > 0 {
        let tmp = a - b * (a / b);
        a = b;
        b = tmp;
    }
    return a;
}

pub fn skp_silk_resampler_init(
    s: &mut SkpSilkResamplerStateStruct,
    mut fs_hz_in: i32,
    mut fs_hz_out: i32,
) -> i32 {
    let mut up2 = 0;
    let mut down2 = 0;
    *s = SkpSilkResamplerStateStruct::default();
    if fs_hz_in < 8000 || fs_hz_in > 192000 || fs_hz_out < 8000 || fs_hz_out > 192000 {
        return -1;
    }
    if fs_hz_in > 96000 {
        s.n_pre_down_samplers = 2;
        s.down_pre_function = Some(skp_silk_resampler_private_down4);
    } else if fs_hz_in > 48000 {
        s.n_pre_down_samplers = 1;
        s.down_pre_function = Some(skp_silk_resampler_down2);
    } else {
        s.n_pre_down_samplers = 0;
    }
    if fs_hz_out > 96000 {
        s.n_post_up_samplers = 2;
        s.up_post_function = Some(skp_silk_resampler_private_up4);
    } else if fs_hz_out > 48000 {
        s.n_post_up_samplers = 1;
        s.up_post_function = Some(skp_silk_resampler_up2);
    } else {
        s.n_post_up_samplers = 0;
    }
    if s.n_pre_down_samplers + s.n_post_up_samplers > 0 {
        s.ratio_q16 = (fs_hz_out << 13) / fs_hz_in << 3;
        while skp_s_mul_w_w!(s.ratio_q16, fs_hz_in) < fs_hz_out {
            s.ratio_q16 += 1;
        }
        s.batch_size_pre_post = fs_hz_in / 100;
        fs_hz_in = fs_hz_in >> s.n_pre_down_samplers;
        fs_hz_out = fs_hz_out >> s.n_post_up_samplers;
    }
    s.batch_size = fs_hz_in / 100;
    if s.batch_size * 100 != fs_hz_in || fs_hz_in % 100 != 0 {
        let cycle_len = fs_hz_in / gcd(fs_hz_in, fs_hz_out);
        let cycles_per_batch = 480 / cycle_len;
        if cycles_per_batch == 0 {
            s.batch_size = 480;
        } else {
            s.batch_size = cycles_per_batch * cycle_len;
        }
    }
    if fs_hz_out > fs_hz_in {
        if fs_hz_out == fs_hz_in * 2 {
            s.resampler_function = Some(skp_silk_resampler_private_up2_hq_wrapper);
        } else {
            s.resampler_function = Some(skp_silk_resampler_private_iir_fir);
            up2 = 1;
            if fs_hz_in > 24000 {
                s.up2_function = Some(skp_silk_resampler_up2);
            } else {
                s.up2_function = Some(skp_silk_resampler_private_up2_hq);
            }
        }
    } else if fs_hz_out < fs_hz_in {
        if fs_hz_out * 4 == fs_hz_in * 3 {
            s.fir_fracs = 3;
            s.coefs = Some(&SKP_SILK_RESAMPLER_3_4_COEFS);
            s.resampler_function = Some(skp_silk_resampler_private_down_fir);
        } else if fs_hz_out * 3 == fs_hz_in * 2 {
            s.fir_fracs = 2;
            s.coefs = Some(&SKP_SILK_RESAMPLER_2_3_COEFS);
            s.resampler_function = Some(skp_silk_resampler_private_down_fir);
        } else if fs_hz_out * 2 == fs_hz_in {
            s.fir_fracs = 1;
            s.coefs = Some(&SKP_SILK_RESAMPLER_1_2_COEFS);
            s.resampler_function = Some(skp_silk_resampler_private_down_fir);
        } else if fs_hz_out * 8 == fs_hz_in * 3 {
            s.fir_fracs = 3;
            s.coefs = Some(&SKP_SILK_RESAMPLER_3_8_COEFS);
            s.resampler_function = Some(skp_silk_resampler_private_down_fir);
        } else if fs_hz_out * 3 == fs_hz_in {
            s.fir_fracs = 1;
            s.coefs = Some(&SKP_SILK_RESAMPLER_1_3_COEFS);
            s.resampler_function = Some(skp_silk_resampler_private_down_fir);
        } else if fs_hz_out * 4 == fs_hz_in {
            s.fir_fracs = 1;
            down2 = 1;
            s.coefs = Some(&SKP_SILK_RESAMPLER_1_2_COEFS);
            s.resampler_function = Some(skp_silk_resampler_private_down_fir);
        } else if fs_hz_out * 6 == fs_hz_in {
            s.fir_fracs = 1;
            down2 = 1;
            s.coefs = Some(&SKP_SILK_RESAMPLER_1_3_COEFS);
            s.resampler_function = Some(skp_silk_resampler_private_down_fir);
        } else if fs_hz_out * 441 == fs_hz_in * 80 {
            s.coefs = Some(&SKP_SILK_RESAMPLER_80_441_ARMA4_COEFS);
            s.resampler_function = Some(skp_silk_resampler_private_iir_fir);
        } else if fs_hz_out * 441 == fs_hz_in * 120 {
            s.coefs = Some(&SKP_SILK_RESAMPLER_120_441_ARMA4_COEFS);
            s.resampler_function = Some(skp_silk_resampler_private_iir_fir);
        } else if fs_hz_out * 441 == fs_hz_in * 160 {
            s.coefs = Some(&SKP_SILK_RESAMPLER_160_441_ARMA4_COEFS);
            s.resampler_function = Some(skp_silk_resampler_private_iir_fir);
        } else if fs_hz_out * 441 == fs_hz_in * 240 {
            s.coefs = Some(&SKP_SILK_RESAMPLER_240_441_ARMA4_COEFS);
            s.resampler_function = Some(skp_silk_resampler_private_iir_fir);
        } else if fs_hz_out * 441 == fs_hz_in * 320 {
            s.coefs = Some(&SKP_SILK_RESAMPLER_320_441_ARMA4_COEFS);
            s.resampler_function = Some(skp_silk_resampler_private_iir_fir);
        } else {
            s.resampler_function = Some(skp_silk_resampler_private_iir_fir);
            up2 = 1;
            if fs_hz_in > 24000 {
                s.up2_function = Some(skp_silk_resampler_up2);
            } else {
                s.up2_function = Some(skp_silk_resampler_private_up2_hq);
            }
        }
    } else {
        s.resampler_function = Some(skp_silk_resampler_private_copy);
    }
    s.input2x = up2 | down2;
    s.inv_ratio_q16 = (fs_hz_in << 14 + up2 - down2) / fs_hz_out << 2;
    while skp_s_mul_w_w!(s.inv_ratio_q16, skp_l_shift!(fs_hz_out, down2))
        < skp_l_shift!(fs_hz_in, up2)
    {
        s.inv_ratio_q16 += 1;
    }
    s.magic_number = 123456789;
    return 0;
}

pub fn skp_silk_resampler(
    s: &mut SkpSilkResamplerStateStruct,
    mut out: &mut [i16],
    mut in_0: &[i16],
    mut in_len: usize,
) -> i32 {
    if s.magic_number != 123456789 {
        return -(1);
    }
    if s.n_pre_down_samplers + s.n_post_up_samplers > 0 {
        let mut in_buf: [i16; 480] = [0; 480];
        let mut out_buf: [i16; 480] = [0; 480];
        while in_len > 0 {
            let n_samples_in = if in_len < s.batch_size_pre_post as usize {
                in_len
            } else {
                s.batch_size_pre_post as usize
            };
            let n_samples_out = (s.ratio_q16 >> 16) * n_samples_in as i16 as i32
                + ((s.ratio_q16 & 0xffff) * n_samples_in as i16 as i32 >> 16);
            if s.n_pre_down_samplers > 0 {
                (s.down_pre_function).expect("non-null function pointer")(
                    &mut s.s_down_pre,
                    &mut in_buf,
                    in_0,
                    n_samples_in,
                );
                if s.n_post_up_samplers > 0 {
                    (s.resampler_function).expect("non-null function pointer")(
                        s,
                        &mut out_buf,
                        &in_buf,
                        n_samples_in >> s.n_pre_down_samplers,
                    );
                    (s.up_post_function).expect("non-null function pointer")(
                        &mut s.s_up_post,
                        out,
                        &out_buf,
                        (n_samples_out >> s.n_post_up_samplers) as usize,
                    );
                } else {
                    (s.resampler_function).expect("non-null function pointer")(
                        s,
                        out,
                        &in_buf,
                        n_samples_in >> s.n_pre_down_samplers,
                    );
                }
            } else {
                (s.resampler_function).expect("non-null function pointer")(
                    s,
                    &mut out_buf,
                    &in_0,
                    (n_samples_in >> s.n_pre_down_samplers) as usize,
                );
                (s.up_post_function).expect("non-null function pointer")(
                    &mut s.s_up_post,
                    out,
                    &out_buf,
                    (n_samples_out >> s.n_post_up_samplers) as usize,
                );
            }
            in_0 = &in_0[n_samples_in as usize..];
            out = &mut out[n_samples_out as usize..];
            in_len -= n_samples_in;
        }
    } else {
        (s.resampler_function).expect("non-null function pointer")(s, out, in_0, in_len as usize);
    }
    return 0;
}
