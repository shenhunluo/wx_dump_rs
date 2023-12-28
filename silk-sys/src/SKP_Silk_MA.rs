#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#[no_mangle]
pub fn SKP_Silk_MA_Prediction(
    in_0: &[i16],
    mut B: &[i16],
    mut S: &mut [i32],
    mut out: &mut [i16],
    len: usize,
    order: usize,
) {
    let mut in16: libc::c_int = 0;
    let mut out32: libc::c_int = 0;
    for k in 0..len {
        in16 = in_0[k] as libc::c_int;
        out32 = (in16 << 12 as libc::c_int) - S[0];
        out32 = if 12 as libc::c_int == 1 as libc::c_int {
            (out32 >> 1 as libc::c_int) + (out32 & 1 as libc::c_int)
        } else {
            (out32 >> 12 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                >> 1 as libc::c_int
        };
        for d in 0..order - 1 {
            S[d] = (S[d+1] as libc::c_uint)
                .wrapping_add(
                    (in16 as libc::c_short as libc::c_int
                        * B[d] as libc::c_int) as libc::c_uint,
                ) as libc::c_int;
        }
        S[order - 1] = in16 as libc::c_short as libc::c_int
            * B[order - 1] as libc::c_int;
        out[k] = (if out32 > 0x7fff as libc::c_int {
            0x7fff as libc::c_int
        } else if out32 < 0x8000 as libc::c_int as libc::c_short as libc::c_int {
            0x8000 as libc::c_int as libc::c_short as libc::c_int
        } else {
            out32
        }) as libc::c_short;
    }
}
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_LPC_analysis_filter(
    mut in_0: *const libc::c_short,
    mut B: *const libc::c_short,
    mut S: *mut libc::c_short,
    mut out: *mut libc::c_short,
    len: libc::c_int,
    Order: libc::c_int,
) {
    let mut k: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut idx: libc::c_int = 0;
    let mut Order_half: libc::c_int = Order >> 1 as libc::c_int;
    let mut out32_Q12: libc::c_int = 0;
    let mut out32: libc::c_int = 0;
    let mut SA: libc::c_short = 0;
    let mut SB: libc::c_short = 0;
    k = 0 as libc::c_int;
    while k < len {
        SA = *S.offset(0 as libc::c_int as isize);
        out32_Q12 = 0 as libc::c_int;
        j = 0 as libc::c_int;
        while j < Order_half - 1 as libc::c_int {
            idx = 2 as libc::c_int as libc::c_short as libc::c_int
                * j as libc::c_short as libc::c_int + 1 as libc::c_int;
            SB = *S.offset(idx as isize);
            *S.offset(idx as isize) = SA;
            out32_Q12 = out32_Q12
                + SA as libc::c_int
                    * *B.offset((idx - 1 as libc::c_int) as isize) as libc::c_int;
            out32_Q12 = out32_Q12
                + SB as libc::c_int * *B.offset(idx as isize) as libc::c_int;
            SA = *S.offset((idx + 1 as libc::c_int) as isize);
            *S.offset((idx + 1 as libc::c_int) as isize) = SB;
            j += 1;
        }
        SB = *S.offset((Order - 1 as libc::c_int) as isize);
        *S.offset((Order - 1 as libc::c_int) as isize) = SA;
        out32_Q12 = out32_Q12
            + SA as libc::c_int
                * *B.offset((Order - 2 as libc::c_int) as isize) as libc::c_int;
        out32_Q12 = out32_Q12
            + SB as libc::c_int
                * *B.offset((Order - 1 as libc::c_int) as isize) as libc::c_int;
        out32_Q12 = if (((*in_0.offset(k as isize) as libc::c_int) << 12 as libc::c_int)
            - out32_Q12) as libc::c_uint & 0x80000000 as libc::c_uint
            == 0 as libc::c_int as libc::c_uint
        {
            if ((*in_0.offset(k as isize) as libc::c_int) << 12 as libc::c_int)
                as libc::c_uint
                & (out32_Q12 as libc::c_uint ^ 0x80000000 as libc::c_uint)
                & 0x80000000 as libc::c_uint != 0
            {
                0x80000000 as libc::c_uint as libc::c_int
            } else {
                ((*in_0.offset(k as isize) as libc::c_int) << 12 as libc::c_int)
                    - out32_Q12
            }
        } else if (((*in_0.offset(k as isize) as libc::c_int) << 12 as libc::c_int)
            as libc::c_uint ^ 0x80000000 as libc::c_uint) & out32_Q12 as libc::c_uint
            & 0x80000000 as libc::c_uint != 0
        {
            0x7fffffff as libc::c_int
        } else {
            ((*in_0.offset(k as isize) as libc::c_int) << 12 as libc::c_int) - out32_Q12
        };
        out32 = if 12 as libc::c_int == 1 as libc::c_int {
            (out32_Q12 >> 1 as libc::c_int) + (out32_Q12 & 1 as libc::c_int)
        } else {
            (out32_Q12 >> 12 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                >> 1 as libc::c_int
        };
        *out
            .offset(
                k as isize,
            ) = (if out32 > 0x7fff as libc::c_int {
            0x7fff as libc::c_int
        } else if out32 < 0x8000 as libc::c_int as libc::c_short as libc::c_int {
            0x8000 as libc::c_int as libc::c_short as libc::c_int
        } else {
            out32
        }) as libc::c_short;
        *S.offset(0 as libc::c_int as isize) = *in_0.offset(k as isize);
        k += 1;
    }
}
