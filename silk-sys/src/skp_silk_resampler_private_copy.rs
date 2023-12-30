use crate::skp_silk_resampler::SkpSilkResamplerStateStruct;

pub fn skp_silk_resampler_private_copy(
    _ss: &mut SkpSilkResamplerStateStruct,
    out: &mut [i16],
    in_0: &[i16],
    len: usize,
) {
    for i in 0..len {
        out[i] = in_0[i];
    }
}
