#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
use crate::{SKP_Silk_resampler_private_ARMA4::SKP_Silk_resampler_private_ARMA4, SKP_Silk_resampler_rom::SKP_Silk_resampler_frac_FIR_144};
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _SKP_Silk_resampler_state_struct {
    pub sIIR: [libc::c_int; 6],
    pub sFIR: [libc::c_int; 16],
    pub sDown2: [libc::c_int; 2],
    pub resampler_function: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut libc::c_short,
            *const libc::c_short,
            libc::c_int,
        ) -> (),
    >,
    pub up2_function: Option::<
        unsafe extern "C" fn(
            *mut libc::c_int,
            *mut libc::c_short,
            *const libc::c_short,
            libc::c_int,
        ) -> (),
    >,
    pub batchSize: libc::c_int,
    pub invRatio_Q16: libc::c_int,
    pub FIR_Fracs: libc::c_int,
    pub input2x: libc::c_int,
    pub Coefs: *const libc::c_short,
    pub sDownPre: [libc::c_int; 2],
    pub sUpPost: [libc::c_int; 2],
    pub down_pre_function: Option::<
        unsafe extern "C" fn(
            *mut libc::c_int,
            *mut libc::c_short,
            *const libc::c_short,
            libc::c_int,
        ) -> (),
    >,
    pub up_post_function: Option::<
        unsafe extern "C" fn(
            *mut libc::c_int,
            *mut libc::c_short,
            *const libc::c_short,
            libc::c_int,
        ) -> (),
    >,
    pub batchSizePrePost: libc::c_int,
    pub ratio_Q16: libc::c_int,
    pub nPreDownsamplers: libc::c_int,
    pub nPostUpsamplers: libc::c_int,
    pub magic_number: libc::c_int,
}
pub type SKP_Silk_resampler_state_struct = _SKP_Silk_resampler_state_struct;
#[inline]
unsafe extern "C" fn SKP_Silk_resampler_private_IIR_FIR_INTERPOL(
    mut out: *mut libc::c_short,
    mut buf: *mut libc::c_short,
    mut max_index_Q16: libc::c_int,
    mut index_increment_Q16: libc::c_int,
) -> *mut libc::c_short {
    let mut index_Q16: libc::c_int = 0;
    let mut res_Q15: libc::c_int = 0;
    let mut buf_ptr: *mut libc::c_short = 0 as *mut libc::c_short;
    let mut table_index: libc::c_int = 0;
    index_Q16 = 0 as libc::c_int;
    while index_Q16 < max_index_Q16 {
        table_index = ((index_Q16 & 0xffff as libc::c_int) >> 16 as libc::c_int)
            * 144 as libc::c_int as libc::c_short as libc::c_int
            + ((index_Q16 & 0xffff as libc::c_int & 0xffff as libc::c_int)
                * 144 as libc::c_int as libc::c_short as libc::c_int
                >> 16 as libc::c_int);
        buf_ptr = &mut *buf.offset((index_Q16 >> 16 as libc::c_int) as isize)
            as *mut libc::c_short;
        res_Q15 = *buf_ptr.offset(0 as libc::c_int as isize) as libc::c_int
            * SKP_Silk_resampler_frac_FIR_144[table_index
                as usize][0 as libc::c_int as usize] as libc::c_int;
        res_Q15 = res_Q15
            + *buf_ptr.offset(1 as libc::c_int as isize) as libc::c_int
                * SKP_Silk_resampler_frac_FIR_144[table_index
                    as usize][1 as libc::c_int as usize] as libc::c_int;
        res_Q15 = res_Q15
            + *buf_ptr.offset(2 as libc::c_int as isize) as libc::c_int
                * SKP_Silk_resampler_frac_FIR_144[table_index
                    as usize][2 as libc::c_int as usize] as libc::c_int;
        res_Q15 = res_Q15
            + *buf_ptr.offset(3 as libc::c_int as isize) as libc::c_int
                * SKP_Silk_resampler_frac_FIR_144[(143 as libc::c_int - table_index)
                    as usize][2 as libc::c_int as usize] as libc::c_int;
        res_Q15 = res_Q15
            + *buf_ptr.offset(4 as libc::c_int as isize) as libc::c_int
                * SKP_Silk_resampler_frac_FIR_144[(143 as libc::c_int - table_index)
                    as usize][1 as libc::c_int as usize] as libc::c_int;
        res_Q15 = res_Q15
            + *buf_ptr.offset(5 as libc::c_int as isize) as libc::c_int
                * SKP_Silk_resampler_frac_FIR_144[(143 as libc::c_int - table_index)
                    as usize][0 as libc::c_int as usize] as libc::c_int;
        let fresh0 = out;
        out = out.offset(1);
        *fresh0 = (if (if 15 as libc::c_int == 1 as libc::c_int {
            (res_Q15 >> 1 as libc::c_int) + (res_Q15 & 1 as libc::c_int)
        } else {
            (res_Q15 >> 15 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                >> 1 as libc::c_int
        }) > 0x7fff as libc::c_int
        {
            0x7fff as libc::c_int
        } else if (if 15 as libc::c_int == 1 as libc::c_int {
            (res_Q15 >> 1 as libc::c_int) + (res_Q15 & 1 as libc::c_int)
        } else {
            (res_Q15 >> 15 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                >> 1 as libc::c_int
        }) < 0x8000 as libc::c_int as libc::c_short as libc::c_int
        {
            0x8000 as libc::c_int as libc::c_short as libc::c_int
        } else if 15 as libc::c_int == 1 as libc::c_int {
            (res_Q15 >> 1 as libc::c_int) + (res_Q15 & 1 as libc::c_int)
        } else {
            (res_Q15 >> 15 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                >> 1 as libc::c_int
        }) as libc::c_short;
        index_Q16 += index_increment_Q16;
    }
    return out;
}
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_resampler_private_IIR_FIR(
    mut SS: *mut libc::c_void,
    mut out: *mut libc::c_short,
    mut in_0: *const libc::c_short,
    mut inLen: libc::c_int,
) {
    let mut S: *mut SKP_Silk_resampler_state_struct = SS
        as *mut SKP_Silk_resampler_state_struct;
    let mut nSamplesIn: libc::c_int = 0;
    let mut max_index_Q16: libc::c_int = 0;
    let mut index_increment_Q16: libc::c_int = 0;
    let mut buf: [libc::c_short; 966] = [0; 966];
    memcpy(
        buf.as_mut_ptr() as *mut libc::c_void,
        ((*S).sFIR).as_mut_ptr() as *const libc::c_void,
        (6 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    index_increment_Q16 = (*S).invRatio_Q16;
    loop {
        nSamplesIn = if inLen < (*S).batchSize { inLen } else { (*S).batchSize };
        if (*S).input2x == 1 as libc::c_int {
            ((*S).up2_function)
                .expect(
                    "non-null function pointer",
                )(
                ((*S).sIIR).as_mut_ptr(),
                &mut *buf.as_mut_ptr().offset(6 as libc::c_int as isize),
                in_0,
                nSamplesIn,
            );
        } else {
            SKP_Silk_resampler_private_ARMA4(
                ((*S).sIIR).as_mut_ptr(),
                &mut *buf.as_mut_ptr().offset(6 as libc::c_int as isize),
                in_0,
                (*S).Coefs,
                nSamplesIn,
            );
        }
        max_index_Q16 = nSamplesIn << 16 as libc::c_int + (*S).input2x;
        out = SKP_Silk_resampler_private_IIR_FIR_INTERPOL(
            out,
            buf.as_mut_ptr(),
            max_index_Q16,
            index_increment_Q16,
        );
        in_0 = in_0.offset(nSamplesIn as isize);
        inLen -= nSamplesIn;
        if !(inLen > 0 as libc::c_int) {
            break;
        }
        memcpy(
            buf.as_mut_ptr() as *mut libc::c_void,
            &mut *buf.as_mut_ptr().offset((nSamplesIn << (*S).input2x) as isize)
                as *mut libc::c_short as *const libc::c_void,
            (6 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
        );
    }
    memcpy(
        ((*S).sFIR).as_mut_ptr() as *mut libc::c_void,
        &mut *buf.as_mut_ptr().offset((nSamplesIn << (*S).input2x) as isize)
            as *mut libc::c_short as *const libc::c_void,
        (6 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
}
