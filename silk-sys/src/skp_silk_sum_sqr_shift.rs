use crate::{i16_to_i32, skp_s_mla_b_b_ovflw, skp_s_mla_t_t_ovflw, skp_s_mul_b_b};

pub fn skp_silk_sum_sqr_shift(energy: &mut i32, shift_ptr: &mut i32, x: &[i16], mut len: usize) {
    let (mut nrg, mut i) = if x.as_ptr() as i32 & 2 != 0 {
        (skp_s_mul_b_b!(x[0], x[0]), 1)
    } else {
        (0, 0)
    };
    let mut shift = 0;
    len -= 1;
    while i < len {
        let in32 = i16_to_i32!(x[i], x[i + 1]);
        nrg = skp_s_mla_b_b_ovflw!(nrg, in32, in32);
        nrg = skp_s_mla_t_t_ovflw!(nrg, in32, in32);
        i += 2;
        if !(nrg < 0) {
            continue;
        }
        nrg = (nrg as u32 >> 2) as i32;
        shift = 2;
        break;
    }
    while i < len {
        let in32 = i16_to_i32!(x[i], x[i + 1]);
        let nrg_tmp = skp_s_mul_b_b!(in32, in32);
        let nrg_tmp = skp_s_mla_t_t_ovflw!(nrg_tmp, in32, in32);
        nrg = (nrg as u32).wrapping_add(nrg_tmp as u32 >> shift) as i32;
        if nrg < 0 {
            nrg = (nrg as u32 >> 2) as i32;
            shift += 2;
        }
        i += 2;
    }
    if i == len {
        let nrg_tmp = skp_s_mul_b_b!(x[i], x[i]);
        nrg = nrg + (nrg_tmp >> shift);
    }
    if nrg as u32 & 0xc0000000 as u32 != 0 {
        nrg = (nrg as u32 >> 2) as i32;
        shift += 2;
    }
    *shift_ptr = shift;
    *energy = nrg;
}
