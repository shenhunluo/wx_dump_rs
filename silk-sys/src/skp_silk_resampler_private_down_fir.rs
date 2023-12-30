use crate::skp_silk_resampler_down2::skp_silk_resampler_down2;
use crate::skp_silk_resampler_private_ar2::skp_silk_resampler_private_ar2;
use crate::skp_utils::RESAMPLER_DOWN_ORDER_FIR;
use crate::SKP_Silk_resampler::SKP_Silk_resampler_state_struct;
use crate::{skp_r_shift, skp_r_shift_round, skp_s_mla_w_b, skp_s_mul_w_b, skp_sat_16};
#[inline]
fn skp_silk_resampler_private_down_fir_interpol0<'a>(
    mut out: &'a mut [i16],
    buf2: &mut [i32],
    fir_coefs: &[i16],
    max_index_q16: usize,
    index_increment_q16: usize,
) -> &'a mut [i16] {
    for index_q16 in (0..max_index_q16).step_by(index_increment_q16) {
        let buf_ptr = &buf2[skp_r_shift!(index_q16, 16)..];
        let mut res_q6 = skp_s_mul_w_b!(buf_ptr[0] + buf_ptr[11], fir_coefs[0]);
        res_q6 = skp_s_mla_w_b!(res_q6, buf_ptr[1] + buf_ptr[10], fir_coefs[1]);
        res_q6 = skp_s_mla_w_b!(res_q6, buf_ptr[2] + buf_ptr[9], fir_coefs[2]);
        res_q6 = skp_s_mla_w_b!(res_q6, buf_ptr[3] + buf_ptr[8], fir_coefs[3]);
        res_q6 = skp_s_mla_w_b!(res_q6, buf_ptr[4] + buf_ptr[7], fir_coefs[4]);
        res_q6 = skp_s_mla_w_b!(res_q6, buf_ptr[5] + buf_ptr[6], fir_coefs[5]);
        out[0] = skp_sat_16!(skp_r_shift_round!(res_q6, 6), i32) as i16;
        out = &mut out[1..];
    }
    return out;
}
#[inline]
fn skp_silk_resampler_private_down_fir_interpol1<'a>(
    mut out: &'a mut [i16],
    buf2: &mut [i32],
    index_q16: &[i16],
    max_index_q16: usize,
    index_increment_q16: usize,
    fir_fracs: usize,
) -> &'a mut [i16] {
    for index_Q16 in (0..max_index_q16).step_by(index_increment_q16) {
        let buf_ptr = &buf2[skp_r_shift!(index_Q16, 16)..];
        let interpol_ind = skp_s_mul_w_b!(index_Q16 as i32 & 0xFFFF, fir_fracs as i32);
        let interpol_ptr = &index_q16[(RESAMPLER_DOWN_ORDER_FIR / 2 * interpol_ind) as usize..];
        let mut res_q6 = skp_s_mul_w_b!(buf_ptr[0], interpol_ptr[0]);
        res_q6 = skp_s_mla_w_b!(res_q6, buf_ptr[1], interpol_ptr[1]);
        res_q6 = skp_s_mla_w_b!(res_q6, buf_ptr[2], interpol_ptr[2]);
        res_q6 = skp_s_mla_w_b!(res_q6, buf_ptr[3], interpol_ptr[3]);
        res_q6 = skp_s_mla_w_b!(res_q6, buf_ptr[4], interpol_ptr[4]);
        res_q6 = skp_s_mla_w_b!(res_q6, buf_ptr[5], interpol_ptr[5]);
        let interpol_ptr = &index_q16
            [(RESAMPLER_DOWN_ORDER_FIR / 2 * (fir_fracs as i32 - 1 - interpol_ind)) as usize..];
        res_q6 = skp_s_mla_w_b!(res_q6, buf_ptr[11], interpol_ptr[0]);
        res_q6 = skp_s_mla_w_b!(res_q6, buf_ptr[10], interpol_ptr[1]);
        res_q6 = skp_s_mla_w_b!(res_q6, buf_ptr[9], interpol_ptr[2]);
        res_q6 = skp_s_mla_w_b!(res_q6, buf_ptr[8], interpol_ptr[3]);
        res_q6 = skp_s_mla_w_b!(res_q6, buf_ptr[7], interpol_ptr[4]);
        res_q6 = skp_s_mla_w_b!(res_q6, buf_ptr[6], interpol_ptr[5]);

        out[0] = skp_sat_16!(skp_r_shift_round!(res_q6, 6), i32) as i16;
        out = &mut out[1..];
    }
    return out;
}
#[no_mangle]
pub fn skp_silk_resampler_private_down_fir(
    ss: &mut SKP_Silk_resampler_state_struct,
    mut out: &mut [i16],
    mut in_0: &[i16],
    mut len: usize,
) {
    let mut n_samples_in;
    let mut max_index_q16;
    let mut buf1 = [0; 240];
    let mut buf2 = [0; 492];
    for i in 0..12 {
        buf2[i] = ss.sFIR[i];
    }
    let index_q16 = &ss.Coefs.as_ref().unwrap()[2..];
    let index_increment_q16 = ss.invRatio_Q16 as usize;
    loop {
        n_samples_in = if len < ss.batchSize as usize {
            len
        } else {
            ss.batchSize as usize
        };
        if ss.input2x == 1 {
            skp_silk_resampler_down2(&mut ss.sDown2, &mut buf1, in_0, n_samples_in);
            n_samples_in = n_samples_in >> 1;
            skp_silk_resampler_private_ar2(
                &mut ss.sIIR,
                &mut buf2[12..],
                &buf1,
                ss.Coefs.unwrap(),
                n_samples_in,
            );
        } else {
            skp_silk_resampler_private_ar2(
                &mut ss.sIIR,
                &mut buf2[12..],
                in_0,
                ss.Coefs.unwrap(),
                n_samples_in,
            );
        }
        max_index_q16 = n_samples_in << 16;
        if ss.FIR_Fracs == 1 {
            out = skp_silk_resampler_private_down_fir_interpol0(
                out,
                &mut buf2,
                index_q16,
                max_index_q16,
                index_increment_q16,
            );
        } else {
            out = skp_silk_resampler_private_down_fir_interpol1(
                out,
                &mut buf2,
                index_q16,
                max_index_q16,
                index_increment_q16,
                ss.FIR_Fracs as usize,
            );
        }
        in_0 = &in_0[(n_samples_in << ss.input2x) as usize..];
        len -= n_samples_in << ss.input2x;
        if !(len > ss.input2x as usize) {
            break;
        }
        for i in 0..12 {
            buf2[i] = buf2[n_samples_in + i]
        }
    }
    for i in 0..12 {
        ss.sFIR[i] = buf2[n_samples_in + i];
    }
}
