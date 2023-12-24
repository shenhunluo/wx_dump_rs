#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn SKP_Silk_VQ_WMat_EC_FIX(
        ind: *mut libc::c_int,
        rate_dist_Q14: *mut libc::c_int,
        in_Q14: *const libc::c_short,
        W_Q18: *const libc::c_int,
        cb_Q14: *const libc::c_short,
        cl_Q6: *const libc::c_short,
        mu_Q8: libc::c_int,
        L: libc::c_int,
    );
    static SKP_Silk_LTP_gain_BITS_Q6_ptrs: [*const libc::c_short; 3];
    static SKP_Silk_LTP_gain_middle_avg_RD_Q14: libc::c_int;
    static SKP_Silk_LTP_vq_ptrs_Q14: [*const libc::c_short; 3];
    static SKP_Silk_LTP_vq_sizes: [libc::c_int; 3];
}
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_quant_LTP_gains_FIX(
    mut B_Q14: *mut libc::c_short,
    mut cbk_index: *mut libc::c_int,
    mut periodicity_index: *mut libc::c_int,
    mut W_Q18: *const libc::c_int,
    mut mu_Q8: libc::c_int,
    mut lowComplexity: libc::c_int,
) {
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut temp_idx: [libc::c_int; 4] = [0; 4];
    let mut cbk_size: libc::c_int = 0;
    let mut cl_ptr: *const libc::c_short = 0 as *const libc::c_short;
    let mut cbk_ptr_Q14: *const libc::c_short = 0 as *const libc::c_short;
    let mut b_Q14_ptr: *const libc::c_short = 0 as *const libc::c_short;
    let mut W_Q18_ptr: *const libc::c_int = 0 as *const libc::c_int;
    let mut rate_dist_subfr: libc::c_int = 0;
    let mut rate_dist: libc::c_int = 0;
    let mut min_rate_dist: libc::c_int = 0;
    min_rate_dist = 0x7fffffff as libc::c_int;
    k = 0 as libc::c_int;
    while k < 3 as libc::c_int {
        cl_ptr = SKP_Silk_LTP_gain_BITS_Q6_ptrs[k as usize];
        cbk_ptr_Q14 = SKP_Silk_LTP_vq_ptrs_Q14[k as usize];
        cbk_size = SKP_Silk_LTP_vq_sizes[k as usize];
        W_Q18_ptr = W_Q18;
        b_Q14_ptr = B_Q14 as *const libc::c_short;
        rate_dist = 0 as libc::c_int;
        j = 0 as libc::c_int;
        while j < 4 as libc::c_int {
            SKP_Silk_VQ_WMat_EC_FIX(
                &mut *temp_idx.as_mut_ptr().offset(j as isize),
                &mut rate_dist_subfr,
                b_Q14_ptr,
                W_Q18_ptr,
                cbk_ptr_Q14,
                cl_ptr,
                mu_Q8,
                cbk_size,
            );
            rate_dist = if (rate_dist + rate_dist_subfr) as libc::c_uint
                & 0x80000000 as libc::c_uint != 0
            {
                0x7fffffff as libc::c_int
            } else {
                rate_dist + rate_dist_subfr
            };
            b_Q14_ptr = b_Q14_ptr.offset(5 as libc::c_int as isize);
            W_Q18_ptr = W_Q18_ptr.offset((5 as libc::c_int * 5 as libc::c_int) as isize);
            j += 1;
            j;
        }
        rate_dist = if (0x7fffffff as libc::c_int - 1 as libc::c_int) < rate_dist {
            0x7fffffff as libc::c_int - 1 as libc::c_int
        } else {
            rate_dist
        };
        if rate_dist < min_rate_dist {
            min_rate_dist = rate_dist;
            memcpy(
                cbk_index as *mut libc::c_void,
                temp_idx.as_mut_ptr() as *const libc::c_void,
                (4 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
            );
            *periodicity_index = k;
        }
        if lowComplexity != 0 && rate_dist < SKP_Silk_LTP_gain_middle_avg_RD_Q14 {
            break;
        }
        k += 1;
        k;
    }
    cbk_ptr_Q14 = SKP_Silk_LTP_vq_ptrs_Q14[*periodicity_index as usize];
    j = 0 as libc::c_int;
    while j < 4 as libc::c_int {
        k = 0 as libc::c_int;
        while k < 5 as libc::c_int {
            *B_Q14
                .offset(
                    (j * 5 as libc::c_int + k) as isize,
                ) = *cbk_ptr_Q14
                .offset((k + *cbk_index.offset(j as isize) * 5 as libc::c_int) as isize);
            k += 1;
            k;
        }
        j += 1;
        j;
    }
}
