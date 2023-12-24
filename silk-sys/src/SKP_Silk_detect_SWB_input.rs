#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn SKP_Silk_biquad(
        in_0: *const libc::c_short,
        B: *const libc::c_short,
        A: *const libc::c_short,
        S: *mut libc::c_int,
        out: *mut libc::c_short,
        len: libc::c_int,
    );
    fn SKP_Silk_sum_sqr_shift(
        energy: *mut libc::c_int,
        shift: *mut libc::c_int,
        x: *const libc::c_short,
        len: libc::c_int,
    );
    static SKP_Silk_SWB_detect_B_HP_Q13: [[libc::c_short; 3]; 3];
    static SKP_Silk_SWB_detect_A_HP_Q13: [[libc::c_short; 2]; 3];
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SKP_Silk_detect_SWB_state {
    pub S_HP_8_kHz: [[libc::c_int; 2]; 3],
    pub ConsecSmplsAboveThres: libc::c_int,
    pub ActiveSpeech_ms: libc::c_int,
    pub SWB_detected: libc::c_int,
    pub WB_detected: libc::c_int,
}
#[inline]
unsafe extern "C" fn SKP_min_int(mut a: libc::c_int, mut b: libc::c_int) -> libc::c_int {
    return if a < b { a } else { b };
}
#[inline]
unsafe extern "C" fn SKP_max_int(mut a: libc::c_int, mut b: libc::c_int) -> libc::c_int {
    return if a > b { a } else { b };
}
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_detect_SWB_input(
    mut psSWBdetect: *mut SKP_Silk_detect_SWB_state,
    mut samplesIn: *const libc::c_short,
    mut nSamplesIn: libc::c_int,
) {
    let mut HP_8_kHz_len: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut shift: libc::c_int = 0;
    let mut in_HP_8_kHz: [libc::c_short; 480] = [0; 480];
    let mut energy_32: libc::c_int = 0;
    HP_8_kHz_len = SKP_min_int(nSamplesIn, 20 as libc::c_int * 24 as libc::c_int);
    HP_8_kHz_len = SKP_max_int(HP_8_kHz_len, 0 as libc::c_int);
    SKP_Silk_biquad(
        samplesIn,
        (SKP_Silk_SWB_detect_B_HP_Q13[0 as libc::c_int as usize]).as_ptr(),
        (SKP_Silk_SWB_detect_A_HP_Q13[0 as libc::c_int as usize]).as_ptr(),
        ((*psSWBdetect).S_HP_8_kHz[0 as libc::c_int as usize]).as_mut_ptr(),
        in_HP_8_kHz.as_mut_ptr(),
        HP_8_kHz_len,
    );
    i = 1 as libc::c_int;
    while i < 3 as libc::c_int {
        SKP_Silk_biquad(
            in_HP_8_kHz.as_mut_ptr(),
            (SKP_Silk_SWB_detect_B_HP_Q13[i as usize]).as_ptr(),
            (SKP_Silk_SWB_detect_A_HP_Q13[i as usize]).as_ptr(),
            ((*psSWBdetect).S_HP_8_kHz[i as usize]).as_mut_ptr(),
            in_HP_8_kHz.as_mut_ptr(),
            HP_8_kHz_len,
        );
        i += 1;
        i;
    }
    SKP_Silk_sum_sqr_shift(
        &mut energy_32,
        &mut shift,
        in_HP_8_kHz.as_mut_ptr(),
        HP_8_kHz_len,
    );
    if energy_32
        > 10 as libc::c_int as libc::c_short as libc::c_int
            * HP_8_kHz_len as libc::c_short as libc::c_int >> shift
    {
        (*psSWBdetect).ConsecSmplsAboveThres += nSamplesIn;
        if (*psSWBdetect).ConsecSmplsAboveThres > 480 as libc::c_int * 15 as libc::c_int
        {
            (*psSWBdetect).SWB_detected = 1 as libc::c_int;
        }
    } else {
        (*psSWBdetect).ConsecSmplsAboveThres -= nSamplesIn;
        (*psSWBdetect)
            .ConsecSmplsAboveThres = if (*psSWBdetect).ConsecSmplsAboveThres
            > 0 as libc::c_int
        {
            (*psSWBdetect).ConsecSmplsAboveThres
        } else {
            0 as libc::c_int
        };
    }
    if (*psSWBdetect).ActiveSpeech_ms > 15000 as libc::c_int
        && (*psSWBdetect).SWB_detected == 0 as libc::c_int
    {
        (*psSWBdetect).WB_detected = 1 as libc::c_int;
    }
}
