#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_LPC_synthesis_order16(
    mut in_0: *const libc::c_short,
    mut A_Q12: *const libc::c_short,
    Gain_Q26: libc::c_int,
    mut S: *mut libc::c_int,
    mut out: *mut libc::c_short,
    len: libc::c_int,
) {
    let mut k: libc::c_int = 0;
    let mut SA: libc::c_int = 0;
    let mut SB: libc::c_int = 0;
    let mut out32_Q10: libc::c_int = 0;
    let mut out32: libc::c_int = 0;
    let mut Atmp: libc::c_int = 0;
    let mut A_align_Q12: [libc::c_int; 8] = [0; 8];
    k = 0 as libc::c_int;
    while k < 8 as libc::c_int {
        A_align_Q12[k
            as usize] = *A_Q12.offset((2 as libc::c_int * k) as isize) as libc::c_int
            & 0xffff as libc::c_int
            | (*A_Q12.offset((2 as libc::c_int * k + 1 as libc::c_int) as isize)
                as libc::c_int) << 16 as libc::c_int;
        k += 1;
    }
    k = 0 as libc::c_int;
    while k < len {
        SA = *S.offset(15 as libc::c_int as isize);
        Atmp = A_align_Q12[0 as libc::c_int as usize];
        SB = *S.offset(14 as libc::c_int as isize);
        *S.offset(14 as libc::c_int as isize) = SA;
        out32_Q10 = (SA >> 16 as libc::c_int) * Atmp as libc::c_short as libc::c_int
            + ((SA & 0xffff as libc::c_int) * Atmp as libc::c_short as libc::c_int
                >> 16 as libc::c_int);
        out32_Q10 = (out32_Q10 as libc::c_uint)
            .wrapping_add(
                ((SB >> 16 as libc::c_int) * (Atmp >> 16 as libc::c_int)
                    + ((SB & 0xffff as libc::c_int) * (Atmp >> 16 as libc::c_int)
                        >> 16 as libc::c_int)) as libc::c_uint,
            ) as libc::c_int;
        SA = *S.offset(13 as libc::c_int as isize);
        *S.offset(13 as libc::c_int as isize) = SB;
        Atmp = A_align_Q12[1 as libc::c_int as usize];
        SB = *S.offset(12 as libc::c_int as isize);
        *S.offset(12 as libc::c_int as isize) = SA;
        out32_Q10 = (out32_Q10 as libc::c_uint)
            .wrapping_add(
                ((SA >> 16 as libc::c_int) * Atmp as libc::c_short as libc::c_int
                    + ((SA & 0xffff as libc::c_int)
                        * Atmp as libc::c_short as libc::c_int >> 16 as libc::c_int))
                    as libc::c_uint,
            ) as libc::c_int;
        out32_Q10 = (out32_Q10 as libc::c_uint)
            .wrapping_add(
                ((SB >> 16 as libc::c_int) * (Atmp >> 16 as libc::c_int)
                    + ((SB & 0xffff as libc::c_int) * (Atmp >> 16 as libc::c_int)
                        >> 16 as libc::c_int)) as libc::c_uint,
            ) as libc::c_int;
        SA = *S.offset(11 as libc::c_int as isize);
        *S.offset(11 as libc::c_int as isize) = SB;
        Atmp = A_align_Q12[2 as libc::c_int as usize];
        SB = *S.offset(10 as libc::c_int as isize);
        *S.offset(10 as libc::c_int as isize) = SA;
        out32_Q10 = (out32_Q10 as libc::c_uint)
            .wrapping_add(
                ((SA >> 16 as libc::c_int) * Atmp as libc::c_short as libc::c_int
                    + ((SA & 0xffff as libc::c_int)
                        * Atmp as libc::c_short as libc::c_int >> 16 as libc::c_int))
                    as libc::c_uint,
            ) as libc::c_int;
        out32_Q10 = (out32_Q10 as libc::c_uint)
            .wrapping_add(
                ((SB >> 16 as libc::c_int) * (Atmp >> 16 as libc::c_int)
                    + ((SB & 0xffff as libc::c_int) * (Atmp >> 16 as libc::c_int)
                        >> 16 as libc::c_int)) as libc::c_uint,
            ) as libc::c_int;
        SA = *S.offset(9 as libc::c_int as isize);
        *S.offset(9 as libc::c_int as isize) = SB;
        Atmp = A_align_Q12[3 as libc::c_int as usize];
        SB = *S.offset(8 as libc::c_int as isize);
        *S.offset(8 as libc::c_int as isize) = SA;
        out32_Q10 = (out32_Q10 as libc::c_uint)
            .wrapping_add(
                ((SA >> 16 as libc::c_int) * Atmp as libc::c_short as libc::c_int
                    + ((SA & 0xffff as libc::c_int)
                        * Atmp as libc::c_short as libc::c_int >> 16 as libc::c_int))
                    as libc::c_uint,
            ) as libc::c_int;
        out32_Q10 = (out32_Q10 as libc::c_uint)
            .wrapping_add(
                ((SB >> 16 as libc::c_int) * (Atmp >> 16 as libc::c_int)
                    + ((SB & 0xffff as libc::c_int) * (Atmp >> 16 as libc::c_int)
                        >> 16 as libc::c_int)) as libc::c_uint,
            ) as libc::c_int;
        SA = *S.offset(7 as libc::c_int as isize);
        *S.offset(7 as libc::c_int as isize) = SB;
        Atmp = A_align_Q12[4 as libc::c_int as usize];
        SB = *S.offset(6 as libc::c_int as isize);
        *S.offset(6 as libc::c_int as isize) = SA;
        out32_Q10 = (out32_Q10 as libc::c_uint)
            .wrapping_add(
                ((SA >> 16 as libc::c_int) * Atmp as libc::c_short as libc::c_int
                    + ((SA & 0xffff as libc::c_int)
                        * Atmp as libc::c_short as libc::c_int >> 16 as libc::c_int))
                    as libc::c_uint,
            ) as libc::c_int;
        out32_Q10 = (out32_Q10 as libc::c_uint)
            .wrapping_add(
                ((SB >> 16 as libc::c_int) * (Atmp >> 16 as libc::c_int)
                    + ((SB & 0xffff as libc::c_int) * (Atmp >> 16 as libc::c_int)
                        >> 16 as libc::c_int)) as libc::c_uint,
            ) as libc::c_int;
        SA = *S.offset(5 as libc::c_int as isize);
        *S.offset(5 as libc::c_int as isize) = SB;
        Atmp = A_align_Q12[5 as libc::c_int as usize];
        SB = *S.offset(4 as libc::c_int as isize);
        *S.offset(4 as libc::c_int as isize) = SA;
        out32_Q10 = (out32_Q10 as libc::c_uint)
            .wrapping_add(
                ((SA >> 16 as libc::c_int) * Atmp as libc::c_short as libc::c_int
                    + ((SA & 0xffff as libc::c_int)
                        * Atmp as libc::c_short as libc::c_int >> 16 as libc::c_int))
                    as libc::c_uint,
            ) as libc::c_int;
        out32_Q10 = (out32_Q10 as libc::c_uint)
            .wrapping_add(
                ((SB >> 16 as libc::c_int) * (Atmp >> 16 as libc::c_int)
                    + ((SB & 0xffff as libc::c_int) * (Atmp >> 16 as libc::c_int)
                        >> 16 as libc::c_int)) as libc::c_uint,
            ) as libc::c_int;
        SA = *S.offset(3 as libc::c_int as isize);
        *S.offset(3 as libc::c_int as isize) = SB;
        Atmp = A_align_Q12[6 as libc::c_int as usize];
        SB = *S.offset(2 as libc::c_int as isize);
        *S.offset(2 as libc::c_int as isize) = SA;
        out32_Q10 = (out32_Q10 as libc::c_uint)
            .wrapping_add(
                ((SA >> 16 as libc::c_int) * Atmp as libc::c_short as libc::c_int
                    + ((SA & 0xffff as libc::c_int)
                        * Atmp as libc::c_short as libc::c_int >> 16 as libc::c_int))
                    as libc::c_uint,
            ) as libc::c_int;
        out32_Q10 = (out32_Q10 as libc::c_uint)
            .wrapping_add(
                ((SB >> 16 as libc::c_int) * (Atmp >> 16 as libc::c_int)
                    + ((SB & 0xffff as libc::c_int) * (Atmp >> 16 as libc::c_int)
                        >> 16 as libc::c_int)) as libc::c_uint,
            ) as libc::c_int;
        SA = *S.offset(1 as libc::c_int as isize);
        *S.offset(1 as libc::c_int as isize) = SB;
        Atmp = A_align_Q12[7 as libc::c_int as usize];
        SB = *S.offset(0 as libc::c_int as isize);
        *S.offset(0 as libc::c_int as isize) = SA;
        out32_Q10 = (out32_Q10 as libc::c_uint)
            .wrapping_add(
                ((SA >> 16 as libc::c_int) * Atmp as libc::c_short as libc::c_int
                    + ((SA & 0xffff as libc::c_int)
                        * Atmp as libc::c_short as libc::c_int >> 16 as libc::c_int))
                    as libc::c_uint,
            ) as libc::c_int;
        out32_Q10 = (out32_Q10 as libc::c_uint)
            .wrapping_add(
                ((SB >> 16 as libc::c_int) * (Atmp >> 16 as libc::c_int)
                    + ((SB & 0xffff as libc::c_int) * (Atmp >> 16 as libc::c_int)
                        >> 16 as libc::c_int)) as libc::c_uint,
            ) as libc::c_int;
        out32_Q10 = if (out32_Q10
            + ((Gain_Q26 >> 16 as libc::c_int) * *in_0.offset(k as isize) as libc::c_int
                + ((Gain_Q26 & 0xffff as libc::c_int)
                    * *in_0.offset(k as isize) as libc::c_int >> 16 as libc::c_int)))
            as libc::c_uint & 0x80000000 as libc::c_uint
            == 0 as libc::c_int as libc::c_uint
        {
            if (out32_Q10
                & (Gain_Q26 >> 16 as libc::c_int)
                    * *in_0.offset(k as isize) as libc::c_int
                    + ((Gain_Q26 & 0xffff as libc::c_int)
                        * *in_0.offset(k as isize) as libc::c_int >> 16 as libc::c_int))
                as libc::c_uint & 0x80000000 as libc::c_uint
                != 0 as libc::c_int as libc::c_uint
            {
                0x80000000 as libc::c_uint as libc::c_int
            } else {
                out32_Q10
                    + ((Gain_Q26 >> 16 as libc::c_int)
                        * *in_0.offset(k as isize) as libc::c_int
                        + ((Gain_Q26 & 0xffff as libc::c_int)
                            * *in_0.offset(k as isize) as libc::c_int
                            >> 16 as libc::c_int))
            }
        } else if (out32_Q10
            | (Gain_Q26 >> 16 as libc::c_int) * *in_0.offset(k as isize) as libc::c_int
                + ((Gain_Q26 & 0xffff as libc::c_int)
                    * *in_0.offset(k as isize) as libc::c_int >> 16 as libc::c_int))
            as libc::c_uint & 0x80000000 as libc::c_uint
            == 0 as libc::c_int as libc::c_uint
        {
            0x7fffffff as libc::c_int
        } else {
            out32_Q10
                + ((Gain_Q26 >> 16 as libc::c_int)
                    * *in_0.offset(k as isize) as libc::c_int
                    + ((Gain_Q26 & 0xffff as libc::c_int)
                        * *in_0.offset(k as isize) as libc::c_int >> 16 as libc::c_int))
        };
        out32 = if 10 as libc::c_int == 1 as libc::c_int {
            (out32_Q10 >> 1 as libc::c_int) + (out32_Q10 & 1 as libc::c_int)
        } else {
            (out32_Q10 >> 10 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
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
        *S
            .offset(
                15 as libc::c_int as isize,
            ) = (if 0x80000000 as libc::c_uint as libc::c_int >> 4 as libc::c_int
            > 0x7fffffff as libc::c_int >> 4 as libc::c_int
        {
            if out32_Q10 > 0x80000000 as libc::c_uint as libc::c_int >> 4 as libc::c_int
            {
                0x80000000 as libc::c_uint as libc::c_int >> 4 as libc::c_int
            } else {
                if out32_Q10 < 0x7fffffff as libc::c_int >> 4 as libc::c_int {
                    0x7fffffff as libc::c_int >> 4 as libc::c_int
                } else {
                    out32_Q10
                }
            }
        } else {
            if out32_Q10 > 0x7fffffff as libc::c_int >> 4 as libc::c_int {
                0x7fffffff as libc::c_int >> 4 as libc::c_int
            } else {
                if out32_Q10
                    < 0x80000000 as libc::c_uint as libc::c_int >> 4 as libc::c_int
                {
                    0x80000000 as libc::c_uint as libc::c_int >> 4 as libc::c_int
                } else {
                    out32_Q10
                }
            }
        }) << 4 as libc::c_int;
        k += 1;
    }
}
