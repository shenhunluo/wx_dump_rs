#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_log2lin(inLog_Q7: libc::c_int) -> libc::c_int {
    let mut out: libc::c_int = 0;
    let mut frac_Q7: libc::c_int = 0;
    if inLog_Q7 < 0 as libc::c_int {
        return 0 as libc::c_int
    } else if inLog_Q7 >= (31 as libc::c_int) << 7 as libc::c_int {
        return 0x7fffffff as libc::c_int
    }
    out = (1 as libc::c_int) << (inLog_Q7 >> 7 as libc::c_int);
    frac_Q7 = inLog_Q7 & 0x7f as libc::c_int;
    if inLog_Q7 < 2048 as libc::c_int {
        out = out
            + (out
                * (frac_Q7
                    + ((frac_Q7 * (128 as libc::c_int - frac_Q7) >> 16 as libc::c_int)
                        * -(174 as libc::c_int) as libc::c_short as libc::c_int
                        + ((frac_Q7 * (128 as libc::c_int - frac_Q7)
                            & 0xffff as libc::c_int)
                            * -(174 as libc::c_int) as libc::c_short as libc::c_int
                            >> 16 as libc::c_int))) >> 7 as libc::c_int);
    } else {
        out = out
            + (out >> 7 as libc::c_int)
                * (frac_Q7
                    + ((frac_Q7 * (128 as libc::c_int - frac_Q7) >> 16 as libc::c_int)
                        * -(174 as libc::c_int) as libc::c_short as libc::c_int
                        + ((frac_Q7 * (128 as libc::c_int - frac_Q7)
                            & 0xffff as libc::c_int)
                            * -(174 as libc::c_int) as libc::c_short as libc::c_int
                            >> 16 as libc::c_int)));
    }
    return out;
}
