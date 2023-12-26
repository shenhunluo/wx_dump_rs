pub fn skp_silk_bwexpander_32(ar: &mut [i32; 16], d: usize, chirp_q16: i32) {
    let mut tmp_chirp_q16 = chirp_q16;
    println!("aaaaa");
    for i in 0..(d - 1) {
        ar[i] = (ar[i] >> 16) * tmp_chirp_q16 as i16 as i32
            + ((ar[i] & 0xffff) * tmp_chirp_q16 as i16 as i32 >> 16)
            + ar[i]
                * (if 16 == 1 {
                    (tmp_chirp_q16 >> 1) + (tmp_chirp_q16 & 1)
                } else {
                    (tmp_chirp_q16 >> 16 - 1) + 1 >> 1
                });
        tmp_chirp_q16 = (chirp_q16 >> 16) * tmp_chirp_q16 as i16 as i32
            + ((chirp_q16 & 0xffff) * tmp_chirp_q16 as i16 as i32 >> 16)
            + chirp_q16
                * (if 16 == 1 {
                    (tmp_chirp_q16 >> 1) + (tmp_chirp_q16 & 1)
                } else {
                    (tmp_chirp_q16 >> 16 - 1) + 1 >> 1
                });
    }
    ar[d - 1] = (ar[d - 1] >> 16) * tmp_chirp_q16 as i16 as i32
        + ((ar[d - 1] & 0xffff) * tmp_chirp_q16 as i16 as i32 >> 16)
        + ar[d - 1]
            * (if 16 == 1 {
                (tmp_chirp_q16 >> 1) + (tmp_chirp_q16 & 1)
            } else {
                (tmp_chirp_q16 >> 16 - 1) + 1 >> 1
            });
}
