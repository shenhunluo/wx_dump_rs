#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn SKP_Silk_biquad_alt(
        in_0: *const libc::c_short,
        B_Q28: *const libc::c_int,
        A_Q28: *const libc::c_int,
        S: *mut libc::c_int,
        out: *mut libc::c_short,
        len: libc::c_int,
    );
    static SKP_Silk_Transition_LP_B_Q28: [[libc::c_int; 3]; 5];
    static SKP_Silk_Transition_LP_A_Q28: [[libc::c_int; 2]; 5];
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SKP_Silk_LP_state {
    pub In_LP_State: [libc::c_int; 2],
    pub transition_frame_no: libc::c_int,
    pub mode: libc::c_int,
}
#[inline]
unsafe extern "C" fn SKP_Silk_LP_interpolate_filter_taps(
    mut B_Q28: *mut libc::c_int,
    mut A_Q28: *mut libc::c_int,
    ind: libc::c_int,
    fac_Q16: libc::c_int,
) {
    let mut nb: libc::c_int = 0;
    let mut na: libc::c_int = 0;
    if ind < 5 as libc::c_int - 1 as libc::c_int {
        if fac_Q16 > 0 as libc::c_int {
            if fac_Q16
                == (if fac_Q16 > 0x7fff as libc::c_int {
                    0x7fff as libc::c_int
                } else {
                    (if fac_Q16 < 0x8000 as libc::c_int as libc::c_short as libc::c_int {
                        0x8000 as libc::c_int as libc::c_short as libc::c_int
                    } else {
                        fac_Q16
                    })
                })
            {
                nb = 0 as libc::c_int;
                while nb < 3 as libc::c_int {
                    *B_Q28
                        .offset(
                            nb as isize,
                        ) = SKP_Silk_Transition_LP_B_Q28[ind as usize][nb as usize]
                        + ((SKP_Silk_Transition_LP_B_Q28[(ind + 1 as libc::c_int)
                            as usize][nb as usize]
                            - SKP_Silk_Transition_LP_B_Q28[ind as usize][nb as usize]
                            >> 16 as libc::c_int)
                            * fac_Q16 as libc::c_short as libc::c_int
                            + ((SKP_Silk_Transition_LP_B_Q28[(ind + 1 as libc::c_int)
                                as usize][nb as usize]
                                - SKP_Silk_Transition_LP_B_Q28[ind as usize][nb as usize]
                                & 0xffff as libc::c_int)
                                * fac_Q16 as libc::c_short as libc::c_int
                                >> 16 as libc::c_int));
                    nb += 1;
                    nb;
                }
                na = 0 as libc::c_int;
                while na < 2 as libc::c_int {
                    *A_Q28
                        .offset(
                            na as isize,
                        ) = SKP_Silk_Transition_LP_A_Q28[ind as usize][na as usize]
                        + ((SKP_Silk_Transition_LP_A_Q28[(ind + 1 as libc::c_int)
                            as usize][na as usize]
                            - SKP_Silk_Transition_LP_A_Q28[ind as usize][na as usize]
                            >> 16 as libc::c_int)
                            * fac_Q16 as libc::c_short as libc::c_int
                            + ((SKP_Silk_Transition_LP_A_Q28[(ind + 1 as libc::c_int)
                                as usize][na as usize]
                                - SKP_Silk_Transition_LP_A_Q28[ind as usize][na as usize]
                                & 0xffff as libc::c_int)
                                * fac_Q16 as libc::c_short as libc::c_int
                                >> 16 as libc::c_int));
                    na += 1;
                    na;
                }
            } else if fac_Q16 == (1 as libc::c_int) << 15 as libc::c_int {
                nb = 0 as libc::c_int;
                while nb < 3 as libc::c_int {
                    *B_Q28
                        .offset(
                            nb as isize,
                        ) = SKP_Silk_Transition_LP_B_Q28[ind as usize][nb as usize]
                        + SKP_Silk_Transition_LP_B_Q28[(ind + 1 as libc::c_int)
                            as usize][nb as usize] >> 1 as libc::c_int;
                    nb += 1;
                    nb;
                }
                na = 0 as libc::c_int;
                while na < 2 as libc::c_int {
                    *A_Q28
                        .offset(
                            na as isize,
                        ) = SKP_Silk_Transition_LP_A_Q28[ind as usize][na as usize]
                        + SKP_Silk_Transition_LP_A_Q28[(ind + 1 as libc::c_int)
                            as usize][na as usize] >> 1 as libc::c_int;
                    na += 1;
                    na;
                }
            } else {
                nb = 0 as libc::c_int;
                while nb < 3 as libc::c_int {
                    *B_Q28
                        .offset(
                            nb as isize,
                        ) = SKP_Silk_Transition_LP_B_Q28[(ind + 1 as libc::c_int)
                        as usize][nb as usize]
                        + ((SKP_Silk_Transition_LP_B_Q28[ind as usize][nb as usize]
                            - SKP_Silk_Transition_LP_B_Q28[(ind + 1 as libc::c_int)
                                as usize][nb as usize] >> 16 as libc::c_int)
                            * (((1 as libc::c_int) << 16 as libc::c_int) - fac_Q16)
                                as libc::c_short as libc::c_int
                            + ((SKP_Silk_Transition_LP_B_Q28[ind as usize][nb as usize]
                                - SKP_Silk_Transition_LP_B_Q28[(ind + 1 as libc::c_int)
                                    as usize][nb as usize] & 0xffff as libc::c_int)
                                * (((1 as libc::c_int) << 16 as libc::c_int) - fac_Q16)
                                    as libc::c_short as libc::c_int >> 16 as libc::c_int));
                    nb += 1;
                    nb;
                }
                na = 0 as libc::c_int;
                while na < 2 as libc::c_int {
                    *A_Q28
                        .offset(
                            na as isize,
                        ) = SKP_Silk_Transition_LP_A_Q28[(ind + 1 as libc::c_int)
                        as usize][na as usize]
                        + ((SKP_Silk_Transition_LP_A_Q28[ind as usize][na as usize]
                            - SKP_Silk_Transition_LP_A_Q28[(ind + 1 as libc::c_int)
                                as usize][na as usize] >> 16 as libc::c_int)
                            * (((1 as libc::c_int) << 16 as libc::c_int) - fac_Q16)
                                as libc::c_short as libc::c_int
                            + ((SKP_Silk_Transition_LP_A_Q28[ind as usize][na as usize]
                                - SKP_Silk_Transition_LP_A_Q28[(ind + 1 as libc::c_int)
                                    as usize][na as usize] & 0xffff as libc::c_int)
                                * (((1 as libc::c_int) << 16 as libc::c_int) - fac_Q16)
                                    as libc::c_short as libc::c_int >> 16 as libc::c_int));
                    na += 1;
                    na;
                }
            }
        } else {
            memcpy(
                B_Q28 as *mut libc::c_void,
                (SKP_Silk_Transition_LP_B_Q28[ind as usize]).as_ptr()
                    as *const libc::c_void,
                (3 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
            );
            memcpy(
                A_Q28 as *mut libc::c_void,
                (SKP_Silk_Transition_LP_A_Q28[ind as usize]).as_ptr()
                    as *const libc::c_void,
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
            );
        }
    } else {
        memcpy(
            B_Q28 as *mut libc::c_void,
            (SKP_Silk_Transition_LP_B_Q28[(5 as libc::c_int - 1 as libc::c_int)
                as usize])
                .as_ptr() as *const libc::c_void,
            (3 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
        );
        memcpy(
            A_Q28 as *mut libc::c_void,
            (SKP_Silk_Transition_LP_A_Q28[(5 as libc::c_int - 1 as libc::c_int)
                as usize])
                .as_ptr() as *const libc::c_void,
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_LP_variable_cutoff(
    mut psLP: *mut SKP_Silk_LP_state,
    mut out: *mut libc::c_short,
    mut in_0: *const libc::c_short,
    frame_length: libc::c_int,
) {
    let mut B_Q28: [libc::c_int; 3] = [0; 3];
    let mut A_Q28: [libc::c_int; 2] = [0; 2];
    let mut fac_Q16: libc::c_int = 0 as libc::c_int;
    let mut ind: libc::c_int = 0 as libc::c_int;
    if (*psLP).transition_frame_no > 0 as libc::c_int {
        if (*psLP).mode == 0 as libc::c_int {
            if (*psLP).transition_frame_no < 2560 as libc::c_int / 20 as libc::c_int {
                fac_Q16 = (*psLP).transition_frame_no
                    << 16 as libc::c_int - 5 as libc::c_int;
                ind = fac_Q16 >> 16 as libc::c_int;
                fac_Q16 -= ind << 16 as libc::c_int;
                SKP_Silk_LP_interpolate_filter_taps(
                    B_Q28.as_mut_ptr(),
                    A_Q28.as_mut_ptr(),
                    ind,
                    fac_Q16,
                );
                (*psLP).transition_frame_no += 1;
                (*psLP).transition_frame_no;
            } else {
                SKP_Silk_LP_interpolate_filter_taps(
                    B_Q28.as_mut_ptr(),
                    A_Q28.as_mut_ptr(),
                    5 as libc::c_int - 1 as libc::c_int,
                    0 as libc::c_int,
                );
            }
        } else if (*psLP).transition_frame_no < 5120 as libc::c_int / 20 as libc::c_int {
            fac_Q16 = 5120 as libc::c_int / 20 as libc::c_int
                - (*psLP).transition_frame_no << 16 as libc::c_int - 6 as libc::c_int;
            ind = fac_Q16 >> 16 as libc::c_int;
            fac_Q16 -= ind << 16 as libc::c_int;
            SKP_Silk_LP_interpolate_filter_taps(
                B_Q28.as_mut_ptr(),
                A_Q28.as_mut_ptr(),
                ind,
                fac_Q16,
            );
            (*psLP).transition_frame_no += 1;
            (*psLP).transition_frame_no;
        } else {
            SKP_Silk_LP_interpolate_filter_taps(
                B_Q28.as_mut_ptr(),
                A_Q28.as_mut_ptr(),
                0 as libc::c_int,
                0 as libc::c_int,
            );
        }
    }
    if (*psLP).transition_frame_no > 0 as libc::c_int {
        SKP_Silk_biquad_alt(
            in_0,
            B_Q28.as_mut_ptr(),
            A_Q28.as_mut_ptr(),
            ((*psLP).In_LP_State).as_mut_ptr(),
            out,
            frame_length,
        );
    } else {
        memcpy(
            out as *mut libc::c_void,
            in_0 as *const libc::c_void,
            (frame_length as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_short>() as libc::c_ulong),
        );
    };
}
