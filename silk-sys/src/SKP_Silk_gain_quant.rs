#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]

use crate::{SKP_Silk_lin2log::SKP_Silk_lin2log, SKP_Silk_log2lin::SKP_Silk_log2lin};
#[inline]
unsafe extern "C" fn SKP_min_32(mut a: libc::c_int, mut b: libc::c_int) -> libc::c_int {
    return if a < b { a } else { b };
}
#[inline]
unsafe extern "C" fn SKP_max_int(mut a: libc::c_int, mut b: libc::c_int) -> libc::c_int {
    return if a > b { a } else { b };
}
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_gains_quant(
    mut ind: *mut libc::c_int,
    mut gain_Q16: *mut libc::c_int,
    mut prev_ind: *mut libc::c_int,
    conditional: libc::c_int,
) {
    let mut k: libc::c_int = 0;
    k = 0 as libc::c_int;
    while k < 4 as libc::c_int {
        *ind
            .offset(
                k as isize,
            ) = (65536 as libc::c_int * (64 as libc::c_int - 1 as libc::c_int)
            / ((86 as libc::c_int - 6 as libc::c_int) * 128 as libc::c_int
                / 6 as libc::c_int) >> 16 as libc::c_int)
            * (SKP_Silk_lin2log(*gain_Q16.offset(k as isize))
                - (6 as libc::c_int * 128 as libc::c_int / 6 as libc::c_int
                    + 16 as libc::c_int * 128 as libc::c_int)) as libc::c_short
                as libc::c_int
            + ((65536 as libc::c_int * (64 as libc::c_int - 1 as libc::c_int)
                / ((86 as libc::c_int - 6 as libc::c_int) * 128 as libc::c_int
                    / 6 as libc::c_int) & 0xffff as libc::c_int)
                * (SKP_Silk_lin2log(*gain_Q16.offset(k as isize))
                    - (6 as libc::c_int * 128 as libc::c_int / 6 as libc::c_int
                        + 16 as libc::c_int * 128 as libc::c_int)) as libc::c_short
                    as libc::c_int >> 16 as libc::c_int);
        if *ind.offset(k as isize) < *prev_ind {
            let ref mut fresh0 = *ind.offset(k as isize);
            *fresh0 += 1;
        }
        if k == 0 as libc::c_int && conditional == 0 as libc::c_int {
            *ind
                .offset(
                    k as isize,
                ) = if 0 as libc::c_int > 64 as libc::c_int - 1 as libc::c_int {
                if *ind.offset(k as isize) > 0 as libc::c_int {
                    0 as libc::c_int
                } else if *ind.offset(k as isize) < 64 as libc::c_int - 1 as libc::c_int
                {
                    64 as libc::c_int - 1 as libc::c_int
                } else {
                    *ind.offset(k as isize)
                }
            } else if *ind.offset(k as isize) > 64 as libc::c_int - 1 as libc::c_int {
                64 as libc::c_int - 1 as libc::c_int
            } else if *ind.offset(k as isize) < 0 as libc::c_int {
                0 as libc::c_int
            } else {
                *ind.offset(k as isize)
            };
            *ind
                .offset(
                    k as isize,
                ) = SKP_max_int(
                *ind.offset(k as isize),
                *prev_ind + -(4 as libc::c_int),
            );
            *prev_ind = *ind.offset(k as isize);
        } else {
            *ind
                .offset(
                    k as isize,
                ) = if -(4 as libc::c_int) > 40 as libc::c_int {
                if *ind.offset(k as isize) - *prev_ind > -(4 as libc::c_int) {
                    -(4 as libc::c_int)
                } else if *ind.offset(k as isize) - *prev_ind < 40 as libc::c_int {
                    40 as libc::c_int
                } else {
                    *ind.offset(k as isize) - *prev_ind
                }
            } else if *ind.offset(k as isize) - *prev_ind > 40 as libc::c_int {
                40 as libc::c_int
            } else if *ind.offset(k as isize) - *prev_ind < -(4 as libc::c_int) {
                -(4 as libc::c_int)
            } else {
                *ind.offset(k as isize) - *prev_ind
            };
            *prev_ind += *ind.offset(k as isize);
            *ind.offset(k as isize) -= -(4 as libc::c_int);
        }
        *gain_Q16
            .offset(
                k as isize,
            ) = SKP_Silk_log2lin(
            SKP_min_32(
                (65536 as libc::c_int
                    * ((86 as libc::c_int - 6 as libc::c_int) * 128 as libc::c_int
                        / 6 as libc::c_int) / (64 as libc::c_int - 1 as libc::c_int)
                    >> 16 as libc::c_int) * *prev_ind as libc::c_short as libc::c_int
                    + ((65536 as libc::c_int
                        * ((86 as libc::c_int - 6 as libc::c_int) * 128 as libc::c_int
                            / 6 as libc::c_int) / (64 as libc::c_int - 1 as libc::c_int)
                        & 0xffff as libc::c_int)
                        * *prev_ind as libc::c_short as libc::c_int >> 16 as libc::c_int)
                    + (6 as libc::c_int * 128 as libc::c_int / 6 as libc::c_int
                        + 16 as libc::c_int * 128 as libc::c_int),
                3967 as libc::c_int,
            ),
        );
        k += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_gains_dequant(
    mut gain_Q16: *mut libc::c_int,
    mut ind: *const libc::c_int,
    mut prev_ind: *mut libc::c_int,
    conditional: libc::c_int,
) {
    let mut k: libc::c_int = 0;
    k = 0 as libc::c_int;
    while k < 4 as libc::c_int {
        if k == 0 as libc::c_int && conditional == 0 as libc::c_int {
            *prev_ind = *ind.offset(k as isize);
        } else {
            *prev_ind += *ind.offset(k as isize) + -(4 as libc::c_int);
        }
        *gain_Q16
            .offset(
                k as isize,
            ) = SKP_Silk_log2lin(
            SKP_min_32(
                (65536 as libc::c_int
                    * ((86 as libc::c_int - 6 as libc::c_int) * 128 as libc::c_int
                        / 6 as libc::c_int) / (64 as libc::c_int - 1 as libc::c_int)
                    >> 16 as libc::c_int) * *prev_ind as libc::c_short as libc::c_int
                    + ((65536 as libc::c_int
                        * ((86 as libc::c_int - 6 as libc::c_int) * 128 as libc::c_int
                            / 6 as libc::c_int) / (64 as libc::c_int - 1 as libc::c_int)
                        & 0xffff as libc::c_int)
                        * *prev_ind as libc::c_short as libc::c_int >> 16 as libc::c_int)
                    + (6 as libc::c_int * 128 as libc::c_int / 6 as libc::c_int
                        + 16 as libc::c_int * 128 as libc::c_int),
                3967 as libc::c_int,
            ),
        );
        k += 1;
    }
}
