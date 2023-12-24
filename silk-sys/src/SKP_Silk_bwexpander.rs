#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_bwexpander(
    mut ar: *mut libc::c_short,
    d: libc::c_int,
    mut chirp_Q16: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut chirp_minus_one_Q16: libc::c_int = 0;
    chirp_minus_one_Q16 = chirp_Q16 - 65536 as libc::c_int;
    i = 0 as libc::c_int;
    while i < d - 1 as libc::c_int {
        *ar
            .offset(
                i as isize,
            ) = (if 16 as libc::c_int == 1 as libc::c_int {
            (chirp_Q16 * *ar.offset(i as isize) as libc::c_int >> 1 as libc::c_int)
                + (chirp_Q16 * *ar.offset(i as isize) as libc::c_int & 1 as libc::c_int)
        } else {
            (chirp_Q16 * *ar.offset(i as isize) as libc::c_int
                >> 16 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                >> 1 as libc::c_int
        }) as libc::c_short;
        chirp_Q16
            += if 16 as libc::c_int == 1 as libc::c_int {
                (chirp_Q16 * chirp_minus_one_Q16 >> 1 as libc::c_int)
                    + (chirp_Q16 * chirp_minus_one_Q16 & 1 as libc::c_int)
            } else {
                (chirp_Q16 * chirp_minus_one_Q16 >> 16 as libc::c_int - 1 as libc::c_int)
                    + 1 as libc::c_int >> 1 as libc::c_int
            };
        i += 1;
    }
    *ar
        .offset(
            (d - 1 as libc::c_int) as isize,
        ) = (if 16 as libc::c_int == 1 as libc::c_int {
        (chirp_Q16 * *ar.offset((d - 1 as libc::c_int) as isize) as libc::c_int
            >> 1 as libc::c_int)
            + (chirp_Q16 * *ar.offset((d - 1 as libc::c_int) as isize) as libc::c_int
                & 1 as libc::c_int)
    } else {
        (chirp_Q16 * *ar.offset((d - 1 as libc::c_int) as isize) as libc::c_int
            >> 16 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
            >> 1 as libc::c_int
    }) as libc::c_short;
}
