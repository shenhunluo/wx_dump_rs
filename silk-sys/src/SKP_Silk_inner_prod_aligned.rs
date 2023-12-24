#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
pub type __int64_t = libc::c_longlong;
pub type int64_t = __int64_t;
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_inner_prod_aligned(
    inVec1: *const libc::c_short,
    inVec2: *const libc::c_short,
    len: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut sum: libc::c_int = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < len {
        sum = sum
            + *inVec1.offset(i as isize) as libc::c_int
                * *inVec2.offset(i as isize) as libc::c_int;
        i += 1;
        i;
    }
    return sum;
}
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_inner_prod16_aligned_64(
    mut inVec1: *const libc::c_short,
    mut inVec2: *const libc::c_short,
    len: libc::c_int,
) -> int64_t {
    let mut i: libc::c_int = 0;
    let mut sum: int64_t = 0 as libc::c_int as int64_t;
    i = 0 as libc::c_int;
    while i < len {
        sum = sum
            + (*inVec1.offset(i as isize) as libc::c_int
                * *inVec2.offset(i as isize) as libc::c_int) as int64_t;
        i += 1;
        i;
    }
    return sum;
}
