use crate::SKP_Silk_resampler::SKP_Silk_resampler_state_struct;

pub fn skp_silk_resampler_private_copy(
    _ss: &mut SKP_Silk_resampler_state_struct,
    out: &mut [i16],
    in_0: &[i16],
    len: usize,
) {
    for i in 0..len {
        out[i] = in_0[i];
    }
}
