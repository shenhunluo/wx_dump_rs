use crate::skp_r_shift_round;

pub fn skp_silk_bwexpander(ar: &mut [i16], d: usize, mut chirp_q16: i32) {
    let chirp_minus_one_q16 = chirp_q16 - 65536;
    for i in 0..d - 1 {
        ar[i] = skp_r_shift_round!(chirp_q16 * ar[i] as i32, 16) as i16;
        chirp_q16 += skp_r_shift_round!(chirp_q16 * chirp_minus_one_q16, 16);
    }
    ar[d - 1] = skp_r_shift_round!(chirp_q16 * ar[d - 1] as i32, 16) as i16;
}
