#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn SKP_Silk_LPC_analysis_filter(
        in_0: *const libc::c_short,
        B: *const libc::c_short,
        S: *mut libc::c_short,
        out: *mut libc::c_short,
        len: libc::c_int,
        Order: libc::c_int,
    );
    fn SKP_Silk_sum_sqr_shift(
        energy: *mut libc::c_int,
        shift: *mut libc::c_int,
        x: *const libc::c_short,
        len: libc::c_int,
    );
}
pub type __int64_t = libc::c_longlong;
pub type int64_t = __int64_t;
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
pub unsafe extern "C" fn SKP_Silk_residual_energy_FIX(
    mut nrgs: *mut libc::c_int,
    mut nrgsQ: *mut libc::c_int,
    mut x: *const libc::c_short,
    mut a_Q12: *mut [libc::c_short; 16],
    mut gains: *const libc::c_int,
    subfr_length: libc::c_int,
    LPC_order: libc::c_int,
) {
    let mut offset: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut rshift: libc::c_int = 0;
    let mut lz1: libc::c_int = 0;
    let mut lz2: libc::c_int = 0;
    let mut LPC_res_ptr: *mut libc::c_short = 0 as *mut libc::c_short;
    let mut LPC_res: [libc::c_short; 272] = [0; 272];
    let mut x_ptr: *const libc::c_short = 0 as *const libc::c_short;
    let mut S: [libc::c_short; 16] = [0; 16];
    let mut tmp32: libc::c_int = 0;
    x_ptr = x;
    offset = LPC_order + subfr_length;
    i = 0 as libc::c_int;
    while i < 2 as libc::c_int {
        memset(
            S.as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            (LPC_order as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_short>() as libc::c_ulong),
        );
        SKP_Silk_LPC_analysis_filter(
            x_ptr,
            (*a_Q12.offset(i as isize)).as_mut_ptr(),
            S.as_mut_ptr(),
            LPC_res.as_mut_ptr(),
            (4 as libc::c_int >> 1 as libc::c_int) * offset,
            LPC_order,
        );
        LPC_res_ptr = LPC_res.as_mut_ptr().offset(LPC_order as isize);
        j = 0 as libc::c_int;
        while j < 4 as libc::c_int >> 1 as libc::c_int {
            SKP_Silk_sum_sqr_shift(
                &mut *nrgs
                    .offset((i * (4 as libc::c_int >> 1 as libc::c_int) + j) as isize),
                &mut rshift,
                LPC_res_ptr,
                subfr_length,
            );
            *nrgsQ
                .offset(
                    (i * (4 as libc::c_int >> 1 as libc::c_int) + j) as isize,
                ) = -rshift;
            LPC_res_ptr = LPC_res_ptr.offset(offset as isize);
            j += 1;
            j;
        }
        x_ptr = x_ptr.offset(((4 as libc::c_int >> 1 as libc::c_int) * offset) as isize);
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        lz1 = SKP_Silk_CLZ32(*nrgs.offset(i as isize)) - 1 as libc::c_int;
        lz2 = SKP_Silk_CLZ32(*gains.offset(i as isize)) - 1 as libc::c_int;
        tmp32 = *gains.offset(i as isize) << lz2;
        tmp32 = (tmp32 as int64_t * tmp32 as int64_t >> 32 as libc::c_int)
            as libc::c_int;
        *nrgs
            .offset(
                i as isize,
            ) = (tmp32 as int64_t * (*nrgs.offset(i as isize) << lz1) as int64_t
            >> 32 as libc::c_int) as libc::c_int;
        *nrgsQ.offset(i as isize)
            += lz1 + 2 as libc::c_int * lz2 - 32 as libc::c_int - 32 as libc::c_int;
        i += 1;
        i;
    }
}
