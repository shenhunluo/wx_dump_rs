#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
pub type intptr_t = libc::c_long;
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_sum_sqr_shift(
    mut energy: *mut libc::c_int,
    mut shift: *mut libc::c_int,
    mut x: *const libc::c_short,
    mut len: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut shft: libc::c_int = 0;
    let mut in32: libc::c_int = 0;
    let mut nrg_tmp: libc::c_int = 0;
    let mut nrg: libc::c_int = 0;
    if (x as intptr_t & 2 as libc::c_int as intptr_t) as libc::c_int != 0 as libc::c_int
    {
        nrg = *x.offset(0 as libc::c_int as isize) as libc::c_int
            * *x.offset(0 as libc::c_int as isize) as libc::c_int;
        i = 1 as libc::c_int;
    } else {
        nrg = 0 as libc::c_int;
        i = 0 as libc::c_int;
    }
    shft = 0 as libc::c_int;
    len -= 1;
    while i < len {
        in32 = *(&*x.offset(i as isize) as *const libc::c_short as *mut libc::c_int);
        nrg = (nrg as libc::c_uint)
            .wrapping_add(
                (in32 as libc::c_short as libc::c_int
                    * in32 as libc::c_short as libc::c_int) as libc::c_uint,
            ) as libc::c_int;
        nrg = (nrg as libc::c_uint)
            .wrapping_add(
                ((in32 >> 16 as libc::c_int) * (in32 >> 16 as libc::c_int))
                    as libc::c_uint,
            ) as libc::c_int;
        i += 2 as libc::c_int;
        if !(nrg < 0 as libc::c_int) {
            continue;
        }
        nrg = (nrg as libc::c_uint >> 2 as libc::c_int) as libc::c_int;
        shft = 2 as libc::c_int;
        break;
    }
    while i < len {
        in32 = *(&*x.offset(i as isize) as *const libc::c_short as *mut libc::c_int);
        nrg_tmp = in32 as libc::c_short as libc::c_int
            * in32 as libc::c_short as libc::c_int;
        nrg_tmp = (nrg_tmp as libc::c_uint)
            .wrapping_add(
                ((in32 >> 16 as libc::c_int) * (in32 >> 16 as libc::c_int))
                    as libc::c_uint,
            ) as libc::c_int;
        nrg = (nrg as libc::c_uint).wrapping_add(nrg_tmp as libc::c_uint >> shft)
            as libc::c_int;
        if nrg < 0 as libc::c_int {
            nrg = (nrg as libc::c_uint >> 2 as libc::c_int) as libc::c_int;
            shft += 2 as libc::c_int;
        }
        i += 2 as libc::c_int;
    }
    if i == len {
        nrg_tmp = *x.offset(i as isize) as libc::c_int
            * *x.offset(i as isize) as libc::c_int;
        nrg = nrg + (nrg_tmp >> shft);
    }
    if nrg as libc::c_uint & 0xc0000000 as libc::c_uint != 0 {
        nrg = (nrg as libc::c_uint >> 2 as libc::c_int) as libc::c_int;
        shft += 2 as libc::c_int;
    }
    *shift = shft;
    *energy = nrg;
}
