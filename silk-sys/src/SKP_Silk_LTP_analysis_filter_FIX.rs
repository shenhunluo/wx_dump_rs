#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_LTP_analysis_filter_FIX(
    mut LTP_res: *mut libc::c_short,
    mut x: *const libc::c_short,
    mut LTPCoef_Q14: *const libc::c_short,
    mut pitchL: *const libc::c_int,
    mut invGains_Q16: *const libc::c_int,
    subfr_length: libc::c_int,
    pre_length: libc::c_int,
) {
    let mut x_ptr: *const libc::c_short = 0 as *const libc::c_short;
    let mut x_lag_ptr: *const libc::c_short = 0 as *const libc::c_short;
    let mut Btmp_Q14: [libc::c_short; 5] = [0; 5];
    let mut LTP_res_ptr: *mut libc::c_short = 0 as *mut libc::c_short;
    let mut k: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut LTP_est: libc::c_int = 0;
    x_ptr = x;
    LTP_res_ptr = LTP_res;
    k = 0 as libc::c_int;
    while k < 4 as libc::c_int {
        x_lag_ptr = x_ptr.offset(-(*pitchL.offset(k as isize) as isize));
        i = 0 as libc::c_int;
        while i < 5 as libc::c_int {
            Btmp_Q14[i
                as usize] = *LTPCoef_Q14.offset((k * 5 as libc::c_int + i) as isize);
            i += 1;
            i;
        }
        i = 0 as libc::c_int;
        while i < subfr_length + pre_length {
            *LTP_res_ptr.offset(i as isize) = *x_ptr.offset(i as isize);
            LTP_est = *x_lag_ptr.offset((5 as libc::c_int / 2 as libc::c_int) as isize)
                as libc::c_int * Btmp_Q14[0 as libc::c_int as usize] as libc::c_int;
            j = 1 as libc::c_int;
            while j < 5 as libc::c_int {
                LTP_est = (LTP_est as libc::c_uint)
                    .wrapping_add(
                        (*x_lag_ptr
                            .offset((5 as libc::c_int / 2 as libc::c_int - j) as isize)
                            as libc::c_int * Btmp_Q14[j as usize] as libc::c_int)
                            as libc::c_uint,
                    ) as libc::c_int;
                j += 1;
                j;
            }
            LTP_est = if 14 as libc::c_int == 1 as libc::c_int {
                (LTP_est >> 1 as libc::c_int) + (LTP_est & 1 as libc::c_int)
            } else {
                (LTP_est >> 14 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                    >> 1 as libc::c_int
            };
            *LTP_res_ptr
                .offset(
                    i as isize,
                ) = (if *x_ptr.offset(i as isize) as libc::c_int - LTP_est
                > 0x7fff as libc::c_int
            {
                0x7fff as libc::c_int
            } else if *x_ptr.offset(i as isize) as libc::c_int - LTP_est
                < 0x8000 as libc::c_int as libc::c_short as libc::c_int
            {
                0x8000 as libc::c_int as libc::c_short as libc::c_int
            } else {
                *x_ptr.offset(i as isize) as libc::c_int - LTP_est
            }) as libc::c_short;
            *LTP_res_ptr
                .offset(
                    i as isize,
                ) = ((*invGains_Q16.offset(k as isize) >> 16 as libc::c_int)
                * *LTP_res_ptr.offset(i as isize) as libc::c_int
                + ((*invGains_Q16.offset(k as isize) & 0xffff as libc::c_int)
                    * *LTP_res_ptr.offset(i as isize) as libc::c_int
                    >> 16 as libc::c_int)) as libc::c_short;
            x_lag_ptr = x_lag_ptr.offset(1);
            x_lag_ptr;
            i += 1;
            i;
        }
        LTP_res_ptr = LTP_res_ptr.offset((subfr_length + pre_length) as isize);
        x_ptr = x_ptr.offset(subfr_length as isize);
        k += 1;
        k;
    }
}
