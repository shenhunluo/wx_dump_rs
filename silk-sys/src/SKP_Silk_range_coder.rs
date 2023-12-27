#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]

#[derive(Copy, Clone)]
pub struct SKP_Silk_range_coder_state {
    pub bufferLength: libc::c_int,
    pub bufferIx: libc::c_int,
    pub base_Q32: libc::c_uint,
    pub range_Q16: libc::c_uint,
    pub error: libc::c_int,
    pub buffer: [libc::c_uchar; 1024],
}

impl Default for SKP_Silk_range_coder_state {
    fn default() -> Self {
        Self { bufferLength: Default::default(), bufferIx: Default::default(), base_Q32: Default::default(), range_Q16: Default::default(), error: Default::default(), buffer: [0;1024] }
    }
}

extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
#[inline]
unsafe extern "C" fn SKP_Silk_CLZ16(mut in16: libc::c_short) -> libc::c_int {
    let mut out32: libc::c_int = 0 as libc::c_int;
    if in16 as libc::c_int == 0 as libc::c_int {
        return 16 as libc::c_int;
    }
    if in16 as libc::c_int & 0xff00 as libc::c_int != 0 {
        if in16 as libc::c_int & 0xf000 as libc::c_int != 0 {
            in16 = (in16 as libc::c_int >> 12 as libc::c_int) as libc::c_short;
        } else {
            out32 += 4 as libc::c_int;
            in16 = (in16 as libc::c_int >> 8 as libc::c_int) as libc::c_short;
        }
    } else if in16 as libc::c_int & 0xfff0 as libc::c_int != 0 {
        out32 += 8 as libc::c_int;
        in16 = (in16 as libc::c_int >> 4 as libc::c_int) as libc::c_short;
    } else {
        out32 += 12 as libc::c_int;
    }
    if in16 as libc::c_int & 0xc as libc::c_int != 0 {
        if in16 as libc::c_int & 0x8 as libc::c_int != 0 {
            return out32 + 0 as libc::c_int
        } else {
            return out32 + 1 as libc::c_int
        }
    } else if in16 as libc::c_int & 0xe as libc::c_int != 0 {
        return out32 + 2 as libc::c_int
    } else {
        return out32 + 3 as libc::c_int
    };
}
#[inline]
unsafe extern "C" fn SKP_Silk_CLZ32(mut in32: libc::c_int) -> libc::c_int {
    if in32 as libc::c_uint & 0xffff0000 as libc::c_uint != 0 {
        return SKP_Silk_CLZ16((in32 >> 16 as libc::c_int) as libc::c_short)
    } else {
        return SKP_Silk_CLZ16(in32 as libc::c_short) + 16 as libc::c_int
    };
}
#[no_mangle]
pub fn SKP_Silk_range_encoder(
    mut psRC: &mut SKP_Silk_range_coder_state,
    data: libc::c_int,
    mut prob: &[u16],
) {
    let mut low_Q16: libc::c_uint = 0;
    let mut high_Q16: libc::c_uint = 0;
    let mut base_tmp: libc::c_uint = 0;
    let mut range_Q32: libc::c_uint = 0;
    let mut base_Q32: libc::c_uint = psRC.base_Q32;
    let mut range_Q16: libc::c_uint = psRC.range_Q16;
    let mut bufferIx: libc::c_int = psRC.bufferIx;
    let mut buffer = &mut psRC.buffer;
    if (*psRC).error != 0 {
        return;
    }
    low_Q16 = prob[data as usize] as u32;
    high_Q16 = prob[(data + 1) as usize] as u32;
    base_tmp = base_Q32;
    base_Q32 = base_Q32.wrapping_add(range_Q16.wrapping_mul(low_Q16));
    range_Q32 = range_Q16.wrapping_mul(high_Q16.wrapping_sub(low_Q16));
    if base_Q32 < base_tmp {
        let mut bufferIx_tmp: libc::c_int = bufferIx;
        loop {
            bufferIx_tmp -= 1;
            let mut fresh0 = &mut buffer[bufferIx_tmp as usize];
            *fresh0 = fresh0.wrapping_add(1);
            if !(*fresh0 as libc::c_int == 0 as libc::c_int) {
                break;
            }
        }
    }
    if range_Q32 & 0xff000000 as libc::c_uint != 0 {
        range_Q16 = range_Q32 >> 16 as libc::c_int;
    } else {
        if range_Q32 & 0xffff0000 as libc::c_uint != 0 {
            range_Q16 = range_Q32 >> 8 as libc::c_int;
        } else {
            range_Q16 = range_Q32;
            if bufferIx >= (*psRC).bufferLength {
                (*psRC).error = -(1 as libc::c_int);
                return;
            }
            let fresh1 = bufferIx;
            bufferIx = bufferIx + 1;
            buffer[fresh1 as usize] = (base_Q32 >> 24 as libc::c_int) as libc::c_uchar;
            base_Q32 = base_Q32 << 8 as libc::c_int;
        }
        if bufferIx >= (*psRC).bufferLength {
            (*psRC).error = -(1 as libc::c_int);
            return;
        }
        let fresh2 = bufferIx;
        bufferIx = bufferIx + 1;
        buffer[fresh2 as usize] = (base_Q32 >> 24 as libc::c_int) as libc::c_uchar;
        base_Q32 = base_Q32 << 8 as libc::c_int;
    }
    psRC.base_Q32 = base_Q32;
    psRC.range_Q16 = range_Q16;
    psRC.bufferIx = bufferIx;
}
#[no_mangle]
pub unsafe fn SKP_Silk_range_encoder_multi(
    mut psRC: &mut SKP_Silk_range_coder_state,
    mut data: *const libc::c_int,
    prob: &[&[u16]],
    nSymbols: libc::c_int,
) {
    for k in 0..nSymbols as usize {
        SKP_Silk_range_encoder(psRC, *data.offset(k as isize), &prob[k]);
    }
}
#[no_mangle]
pub fn SKP_Silk_range_decoder(
    psRC: &mut SKP_Silk_range_coder_state,
    prob: &[u16],
    mut probIx: i32,
) -> i32 {
    let mut low_Q16 = 0;
    let mut high_Q16 = 0;
    let mut base_tmp = 0;
    let mut range_Q32 = 0;
    let mut base_Q32 = psRC.base_Q32;
    let mut range_Q16 = psRC.range_Q16;
    let mut bufferIx = psRC.bufferIx;
    let mut buffer = &psRC.buffer[4..];
    if psRC.error != 0 {
        return 0;
    }
    high_Q16 = prob[probIx as usize] as u32;
    base_tmp = range_Q16.wrapping_mul(high_Q16);
    if base_tmp > base_Q32 {
        loop {
            probIx -= 1;
            low_Q16 = prob[probIx as usize] as u32;
            base_tmp = range_Q16.wrapping_mul(low_Q16);
            if base_tmp <= base_Q32 {
                break;
            }
            high_Q16 = low_Q16;
            if high_Q16 == 0 {
                psRC.error = -2;
                return 0;
            }
        }
    } else {
        loop {
            low_Q16 = high_Q16;
            probIx += 1;
            high_Q16 = prob[probIx as usize] as u32;
            base_tmp = range_Q16.wrapping_mul(high_Q16);
            if base_tmp > base_Q32 {
                probIx -= 1;
                break;
            } else if high_Q16 == 0xffff {
                (*psRC).error = -2;
                return 0;
            }
        }
    }

    base_Q32 = base_Q32.wrapping_sub(range_Q16.wrapping_mul(low_Q16));
    range_Q32 = range_Q16.wrapping_mul(high_Q16.wrapping_sub(low_Q16));
    if range_Q32 & 0xff000000 != 0 {
        range_Q16 = range_Q32 >> 16;
    } else {
        if range_Q32 & 0xffff0000 != 0 {
            range_Q16 = range_Q32 >> 8;
            if base_Q32 >> 24 != 0 {
                (*psRC).error = -3;
                return 0;
            }
        } else {
            range_Q16 = range_Q32;
            if base_Q32 >> 16 != 0 {
                (*psRC).error = -3;
                return 0;
            }
            base_Q32 = base_Q32 << 8;
            if bufferIx < (*psRC).bufferLength {
                let fresh3 = bufferIx;
                bufferIx = bufferIx + 1;
                base_Q32 |= buffer[fresh3 as usize] as u32;
            }
        }
        base_Q32 = base_Q32 << 8;
        if bufferIx < (*psRC).bufferLength {
            let fresh4 = bufferIx;
            bufferIx = bufferIx + 1;
            base_Q32 |= buffer[fresh4 as usize] as u32;
        }
    }
    if range_Q16 == 0 {
        (*psRC).error = -4;
        return 0;
    }
    psRC.base_Q32 = base_Q32;
    psRC.range_Q16 = range_Q16;
    psRC.bufferIx = bufferIx;
    probIx
}
#[no_mangle]
pub unsafe fn SKP_Silk_range_decoder_multi(
    mut data: *mut libc::c_int,
    mut psRC: &mut SKP_Silk_range_coder_state,
    mut prob: &[&[u16]],
    mut probStartIx: &[i32],
    nSymbols: libc::c_int,
) {
    let mut k: libc::c_int = 0;
    k = 0 as libc::c_int;
    while k < nSymbols {
        *data.offset(k as isize) = SKP_Silk_range_decoder(
            psRC,
            &prob[k as usize],
            probStartIx[k as usize],
        );
        k += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_range_enc_init(
    mut psRC: *mut SKP_Silk_range_coder_state,
) {
    (*psRC).bufferLength = 1024 as libc::c_int;
    (*psRC).range_Q16 = 0xffff as libc::c_int as libc::c_uint;
    (*psRC).bufferIx = 0 as libc::c_int;
    (*psRC).base_Q32 = 0 as libc::c_int as libc::c_uint;
    (*psRC).error = 0 as libc::c_int;
}

pub fn SKP_Silk_range_dec_init(
    mut psRC: &mut SKP_Silk_range_coder_state,
    mut buffer: &[u8],
    bufferLength: i32,
) {
    if bufferLength > 1024 || bufferLength < 0 {
        (*psRC).error = -8;
        return;
    }
    for i in 0..bufferLength as usize {
        psRC.buffer[i] = buffer[i];
    }
    psRC.bufferLength = bufferLength;
    psRC.bufferIx = 0;
    psRC.base_Q32 = (buffer[0] as u32) << 24 | 
                    (buffer[1] as u32) << 16 | 
                    (buffer[2] as u32) << 8  | 
                    buffer[3] as u32
                    ;
    psRC.range_Q16 = 0xffff;
    psRC.error = 0;
}

#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_range_coder_get_length(
    mut psRC: *const SKP_Silk_range_coder_state,
    mut nBytes: *mut libc::c_int,
) -> libc::c_int {
    let mut nBits: libc::c_int = 0;
    nBits = ((*psRC).bufferIx << 3 as libc::c_int)
        + SKP_Silk_CLZ32(
            ((*psRC).range_Q16).wrapping_sub(1 as libc::c_int as libc::c_uint)
                as libc::c_int,
        ) - 14 as libc::c_int;
    *nBytes = nBits + 7 as libc::c_int >> 3 as libc::c_int;
    return nBits;
}
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_range_enc_wrap_up(
    mut psRC: *mut SKP_Silk_range_coder_state,
) {
    let mut bufferIx_tmp: libc::c_int = 0;
    let mut bits_to_store: libc::c_int = 0;
    let mut bits_in_stream: libc::c_int = 0;
    let mut nBytes: libc::c_int = 0;
    let mut mask: libc::c_int = 0;
    let mut base_Q24: libc::c_uint = 0;
    base_Q24 = (*psRC).base_Q32 >> 8 as libc::c_int;
    bits_in_stream = SKP_Silk_range_coder_get_length(psRC, &mut nBytes);
    bits_to_store = bits_in_stream - ((*psRC).bufferIx << 3 as libc::c_int);
    base_Q24 = base_Q24
        .wrapping_add(
            (0x800000 as libc::c_int >> bits_to_store - 1 as libc::c_int) as libc::c_uint,
        );
    base_Q24 &= (0xffffffff as libc::c_uint) << 24 as libc::c_int - bits_to_store;
    if base_Q24 & 0x1000000 as libc::c_int as libc::c_uint != 0 {
        bufferIx_tmp = (*psRC).bufferIx;
        loop {
            bufferIx_tmp -= 1;
            (*psRC)
                .buffer[bufferIx_tmp
                as usize] = ((*psRC).buffer[bufferIx_tmp as usize]).wrapping_add(1);
            if !((*psRC).buffer[bufferIx_tmp as usize] as libc::c_int
                == 0 as libc::c_int)
            {
                break;
            }
        }
    }
    if (*psRC).bufferIx < (*psRC).bufferLength {
        let fresh5 = (*psRC).bufferIx;
        (*psRC).bufferIx = (*psRC).bufferIx + 1;
        (*psRC)
            .buffer[fresh5 as usize] = (base_Q24 >> 16 as libc::c_int) as libc::c_uchar;
        if bits_to_store > 8 as libc::c_int {
            if (*psRC).bufferIx < (*psRC).bufferLength {
                let fresh6 = (*psRC).bufferIx;
                (*psRC).bufferIx = (*psRC).bufferIx + 1;
                (*psRC)
                    .buffer[fresh6
                    as usize] = (base_Q24 >> 8 as libc::c_int) as libc::c_uchar;
            }
        }
    }
    if bits_in_stream & 7 as libc::c_int != 0 {
        mask = 0xff as libc::c_int >> (bits_in_stream & 7 as libc::c_int);
        if (nBytes - 1 as libc::c_int) < (*psRC).bufferLength {
            (*psRC)
                .buffer[(nBytes - 1 as libc::c_int)
                as usize] = ((*psRC).buffer[(nBytes - 1 as libc::c_int) as usize]
                as libc::c_int | mask) as libc::c_uchar;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_range_coder_check_after_decoding(
    mut psRC: *mut SKP_Silk_range_coder_state,
) {
    let mut bits_in_stream: libc::c_int = 0;
    let mut nBytes: libc::c_int = 0;
    let mut mask: libc::c_int = 0;
    bits_in_stream = SKP_Silk_range_coder_get_length(psRC, &mut nBytes);
    if nBytes - 1 as libc::c_int >= (*psRC).bufferLength {
        (*psRC).error = -(5 as libc::c_int);
        return;
    }
    if bits_in_stream & 7 as libc::c_int != 0 {
        mask = 0xff as libc::c_int >> (bits_in_stream & 7 as libc::c_int);
        if (*psRC).buffer[(nBytes - 1 as libc::c_int) as usize] as libc::c_int & mask
            != mask
        {
            (*psRC).error = -(5 as libc::c_int);
            return;
        }
    }
}
