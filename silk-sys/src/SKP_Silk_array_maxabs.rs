#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_int16_array_maxabs(
    mut vec: *const libc::c_short,
    len: libc::c_int,
) -> libc::c_short {
    let mut max: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut lvl: libc::c_int = 0 as libc::c_int;
    let mut ind: libc::c_int = 0;
    if len == 0 as libc::c_int {
        return 0 as libc::c_int as libc::c_short;
    }
    ind = len - 1 as libc::c_int;
    max = *vec.offset(ind as isize) as libc::c_int
        * *vec.offset(ind as isize) as libc::c_int;
    i = len - 2 as libc::c_int;
    while i >= 0 as libc::c_int {
        lvl = *vec.offset(i as isize) as libc::c_int
            * *vec.offset(i as isize) as libc::c_int;
        if lvl > max {
            max = lvl;
            ind = i;
        }
        i -= 1;
        i;
    }
    if max >= 1073676289 as libc::c_int {
        return 0x7fff as libc::c_int as libc::c_short
    } else if (*vec.offset(ind as isize) as libc::c_int) < 0 as libc::c_int {
        return -(*vec.offset(ind as isize) as libc::c_int) as libc::c_short
    } else {
        return *vec.offset(ind as isize)
    };
}
