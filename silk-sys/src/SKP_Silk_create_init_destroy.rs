#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
use crate::SKP_Silk_dec_API::SKP_Silk_decoder_state;
use crate::SKP_Silk_decoder_set_fs::SKP_Silk_decoder_set_fs;
use crate::SKP_Silk_CNG::SKP_Silk_CNG_Reset;
use crate::SKP_Silk_PLC::SKP_Silk_PLC_Reset;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct SKP_Silk_NLSF_CBS {
    pub nVectors: libc::c_int,
    pub CB_NLSF_Q15: *const libc::c_short,
    pub Rates_Q5: *const libc::c_short,
}
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_init_decoder(
    psDec: &mut SKP_Silk_decoder_state,
) -> libc::c_int {
    SKP_Silk_decoder_set_fs(psDec, 24 as libc::c_int);
    (*psDec).first_frame_after_reset = 1 as libc::c_int;
    (*psDec).prev_inv_gain_Q16 = 65536 as libc::c_int;
    SKP_Silk_CNG_Reset(psDec);
    SKP_Silk_PLC_Reset(psDec);
    return 0 as libc::c_int;
}
