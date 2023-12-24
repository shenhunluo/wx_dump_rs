#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn SKP_Silk_resampler_private_AR2(
        S: *mut libc::c_int,
        out_Q8: *mut libc::c_int,
        in_0: *const libc::c_short,
        A_Q14: *const libc::c_short,
        len: libc::c_int,
    );
    static SKP_Silk_Resampler_1_3_COEFS_LQ: [libc::c_short; 5];
}
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_resampler_down3(
    mut S: *mut libc::c_int,
    mut out: *mut libc::c_short,
    mut in_0: *const libc::c_short,
    mut inLen: libc::c_int,
) {
    let mut nSamplesIn: libc::c_int = 0;
    let mut counter: libc::c_int = 0;
    let mut res_Q6: libc::c_int = 0;
    let mut buf: [libc::c_int; 486] = [0; 486];
    let mut buf_ptr: *mut libc::c_int = 0 as *mut libc::c_int;
    memcpy(
        buf.as_mut_ptr() as *mut libc::c_void,
        S as *const libc::c_void,
        (6 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    loop {
        nSamplesIn = if inLen < 480 as libc::c_int { inLen } else { 480 as libc::c_int };
        SKP_Silk_resampler_private_AR2(
            &mut *S.offset(6 as libc::c_int as isize),
            &mut *buf.as_mut_ptr().offset(6 as libc::c_int as isize),
            in_0,
            SKP_Silk_Resampler_1_3_COEFS_LQ.as_ptr(),
            nSamplesIn,
        );
        buf_ptr = buf.as_mut_ptr();
        counter = nSamplesIn;
        while counter > 2 as libc::c_int {
            res_Q6 = (*buf_ptr.offset(0 as libc::c_int as isize)
                + *buf_ptr.offset(5 as libc::c_int as isize) >> 16 as libc::c_int)
                * SKP_Silk_Resampler_1_3_COEFS_LQ[2 as libc::c_int as usize]
                    as libc::c_int
                + ((*buf_ptr.offset(0 as libc::c_int as isize)
                    + *buf_ptr.offset(5 as libc::c_int as isize) & 0xffff as libc::c_int)
                    * SKP_Silk_Resampler_1_3_COEFS_LQ[2 as libc::c_int as usize]
                        as libc::c_int >> 16 as libc::c_int);
            res_Q6 = res_Q6
                + ((*buf_ptr.offset(1 as libc::c_int as isize)
                    + *buf_ptr.offset(4 as libc::c_int as isize) >> 16 as libc::c_int)
                    * SKP_Silk_Resampler_1_3_COEFS_LQ[3 as libc::c_int as usize]
                        as libc::c_int
                    + ((*buf_ptr.offset(1 as libc::c_int as isize)
                        + *buf_ptr.offset(4 as libc::c_int as isize)
                        & 0xffff as libc::c_int)
                        * SKP_Silk_Resampler_1_3_COEFS_LQ[3 as libc::c_int as usize]
                            as libc::c_int >> 16 as libc::c_int));
            res_Q6 = res_Q6
                + ((*buf_ptr.offset(2 as libc::c_int as isize)
                    + *buf_ptr.offset(3 as libc::c_int as isize) >> 16 as libc::c_int)
                    * SKP_Silk_Resampler_1_3_COEFS_LQ[4 as libc::c_int as usize]
                        as libc::c_int
                    + ((*buf_ptr.offset(2 as libc::c_int as isize)
                        + *buf_ptr.offset(3 as libc::c_int as isize)
                        & 0xffff as libc::c_int)
                        * SKP_Silk_Resampler_1_3_COEFS_LQ[4 as libc::c_int as usize]
                            as libc::c_int >> 16 as libc::c_int));
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
            buf_ptr = buf_ptr.offset(3 as libc::c_int as isize);
            counter -= 3 as libc::c_int;
        }
        in_0 = in_0.offset(nSamplesIn as isize);
        inLen -= nSamplesIn;
        if !(inLen > 0 as libc::c_int) {
            break;
        }
        memcpy(
            buf.as_mut_ptr() as *mut libc::c_void,
            &mut *buf.as_mut_ptr().offset(nSamplesIn as isize) as *mut libc::c_int
                as *const libc::c_void,
            (6 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
        );
    }
    memcpy(
        S as *mut libc::c_void,
        &mut *buf.as_mut_ptr().offset(nSamplesIn as isize) as *mut libc::c_int
            as *const libc::c_void,
        (6 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
}
