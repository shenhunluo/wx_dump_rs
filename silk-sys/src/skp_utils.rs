fn skp_silk_clz16(mut in16: i16) -> i32 {
    let mut out32 = 0;
    if in16 == 0 {
        return 16;
    }
    if in16 as i32 & 0xff00 != 0 {
        if in16 as i32 & 0xf000 != 0 {
            in16 = in16 >> 12;
        } else {
            out32 += 4;
            in16 = in16 >> 8;
        }
    } else if in16 as i32 & 0xfff0 != 0 {
        out32 += 8;
        in16 = in16 >> 4;
    } else {
        out32 += 12;
    }
    if in16 & 0xc != 0 {
        if in16 & 0x8 != 0 {
            out32 + 0
        } else {
            out32 + 1
        }
    } else if in16 & 0xe != 0 {
        out32 + 2
    } else {
        out32 + 3
    }
}

pub fn skp_silk_clz32(in32: i32) -> i32 {
    if in32 as u32 & 0xffff0000 != 0 {
        skp_silk_clz16((in32 >> 16) as i16)
    } else {
        skp_silk_clz16(in32 as i16) + 16 as i32
    }
}
