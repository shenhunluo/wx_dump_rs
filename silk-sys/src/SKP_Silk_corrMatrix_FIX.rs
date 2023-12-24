#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn SKP_Silk_sum_sqr_shift(
        energy: *mut libc::c_int,
        shift: *mut libc::c_int,
        x: *const libc::c_short,
        len: libc::c_int,
    );
    fn SKP_Silk_inner_prod_aligned(
        inVec1: *const libc::c_short,
        inVec2: *const libc::c_short,
        len: libc::c_int,
    ) -> libc::c_int;
}
#[inline]
unsafe extern "C" fn SKP_Silk_CLZ16(mut in16: libc::c_short) -> libc::c_int {
    let mut out32: libc::c_int = 0 as libc::c_int;
    if in16 as libc::c_int == 0 as libc::c_int {
        return 16 as libc::c_int;
    }
    if in16 as libc::c_int & 0xff00 as libc::c_int != 0 {
        if in16 as libc::c_int & 0xf000 as libc::c_int != 0 {
            in16 = (in16 as libc::c_int >> 12 as libc::c_int) as libc::c_short;
        } else {
            out32 += 4 as libc::c_int;
            in16 = (in16 as libc::c_int >> 8 as libc::c_int) as libc::c_short;
        }
    } else if in16 as libc::c_int & 0xfff0 as libc::c_int != 0 {
        out32 += 8 as libc::c_int;
        in16 = (in16 as libc::c_int >> 4 as libc::c_int) as libc::c_short;
    } else {
        out32 += 12 as libc::c_int;
    }
    if in16 as libc::c_int & 0xc as libc::c_int != 0 {
        if in16 as libc::c_int & 0x8 as libc::c_int != 0 {
            return out32 + 0 as libc::c_int
        } else {
            return out32 + 1 as libc::c_int
        }
    } else if in16 as libc::c_int & 0xe as libc::c_int != 0 {
        return out32 + 2 as libc::c_int
    } else {
        return out32 + 3 as libc::c_int
    };
}
#[inline]
unsafe extern "C" fn SKP_Silk_CLZ32(mut in32: libc::c_int) -> libc::c_int {
    if in32 as libc::c_uint & 0xffff0000 as libc::c_uint != 0 {
        return SKP_Silk_CLZ16((in32 >> 16 as libc::c_int) as libc::c_short)
    } else {
        return SKP_Silk_CLZ16(in32 as libc::c_short) + 16 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_corrVector_FIX(
    mut x: *const libc::c_short,
    mut t: *const libc::c_short,
    L: libc::c_int,
    order: libc::c_int,
    mut Xt: *mut libc::c_int,
    rshifts: libc::c_int,
) {
    let mut lag: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut ptr1: *const libc::c_short = 0 as *const libc::c_short;
    let mut ptr2: *const libc::c_short = 0 as *const libc::c_short;
    let mut inner_prod: libc::c_int = 0;
    ptr1 = &*x.offset((order - 1 as libc::c_int) as isize) as *const libc::c_short;
    ptr2 = t;
    if rshifts > 0 as libc::c_int {
        lag = 0 as libc::c_int;
        while lag < order {
            inner_prod = 0 as libc::c_int;
            i = 0 as libc::c_int;
            while i < L {
                inner_prod
                    += *ptr1.offset(i as isize) as libc::c_int
                        * *ptr2.offset(i as isize) as libc::c_int >> rshifts;
                i += 1;
                i;
            }
            *Xt.offset(lag as isize) = inner_prod;
            ptr1 = ptr1.offset(-1);
            ptr1;
            lag += 1;
            lag;
        }
    } else {
        lag = 0 as libc::c_int;
        while lag < order {
            *Xt.offset(lag as isize) = SKP_Silk_inner_prod_aligned(ptr1, ptr2, L);
            ptr1 = ptr1.offset(-1);
            ptr1;
            lag += 1;
            lag;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_corrMatrix_FIX(
    mut x: *const libc::c_short,
    L: libc::c_int,
    order: libc::c_int,
    head_room: libc::c_int,
    mut XX: *mut libc::c_int,
    mut rshifts: *mut libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut lag: libc::c_int = 0;
    let mut rshifts_local: libc::c_int = 0;
    let mut head_room_rshifts: libc::c_int = 0;
    let mut energy: libc::c_int = 0;
    let mut ptr1: *const libc::c_short = 0 as *const libc::c_short;
    let mut ptr2: *const libc::c_short = 0 as *const libc::c_short;
    SKP_Silk_sum_sqr_shift(
        &mut energy,
        &mut rshifts_local,
        x,
        L + order - 1 as libc::c_int,
    );
    head_room_rshifts = if head_room - SKP_Silk_CLZ32(energy) > 0 as libc::c_int {
        head_room - SKP_Silk_CLZ32(energy)
    } else {
        0 as libc::c_int
    };
    energy = energy >> head_room_rshifts;
    rshifts_local += head_room_rshifts;
    i = 0 as libc::c_int;
    while i < order - 1 as libc::c_int {
        energy
            -= *x.offset(i as isize) as libc::c_int
                * *x.offset(i as isize) as libc::c_int >> rshifts_local;
        i += 1;
        i;
    }
    if rshifts_local < *rshifts {
        energy = energy >> *rshifts - rshifts_local;
        rshifts_local = *rshifts;
    }
    *XX.offset((0 as libc::c_int * order + 0 as libc::c_int) as isize) = energy;
    ptr1 = &*x.offset((order - 1 as libc::c_int) as isize) as *const libc::c_short;
    j = 1 as libc::c_int;
    while j < order {
        energy = energy
            - (*ptr1.offset((L - j) as isize) as libc::c_int
                * *ptr1.offset((L - j) as isize) as libc::c_int >> rshifts_local);
        energy = energy
            + (*ptr1.offset(-j as isize) as libc::c_int
                * *ptr1.offset(-j as isize) as libc::c_int >> rshifts_local);
        *XX.offset((j * order + j) as isize) = energy;
        j += 1;
        j;
    }
    ptr2 = &*x.offset((order - 2 as libc::c_int) as isize) as *const libc::c_short;
    if rshifts_local > 0 as libc::c_int {
        lag = 1 as libc::c_int;
        while lag < order {
            energy = 0 as libc::c_int;
            i = 0 as libc::c_int;
            while i < L {
                energy
                    += *ptr1.offset(i as isize) as libc::c_int
                        * *ptr2.offset(i as isize) as libc::c_int >> rshifts_local;
                i += 1;
                i;
            }
            *XX.offset((lag * order + 0 as libc::c_int) as isize) = energy;
            *XX.offset((0 as libc::c_int * order + lag) as isize) = energy;
            j = 1 as libc::c_int;
            while j < order - lag {
                energy = energy
                    - (*ptr1.offset((L - j) as isize) as libc::c_int
                        * *ptr2.offset((L - j) as isize) as libc::c_int
                        >> rshifts_local);
                energy = energy
                    + (*ptr1.offset(-j as isize) as libc::c_int
                        * *ptr2.offset(-j as isize) as libc::c_int >> rshifts_local);
                *XX.offset(((lag + j) * order + j) as isize) = energy;
                *XX.offset((j * order + (lag + j)) as isize) = energy;
                j += 1;
                j;
            }
            ptr2 = ptr2.offset(-1);
            ptr2;
            lag += 1;
            lag;
        }
    } else {
        lag = 1 as libc::c_int;
        while lag < order {
            energy = SKP_Silk_inner_prod_aligned(ptr1, ptr2, L);
            *XX.offset((lag * order + 0 as libc::c_int) as isize) = energy;
            *XX.offset((0 as libc::c_int * order + lag) as isize) = energy;
            j = 1 as libc::c_int;
            while j < order - lag {
                energy = energy
                    - *ptr1.offset((L - j) as isize) as libc::c_int
                        * *ptr2.offset((L - j) as isize) as libc::c_int;
                energy = energy
                    + *ptr1.offset(-j as isize) as libc::c_int
                        * *ptr2.offset(-j as isize) as libc::c_int;
                *XX.offset(((lag + j) * order + j) as isize) = energy;
                *XX.offset((j * order + (lag + j)) as isize) = energy;
                j += 1;
                j;
            }
            ptr2 = ptr2.offset(-1);
            ptr2;
            lag += 1;
            lag;
        }
    }
    *rshifts = rshifts_local;
}
