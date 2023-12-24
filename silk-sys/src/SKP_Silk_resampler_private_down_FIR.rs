#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
use crate::SKP_Silk_resampler_down2::SKP_Silk_resampler_down2;
use crate::SKP_Silk_resampler_private_AR2::SKP_Silk_resampler_private_AR2;

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
unsafe extern "C" fn SKP_Silk_resampler_private_down_FIR_INTERPOL0(
    mut out: *mut libc::c_short,
    mut buf2: *mut libc::c_int,
    mut FIR_Coefs: *const libc::c_short,
    mut max_index_Q16: libc::c_int,
    mut index_increment_Q16: libc::c_int,
) -> *mut libc::c_short {
    let mut index_Q16: libc::c_int = 0;
    let mut res_Q6: libc::c_int = 0;
    let mut buf_ptr: *mut libc::c_int = 0 as *mut libc::c_int;
    index_Q16 = 0 as libc::c_int;
    while index_Q16 < max_index_Q16 {
        buf_ptr = buf2.offset((index_Q16 >> 16 as libc::c_int) as isize);
        res_Q6 = (*buf_ptr.offset(0 as libc::c_int as isize)
            + *buf_ptr.offset(11 as libc::c_int as isize) >> 16 as libc::c_int)
            * *FIR_Coefs.offset(0 as libc::c_int as isize) as libc::c_int
            + ((*buf_ptr.offset(0 as libc::c_int as isize)
                + *buf_ptr.offset(11 as libc::c_int as isize) & 0xffff as libc::c_int)
                * *FIR_Coefs.offset(0 as libc::c_int as isize) as libc::c_int
                >> 16 as libc::c_int);
        res_Q6 = res_Q6
            + ((*buf_ptr.offset(1 as libc::c_int as isize)
                + *buf_ptr.offset(10 as libc::c_int as isize) >> 16 as libc::c_int)
                * *FIR_Coefs.offset(1 as libc::c_int as isize) as libc::c_int
                + ((*buf_ptr.offset(1 as libc::c_int as isize)
                    + *buf_ptr.offset(10 as libc::c_int as isize)
                    & 0xffff as libc::c_int)
                    * *FIR_Coefs.offset(1 as libc::c_int as isize) as libc::c_int
                    >> 16 as libc::c_int));
        res_Q6 = res_Q6
            + ((*buf_ptr.offset(2 as libc::c_int as isize)
                + *buf_ptr.offset(9 as libc::c_int as isize) >> 16 as libc::c_int)
                * *FIR_Coefs.offset(2 as libc::c_int as isize) as libc::c_int
                + ((*buf_ptr.offset(2 as libc::c_int as isize)
                    + *buf_ptr.offset(9 as libc::c_int as isize) & 0xffff as libc::c_int)
                    * *FIR_Coefs.offset(2 as libc::c_int as isize) as libc::c_int
                    >> 16 as libc::c_int));
        res_Q6 = res_Q6
            + ((*buf_ptr.offset(3 as libc::c_int as isize)
                + *buf_ptr.offset(8 as libc::c_int as isize) >> 16 as libc::c_int)
                * *FIR_Coefs.offset(3 as libc::c_int as isize) as libc::c_int
                + ((*buf_ptr.offset(3 as libc::c_int as isize)
                    + *buf_ptr.offset(8 as libc::c_int as isize) & 0xffff as libc::c_int)
                    * *FIR_Coefs.offset(3 as libc::c_int as isize) as libc::c_int
                    >> 16 as libc::c_int));
        res_Q6 = res_Q6
            + ((*buf_ptr.offset(4 as libc::c_int as isize)
                + *buf_ptr.offset(7 as libc::c_int as isize) >> 16 as libc::c_int)
                * *FIR_Coefs.offset(4 as libc::c_int as isize) as libc::c_int
                + ((*buf_ptr.offset(4 as libc::c_int as isize)
                    + *buf_ptr.offset(7 as libc::c_int as isize) & 0xffff as libc::c_int)
                    * *FIR_Coefs.offset(4 as libc::c_int as isize) as libc::c_int
                    >> 16 as libc::c_int));
        res_Q6 = res_Q6
            + ((*buf_ptr.offset(5 as libc::c_int as isize)
                + *buf_ptr.offset(6 as libc::c_int as isize) >> 16 as libc::c_int)
                * *FIR_Coefs.offset(5 as libc::c_int as isize) as libc::c_int
                + ((*buf_ptr.offset(5 as libc::c_int as isize)
                    + *buf_ptr.offset(6 as libc::c_int as isize) & 0xffff as libc::c_int)
                    * *FIR_Coefs.offset(5 as libc::c_int as isize) as libc::c_int
                    >> 16 as libc::c_int));
        let fresh0 = out;
        out = out.offset(1);
        *fresh0 = (if (if 6 as libc::c_int == 1 as libc::c_int {
            (res_Q6 >> 1 as libc::c_int) + (res_Q6 & 1 as libc::c_int)
        } else {
            (res_Q6 >> 6 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                >> 1 as libc::c_int
        }) > 0x7fff as libc::c_int
        {
            0x7fff as libc::c_int
        } else if (if 6 as libc::c_int == 1 as libc::c_int {
            (res_Q6 >> 1 as libc::c_int) + (res_Q6 & 1 as libc::c_int)
        } else {
            (res_Q6 >> 6 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                >> 1 as libc::c_int
        }) < 0x8000 as libc::c_int as libc::c_short as libc::c_int
        {
            0x8000 as libc::c_int as libc::c_short as libc::c_int
        } else if 6 as libc::c_int == 1 as libc::c_int {
            (res_Q6 >> 1 as libc::c_int) + (res_Q6 & 1 as libc::c_int)
        } else {
            (res_Q6 >> 6 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                >> 1 as libc::c_int
        }) as libc::c_short;
        index_Q16 += index_increment_Q16;
    }
    return out;
}
#[inline]
unsafe extern "C" fn SKP_Silk_resampler_private_down_FIR_INTERPOL1(
    mut out: *mut libc::c_short,
    mut buf2: *mut libc::c_int,
    mut FIR_Coefs: *const libc::c_short,
    mut max_index_Q16: libc::c_int,
    mut index_increment_Q16: libc::c_int,
    mut FIR_Fracs: libc::c_int,
) -> *mut libc::c_short {
    let mut index_Q16: libc::c_int = 0;
    let mut res_Q6: libc::c_int = 0;
    let mut buf_ptr: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut interpol_ind: libc::c_int = 0;
    let mut interpol_ptr: *const libc::c_short = 0 as *const libc::c_short;
    index_Q16 = 0 as libc::c_int;
    while index_Q16 < max_index_Q16 {
        buf_ptr = buf2.offset((index_Q16 >> 16 as libc::c_int) as isize);
        interpol_ind = ((index_Q16 & 0xffff as libc::c_int) >> 16 as libc::c_int)
            * FIR_Fracs as libc::c_short as libc::c_int
            + ((index_Q16 & 0xffff as libc::c_int & 0xffff as libc::c_int)
                * FIR_Fracs as libc::c_short as libc::c_int >> 16 as libc::c_int);
        interpol_ptr = &*FIR_Coefs
            .offset((12 as libc::c_int / 2 as libc::c_int * interpol_ind) as isize)
            as *const libc::c_short;
        res_Q6 = (*buf_ptr.offset(0 as libc::c_int as isize) >> 16 as libc::c_int)
            * *interpol_ptr.offset(0 as libc::c_int as isize) as libc::c_int
            + ((*buf_ptr.offset(0 as libc::c_int as isize) & 0xffff as libc::c_int)
                * *interpol_ptr.offset(0 as libc::c_int as isize) as libc::c_int
                >> 16 as libc::c_int);
        res_Q6 = res_Q6
            + ((*buf_ptr.offset(1 as libc::c_int as isize) >> 16 as libc::c_int)
                * *interpol_ptr.offset(1 as libc::c_int as isize) as libc::c_int
                + ((*buf_ptr.offset(1 as libc::c_int as isize) & 0xffff as libc::c_int)
                    * *interpol_ptr.offset(1 as libc::c_int as isize) as libc::c_int
                    >> 16 as libc::c_int));
        res_Q6 = res_Q6
            + ((*buf_ptr.offset(2 as libc::c_int as isize) >> 16 as libc::c_int)
                * *interpol_ptr.offset(2 as libc::c_int as isize) as libc::c_int
                + ((*buf_ptr.offset(2 as libc::c_int as isize) & 0xffff as libc::c_int)
                    * *interpol_ptr.offset(2 as libc::c_int as isize) as libc::c_int
                    >> 16 as libc::c_int));
        res_Q6 = res_Q6
            + ((*buf_ptr.offset(3 as libc::c_int as isize) >> 16 as libc::c_int)
                * *interpol_ptr.offset(3 as libc::c_int as isize) as libc::c_int
                + ((*buf_ptr.offset(3 as libc::c_int as isize) & 0xffff as libc::c_int)
                    * *interpol_ptr.offset(3 as libc::c_int as isize) as libc::c_int
                    >> 16 as libc::c_int));
        res_Q6 = res_Q6
            + ((*buf_ptr.offset(4 as libc::c_int as isize) >> 16 as libc::c_int)
                * *interpol_ptr.offset(4 as libc::c_int as isize) as libc::c_int
                + ((*buf_ptr.offset(4 as libc::c_int as isize) & 0xffff as libc::c_int)
                    * *interpol_ptr.offset(4 as libc::c_int as isize) as libc::c_int
                    >> 16 as libc::c_int));
        res_Q6 = res_Q6
            + ((*buf_ptr.offset(5 as libc::c_int as isize) >> 16 as libc::c_int)
                * *interpol_ptr.offset(5 as libc::c_int as isize) as libc::c_int
                + ((*buf_ptr.offset(5 as libc::c_int as isize) & 0xffff as libc::c_int)
                    * *interpol_ptr.offset(5 as libc::c_int as isize) as libc::c_int
                    >> 16 as libc::c_int));
        interpol_ptr = &*FIR_Coefs
            .offset(
                (12 as libc::c_int / 2 as libc::c_int
                    * (FIR_Fracs - 1 as libc::c_int - interpol_ind)) as isize,
            ) as *const libc::c_short;
        res_Q6 = res_Q6
            + ((*buf_ptr.offset(11 as libc::c_int as isize) >> 16 as libc::c_int)
                * *interpol_ptr.offset(0 as libc::c_int as isize) as libc::c_int
                + ((*buf_ptr.offset(11 as libc::c_int as isize) & 0xffff as libc::c_int)
                    * *interpol_ptr.offset(0 as libc::c_int as isize) as libc::c_int
                    >> 16 as libc::c_int));
        res_Q6 = res_Q6
            + ((*buf_ptr.offset(10 as libc::c_int as isize) >> 16 as libc::c_int)
                * *interpol_ptr.offset(1 as libc::c_int as isize) as libc::c_int
                + ((*buf_ptr.offset(10 as libc::c_int as isize) & 0xffff as libc::c_int)
                    * *interpol_ptr.offset(1 as libc::c_int as isize) as libc::c_int
                    >> 16 as libc::c_int));
        res_Q6 = res_Q6
            + ((*buf_ptr.offset(9 as libc::c_int as isize) >> 16 as libc::c_int)
                * *interpol_ptr.offset(2 as libc::c_int as isize) as libc::c_int
                + ((*buf_ptr.offset(9 as libc::c_int as isize) & 0xffff as libc::c_int)
                    * *interpol_ptr.offset(2 as libc::c_int as isize) as libc::c_int
                    >> 16 as libc::c_int));
        res_Q6 = res_Q6
            + ((*buf_ptr.offset(8 as libc::c_int as isize) >> 16 as libc::c_int)
                * *interpol_ptr.offset(3 as libc::c_int as isize) as libc::c_int
                + ((*buf_ptr.offset(8 as libc::c_int as isize) & 0xffff as libc::c_int)
                    * *interpol_ptr.offset(3 as libc::c_int as isize) as libc::c_int
                    >> 16 as libc::c_int));
        res_Q6 = res_Q6
            + ((*buf_ptr.offset(7 as libc::c_int as isize) >> 16 as libc::c_int)
                * *interpol_ptr.offset(4 as libc::c_int as isize) as libc::c_int
                + ((*buf_ptr.offset(7 as libc::c_int as isize) & 0xffff as libc::c_int)
                    * *interpol_ptr.offset(4 as libc::c_int as isize) as libc::c_int
                    >> 16 as libc::c_int));
        res_Q6 = res_Q6
            + ((*buf_ptr.offset(6 as libc::c_int as isize) >> 16 as libc::c_int)
                * *interpol_ptr.offset(5 as libc::c_int as isize) as libc::c_int
                + ((*buf_ptr.offset(6 as libc::c_int as isize) & 0xffff as libc::c_int)
                    * *interpol_ptr.offset(5 as libc::c_int as isize) as libc::c_int
                    >> 16 as libc::c_int));
        let fresh1 = out;
        out = out.offset(1);
        *fresh1 = (if (if 6 as libc::c_int == 1 as libc::c_int {
            (res_Q6 >> 1 as libc::c_int) + (res_Q6 & 1 as libc::c_int)
        } else {
            (res_Q6 >> 6 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                >> 1 as libc::c_int
        }) > 0x7fff as libc::c_int
        {
            0x7fff as libc::c_int
        } else if (if 6 as libc::c_int == 1 as libc::c_int {
            (res_Q6 >> 1 as libc::c_int) + (res_Q6 & 1 as libc::c_int)
        } else {
            (res_Q6 >> 6 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                >> 1 as libc::c_int
        }) < 0x8000 as libc::c_int as libc::c_short as libc::c_int
        {
            0x8000 as libc::c_int as libc::c_short as libc::c_int
        } else if 6 as libc::c_int == 1 as libc::c_int {
            (res_Q6 >> 1 as libc::c_int) + (res_Q6 & 1 as libc::c_int)
        } else {
            (res_Q6 >> 6 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                >> 1 as libc::c_int
        }) as libc::c_short;
        index_Q16 += index_increment_Q16;
    }
    return out;
}
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_resampler_private_down_FIR(
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
    let mut buf1: [libc::c_short; 240] = [0; 240];
    let mut buf2: [libc::c_int; 492] = [0; 492];
    let mut FIR_Coefs: *const libc::c_short = 0 as *const libc::c_short;
    memcpy(
        buf2.as_mut_ptr() as *mut libc::c_void,
        ((*S).sFIR).as_mut_ptr() as *const libc::c_void,
        (12 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    FIR_Coefs = &*((*S).Coefs).offset(2 as libc::c_int as isize) as *const libc::c_short;
    index_increment_Q16 = (*S).invRatio_Q16;
    loop {
        nSamplesIn = if inLen < (*S).batchSize { inLen } else { (*S).batchSize };
        if (*S).input2x == 1 as libc::c_int {
            SKP_Silk_resampler_down2(
                ((*S).sDown2).as_mut_ptr(),
                buf1.as_mut_ptr(),
                in_0,
                nSamplesIn,
            );
            nSamplesIn = nSamplesIn >> 1 as libc::c_int;
            SKP_Silk_resampler_private_AR2(
                ((*S).sIIR).as_mut_ptr(),
                &mut *buf2.as_mut_ptr().offset(12 as libc::c_int as isize),
                buf1.as_mut_ptr() as *const libc::c_short,
                (*S).Coefs,
                nSamplesIn,
            );
        } else {
            SKP_Silk_resampler_private_AR2(
                ((*S).sIIR).as_mut_ptr(),
                &mut *buf2.as_mut_ptr().offset(12 as libc::c_int as isize),
                in_0,
                (*S).Coefs,
                nSamplesIn,
            );
        }
        max_index_Q16 = nSamplesIn << 16 as libc::c_int;
        if (*S).FIR_Fracs == 1 as libc::c_int {
            out = SKP_Silk_resampler_private_down_FIR_INTERPOL0(
                out,
                buf2.as_mut_ptr(),
                FIR_Coefs,
                max_index_Q16,
                index_increment_Q16,
            );
        } else {
            out = SKP_Silk_resampler_private_down_FIR_INTERPOL1(
                out,
                buf2.as_mut_ptr(),
                FIR_Coefs,
                max_index_Q16,
                index_increment_Q16,
                (*S).FIR_Fracs,
            );
        }
        in_0 = in_0.offset((nSamplesIn << (*S).input2x) as isize);
        inLen -= nSamplesIn << (*S).input2x;
        if !(inLen > (*S).input2x) {
            break;
        }
        memcpy(
            buf2.as_mut_ptr() as *mut libc::c_void,
            &mut *buf2.as_mut_ptr().offset(nSamplesIn as isize) as *mut libc::c_int
                as *const libc::c_void,
            (12 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
        );
    }
    memcpy(
        ((*S).sFIR).as_mut_ptr() as *mut libc::c_void,
        &mut *buf2.as_mut_ptr().offset(nSamplesIn as isize) as *mut libc::c_int
            as *const libc::c_void,
        (12 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
}
