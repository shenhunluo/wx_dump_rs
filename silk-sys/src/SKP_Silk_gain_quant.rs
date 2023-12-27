#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]

use crate::{SKP_Silk_lin2log::SKP_Silk_lin2log, skp_silk_log2lin::skp_silk_log2lin, skp_s_mul_w_b};
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
                ) = i32::max(
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
            ) = skp_silk_log2lin(
                i32::min(
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

pub fn skp_silk_gains_dequant(
    gain_q16: &mut [i32],
    ind: &[i32],
    prev_ind: &mut i32,
    conditional: i32,
) {
    for k in 0..4 {
        if k == 0 && conditional == 0 {
            *prev_ind = ind[k];
        } else {
            *prev_ind += ind[k] + -4;
        }
        gain_q16[k] = skp_silk_log2lin(
            i32::min(
                skp_s_mul_w_b!(INV_SCALE_Q16,*prev_ind) + OFFSET,
                3967,
            ),
        );
    }
}

static INV_SCALE_Q16: i32  = ( 65536 * ( ( ( 86 - 6 ) * 128 ) / 6 ) ) / ( 64 - 1 );
static OFFSET: i32 = ( 6 * 128 ) / 6 + 16 * 128;
