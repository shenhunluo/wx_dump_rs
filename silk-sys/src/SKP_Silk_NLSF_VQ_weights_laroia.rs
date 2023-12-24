#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#[inline]
unsafe extern "C" fn SKP_min_int(mut a: libc::c_int, mut b: libc::c_int) -> libc::c_int {
    return if a < b { a } else { b };
}
#[inline]
unsafe extern "C" fn SKP_max_int(mut a: libc::c_int, mut b: libc::c_int) -> libc::c_int {
    return if a > b { a } else { b };
}
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_NLSF_VQ_weights_laroia(
    mut pNLSFW_Q6: *mut libc::c_int,
    mut pNLSF_Q15: *const libc::c_int,
    D: libc::c_int,
) {
    let mut k: libc::c_int = 0;
    let mut tmp1_int: libc::c_int = 0;
    let mut tmp2_int: libc::c_int = 0;
    tmp1_int = SKP_max_int(
        *pNLSF_Q15.offset(0 as libc::c_int as isize),
        3 as libc::c_int,
    );
    tmp1_int = ((1 as libc::c_int) << 15 as libc::c_int + 6 as libc::c_int) / tmp1_int;
    tmp2_int = SKP_max_int(
        *pNLSF_Q15.offset(1 as libc::c_int as isize)
            - *pNLSF_Q15.offset(0 as libc::c_int as isize),
        3 as libc::c_int,
    );
    tmp2_int = ((1 as libc::c_int) << 15 as libc::c_int + 6 as libc::c_int) / tmp2_int;
    *pNLSFW_Q6
        .offset(
            0 as libc::c_int as isize,
        ) = SKP_min_int(tmp1_int + tmp2_int, 0x7fff as libc::c_int);
    k = 1 as libc::c_int;
    while k < D - 1 as libc::c_int {
        tmp1_int = SKP_max_int(
            *pNLSF_Q15.offset((k + 1 as libc::c_int) as isize)
                - *pNLSF_Q15.offset(k as isize),
            3 as libc::c_int,
        );
        tmp1_int = ((1 as libc::c_int) << 15 as libc::c_int + 6 as libc::c_int)
            / tmp1_int;
        *pNLSFW_Q6
            .offset(
                k as isize,
            ) = SKP_min_int(tmp1_int + tmp2_int, 0x7fff as libc::c_int);
        tmp2_int = SKP_max_int(
            *pNLSF_Q15.offset((k + 2 as libc::c_int) as isize)
                - *pNLSF_Q15.offset((k + 1 as libc::c_int) as isize),
            3 as libc::c_int,
        );
        tmp2_int = ((1 as libc::c_int) << 15 as libc::c_int + 6 as libc::c_int)
            / tmp2_int;
        *pNLSFW_Q6
            .offset(
                (k + 1 as libc::c_int) as isize,
            ) = SKP_min_int(tmp1_int + tmp2_int, 0x7fff as libc::c_int);
        k += 2 as libc::c_int;
    }
    tmp1_int = SKP_max_int(
        ((1 as libc::c_int) << 15 as libc::c_int)
            - *pNLSF_Q15.offset((D - 1 as libc::c_int) as isize),
        3 as libc::c_int,
    );
    tmp1_int = ((1 as libc::c_int) << 15 as libc::c_int + 6 as libc::c_int) / tmp1_int;
    *pNLSFW_Q6
        .offset(
            (D - 1 as libc::c_int) as isize,
        ) = SKP_min_int(tmp1_int + tmp2_int, 0x7fff as libc::c_int);
}
