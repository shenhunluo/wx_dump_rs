#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_resampler_private_copy(
    mut _SS: *mut libc::c_void,
    mut out: *mut libc::c_short,
    mut in_0: *const libc::c_short,
    mut inLen: libc::c_int,
) {
    memcpy(
        out as *mut libc::c_void,
        in_0 as *const libc::c_void,
        (inLen as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as libc::c_ulong),
    );
}
