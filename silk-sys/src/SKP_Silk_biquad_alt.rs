#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_biquad_alt(
    mut in_0: *const libc::c_short,
    mut B_Q28: *const libc::c_int,
    mut A_Q28: *const libc::c_int,
    mut S: *mut libc::c_int,
    mut out: *mut libc::c_short,
    len: libc::c_int,
) {
    let mut k: libc::c_int = 0;
    let mut inval: libc::c_int = 0;
    let mut A0_U_Q28: libc::c_int = 0;
    let mut A0_L_Q28: libc::c_int = 0;
    let mut A1_U_Q28: libc::c_int = 0;
    let mut A1_L_Q28: libc::c_int = 0;
    let mut out32_Q14: libc::c_int = 0;
    A0_L_Q28 = -*A_Q28.offset(0 as libc::c_int as isize) & 0x3fff as libc::c_int;
    A0_U_Q28 = -*A_Q28.offset(0 as libc::c_int as isize) >> 14 as libc::c_int;
    A1_L_Q28 = -*A_Q28.offset(1 as libc::c_int as isize) & 0x3fff as libc::c_int;
    A1_U_Q28 = -*A_Q28.offset(1 as libc::c_int as isize) >> 14 as libc::c_int;
    k = 0 as libc::c_int;
    while k < len {
        inval = *in_0.offset(k as isize) as libc::c_int;
        out32_Q14 = *S.offset(0 as libc::c_int as isize)
            + ((*B_Q28.offset(0 as libc::c_int as isize) >> 16 as libc::c_int)
                * inval as libc::c_short as libc::c_int
                + ((*B_Q28.offset(0 as libc::c_int as isize) & 0xffff as libc::c_int)
                    * inval as libc::c_short as libc::c_int >> 16 as libc::c_int))
            << 2 as libc::c_int;
        *S
            .offset(
                0 as libc::c_int as isize,
            ) = *S.offset(1 as libc::c_int as isize)
            + (if 14 as libc::c_int == 1 as libc::c_int {
                ((out32_Q14 >> 16 as libc::c_int)
                    * A0_L_Q28 as libc::c_short as libc::c_int
                    + ((out32_Q14 & 0xffff as libc::c_int)
                        * A0_L_Q28 as libc::c_short as libc::c_int >> 16 as libc::c_int)
                    >> 1 as libc::c_int)
                    + ((out32_Q14 >> 16 as libc::c_int)
                        * A0_L_Q28 as libc::c_short as libc::c_int
                        + ((out32_Q14 & 0xffff as libc::c_int)
                            * A0_L_Q28 as libc::c_short as libc::c_int
                            >> 16 as libc::c_int) & 1 as libc::c_int)
            } else {
                ((out32_Q14 >> 16 as libc::c_int)
                    * A0_L_Q28 as libc::c_short as libc::c_int
                    + ((out32_Q14 & 0xffff as libc::c_int)
                        * A0_L_Q28 as libc::c_short as libc::c_int >> 16 as libc::c_int)
                    >> 14 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                    >> 1 as libc::c_int
            });
        *S
            .offset(
                0 as libc::c_int as isize,
            ) = *S.offset(0 as libc::c_int as isize)
            + ((out32_Q14 >> 16 as libc::c_int)
                * A0_U_Q28 as libc::c_short as libc::c_int
                + ((out32_Q14 & 0xffff as libc::c_int)
                    * A0_U_Q28 as libc::c_short as libc::c_int >> 16 as libc::c_int));
        *S
            .offset(
                0 as libc::c_int as isize,
            ) = *S.offset(0 as libc::c_int as isize)
            + ((*B_Q28.offset(1 as libc::c_int as isize) >> 16 as libc::c_int)
                * inval as libc::c_short as libc::c_int
                + ((*B_Q28.offset(1 as libc::c_int as isize) & 0xffff as libc::c_int)
                    * inval as libc::c_short as libc::c_int >> 16 as libc::c_int));
        *S
            .offset(
                1 as libc::c_int as isize,
            ) = if 14 as libc::c_int == 1 as libc::c_int {
            ((out32_Q14 >> 16 as libc::c_int) * A1_L_Q28 as libc::c_short as libc::c_int
                + ((out32_Q14 & 0xffff as libc::c_int)
                    * A1_L_Q28 as libc::c_short as libc::c_int >> 16 as libc::c_int)
                >> 1 as libc::c_int)
                + ((out32_Q14 >> 16 as libc::c_int)
                    * A1_L_Q28 as libc::c_short as libc::c_int
                    + ((out32_Q14 & 0xffff as libc::c_int)
                        * A1_L_Q28 as libc::c_short as libc::c_int >> 16 as libc::c_int)
                    & 1 as libc::c_int)
        } else {
            ((out32_Q14 >> 16 as libc::c_int) * A1_L_Q28 as libc::c_short as libc::c_int
                + ((out32_Q14 & 0xffff as libc::c_int)
                    * A1_L_Q28 as libc::c_short as libc::c_int >> 16 as libc::c_int)
                >> 14 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                >> 1 as libc::c_int
        };
        *S
            .offset(
                1 as libc::c_int as isize,
            ) = *S.offset(1 as libc::c_int as isize)
            + ((out32_Q14 >> 16 as libc::c_int)
                * A1_U_Q28 as libc::c_short as libc::c_int
                + ((out32_Q14 & 0xffff as libc::c_int)
                    * A1_U_Q28 as libc::c_short as libc::c_int >> 16 as libc::c_int));
        *S
            .offset(
                1 as libc::c_int as isize,
            ) = *S.offset(1 as libc::c_int as isize)
            + ((*B_Q28.offset(2 as libc::c_int as isize) >> 16 as libc::c_int)
                * inval as libc::c_short as libc::c_int
                + ((*B_Q28.offset(2 as libc::c_int as isize) & 0xffff as libc::c_int)
                    * inval as libc::c_short as libc::c_int >> 16 as libc::c_int));
        *out
            .offset(
                k as isize,
            ) = (if out32_Q14 + ((1 as libc::c_int) << 14 as libc::c_int)
            - 1 as libc::c_int >> 14 as libc::c_int > 0x7fff as libc::c_int
        {
            0x7fff as libc::c_int
        } else if (out32_Q14 + ((1 as libc::c_int) << 14 as libc::c_int)
            - 1 as libc::c_int >> 14 as libc::c_int)
            < 0x8000 as libc::c_int as libc::c_short as libc::c_int
        {
            0x8000 as libc::c_int as libc::c_short as libc::c_int
        } else {
            out32_Q14 + ((1 as libc::c_int) << 14 as libc::c_int) - 1 as libc::c_int
                >> 14 as libc::c_int
        }) as libc::c_short;
        k += 1;
        k;
    }
}
