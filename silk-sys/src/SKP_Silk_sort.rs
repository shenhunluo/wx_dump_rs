#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_insertion_sort_increasing(
    mut a: *mut libc::c_int,
    mut index: *mut libc::c_int,
    L: libc::c_int,
    K: libc::c_int,
) {
    let mut value: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < K {
        *index.offset(i as isize) = i;
        i += 1;
    }
    i = 1 as libc::c_int;
    while i < K {
        value = *a.offset(i as isize);
        j = i - 1 as libc::c_int;
        while j >= 0 as libc::c_int && value < *a.offset(j as isize) {
            *a.offset((j + 1 as libc::c_int) as isize) = *a.offset(j as isize);
            *index.offset((j + 1 as libc::c_int) as isize) = *index.offset(j as isize);
            j -= 1;
        }
        *a.offset((j + 1 as libc::c_int) as isize) = value;
        *index.offset((j + 1 as libc::c_int) as isize) = i;
        i += 1;
    }
    i = K;
    while i < L {
        value = *a.offset(i as isize);
        if value < *a.offset((K - 1 as libc::c_int) as isize) {
            j = K - 2 as libc::c_int;
            while j >= 0 as libc::c_int && value < *a.offset(j as isize) {
                *a.offset((j + 1 as libc::c_int) as isize) = *a.offset(j as isize);
                *index
                    .offset((j + 1 as libc::c_int) as isize) = *index.offset(j as isize);
                j -= 1;
            }
            *a.offset((j + 1 as libc::c_int) as isize) = value;
            *index.offset((j + 1 as libc::c_int) as isize) = i;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_insertion_sort_decreasing_int16(
    mut a: *mut libc::c_short,
    mut index: *mut libc::c_int,
    L: libc::c_int,
    K: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut value: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < K {
        *index.offset(i as isize) = i;
        i += 1;
    }
    i = 1 as libc::c_int;
    while i < K {
        value = *a.offset(i as isize) as libc::c_int;
        j = i - 1 as libc::c_int;
        while j >= 0 as libc::c_int && value > *a.offset(j as isize) as libc::c_int {
            *a.offset((j + 1 as libc::c_int) as isize) = *a.offset(j as isize);
            *index.offset((j + 1 as libc::c_int) as isize) = *index.offset(j as isize);
            j -= 1;
        }
        *a.offset((j + 1 as libc::c_int) as isize) = value as libc::c_short;
        *index.offset((j + 1 as libc::c_int) as isize) = i;
        i += 1;
    }
    i = K;
    while i < L {
        value = *a.offset(i as isize) as libc::c_int;
        if value > *a.offset((K - 1 as libc::c_int) as isize) as libc::c_int {
            j = K - 2 as libc::c_int;
            while j >= 0 as libc::c_int && value > *a.offset(j as isize) as libc::c_int {
                *a.offset((j + 1 as libc::c_int) as isize) = *a.offset(j as isize);
                *index
                    .offset((j + 1 as libc::c_int) as isize) = *index.offset(j as isize);
                j -= 1;
            }
            *a.offset((j + 1 as libc::c_int) as isize) = value as libc::c_short;
            *index.offset((j + 1 as libc::c_int) as isize) = i;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_insertion_sort_increasing_all_values(
    mut a: *mut libc::c_int,
    L: libc::c_int,
) {
    let mut value: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i < L {
        value = *a.offset(i as isize);
        j = i - 1 as libc::c_int;
        while j >= 0 as libc::c_int && value < *a.offset(j as isize) {
            *a.offset((j + 1 as libc::c_int) as isize) = *a.offset(j as isize);
            j -= 1;
        }
        *a.offset((j + 1 as libc::c_int) as isize) = value;
        i += 1;
    }
}
