use crate::skp_s_mul_w_w;

pub fn skp_silk_bwexpander_32(ar: &mut [i32; 16], d: usize, chirp_q16: i32) {
    let mut tmp_chirp_q16 = chirp_q16;
    for i in 0..(d - 1) {
        ar[i] = skp_s_mul_w_w!(ar[i], tmp_chirp_q16);
        tmp_chirp_q16 = skp_s_mul_w_w!(chirp_q16, tmp_chirp_q16);
    }
    ar[d - 1] = skp_s_mul_w_w!(ar[d - 1], tmp_chirp_q16);
}
