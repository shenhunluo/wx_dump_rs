#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_resampler_private_ARMA4(
    mut S: *mut libc::c_int,
    mut out: *mut libc::c_short,
    mut in_0: *const libc::c_short,
    mut Coef: *const libc::c_short,
    mut len: libc::c_int,
) {
    let mut k: libc::c_int = 0;
    let mut in_Q8: libc::c_int = 0;
    let mut out1_Q8: libc::c_int = 0;
    let mut out2_Q8: libc::c_int = 0;
    let mut X: libc::c_int = 0;
    k = 0 as libc::c_int;
    while k < len {
        in_Q8 = (*in_0.offset(k as isize) as libc::c_int) << 8 as libc::c_int;
        out1_Q8 = in_Q8 + (*S.offset(0 as libc::c_int as isize) << 2 as libc::c_int);
        out2_Q8 = out1_Q8 + (*S.offset(2 as libc::c_int as isize) << 2 as libc::c_int);
        X = *S.offset(1 as libc::c_int as isize)
            + ((in_Q8 >> 16 as libc::c_int)
                * *Coef.offset(0 as libc::c_int as isize) as libc::c_int
                + ((in_Q8 & 0xffff as libc::c_int)
                    * *Coef.offset(0 as libc::c_int as isize) as libc::c_int
                    >> 16 as libc::c_int));
        *S
            .offset(
                0 as libc::c_int as isize,
            ) = X
            + ((out1_Q8 >> 16 as libc::c_int)
                * *Coef.offset(2 as libc::c_int as isize) as libc::c_int
                + ((out1_Q8 & 0xffff as libc::c_int)
                    * *Coef.offset(2 as libc::c_int as isize) as libc::c_int
                    >> 16 as libc::c_int));
        X = *S.offset(3 as libc::c_int as isize)
            + ((out1_Q8 >> 16 as libc::c_int)
                * *Coef.offset(1 as libc::c_int as isize) as libc::c_int
                + ((out1_Q8 & 0xffff as libc::c_int)
                    * *Coef.offset(1 as libc::c_int as isize) as libc::c_int
                    >> 16 as libc::c_int));
        *S
            .offset(
                2 as libc::c_int as isize,
            ) = X
            + ((out2_Q8 >> 16 as libc::c_int)
                * *Coef.offset(4 as libc::c_int as isize) as libc::c_int
                + ((out2_Q8 & 0xffff as libc::c_int)
                    * *Coef.offset(4 as libc::c_int as isize) as libc::c_int
                    >> 16 as libc::c_int));
        *S
            .offset(
                1 as libc::c_int as isize,
            ) = (in_Q8 >> 2 as libc::c_int)
            + ((out1_Q8 >> 16 as libc::c_int)
                * *Coef.offset(3 as libc::c_int as isize) as libc::c_int
                + ((out1_Q8 & 0xffff as libc::c_int)
                    * *Coef.offset(3 as libc::c_int as isize) as libc::c_int
                    >> 16 as libc::c_int));
        *S
            .offset(
                3 as libc::c_int as isize,
            ) = (out1_Q8 >> 2 as libc::c_int)
            + ((out2_Q8 >> 16 as libc::c_int)
                * *Coef.offset(5 as libc::c_int as isize) as libc::c_int
                + ((out2_Q8 & 0xffff as libc::c_int)
                    * *Coef.offset(5 as libc::c_int as isize) as libc::c_int
                    >> 16 as libc::c_int));
        *out
            .offset(
                k as isize,
            ) = (if 128 as libc::c_int
            + ((out2_Q8 >> 16 as libc::c_int)
                * *Coef.offset(6 as libc::c_int as isize) as libc::c_int
                + ((out2_Q8 & 0xffff as libc::c_int)
                    * *Coef.offset(6 as libc::c_int as isize) as libc::c_int
                    >> 16 as libc::c_int)) >> 8 as libc::c_int > 0x7fff as libc::c_int
        {
            0x7fff as libc::c_int
        } else if (128 as libc::c_int
            + ((out2_Q8 >> 16 as libc::c_int)
                * *Coef.offset(6 as libc::c_int as isize) as libc::c_int
                + ((out2_Q8 & 0xffff as libc::c_int)
                    * *Coef.offset(6 as libc::c_int as isize) as libc::c_int
                    >> 16 as libc::c_int)) >> 8 as libc::c_int)
            < 0x8000 as libc::c_int as libc::c_short as libc::c_int
        {
            0x8000 as libc::c_int as libc::c_short as libc::c_int
        } else {
            128 as libc::c_int
                + ((out2_Q8 >> 16 as libc::c_int)
                    * *Coef.offset(6 as libc::c_int as isize) as libc::c_int
                    + ((out2_Q8 & 0xffff as libc::c_int)
                        * *Coef.offset(6 as libc::c_int as isize) as libc::c_int
                        >> 16 as libc::c_int)) >> 8 as libc::c_int
        }) as libc::c_short;
        k += 1;
    }
}
