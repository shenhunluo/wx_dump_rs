
#[macro_export]
macro_rules! skp_s_mul_wb {
    ($a32:expr,$b32:expr) => {{
        (((($a32) >> 16) * ($b32 as i16) as i32) + (((($a32) & 0x0000FFFF) * ($b32 as i16) as i32) >> 16))
    }}
}

#[macro_export]
macro_rules! skp_l_shift {
    ($a:expr,$shift:expr) => {{
        crate::skp_l_shift_32!($a,$shift)
    }}
}

#[macro_export]
macro_rules! skp_l_shift_32 {
    ($a:expr,$shift:expr) => {{
        $a << $shift
    }}
}

#[macro_export]
macro_rules! skp_r_shift {
    ($a:expr,$shift:expr) => {{
        crate::skp_r_shift_32!($a,$shift)
    }}
}

#[macro_export]
macro_rules! skp_r_shift_32 {
    ($a:expr,$shift:expr) => {{
        $a >> $shift
    }}
}

#[macro_export]
macro_rules! skp_limit_32 {
    ($a:expr,$limit1:expr,$limit2:expr) => {{
        crate::skp_limit!($a,$limit1,$limit2)
    }}
}

#[macro_export]
macro_rules! skp_limit {
    ($a:expr,$limit1:expr,$limit2:expr) => {{
        if $limit1 > $limit2 {
            if $a > $limit1 {
                $limit1
            } else {
                if $a < $limit2 {
                    $limit2
                } else {
                    $a
                }
            }
        } else {
            if $a > $limit2 {
                $limit2
            } else {
                if $a < $limit1 {
                    $limit1
                } else {
                    $a
                }
            }
        }
    }}
}

#[macro_export]
macro_rules! skp_r_shift_round {
    ($a:expr,$shift:expr) => {{
        if $shift == 1 {
            ( $a >> 1 ) + ( $a & 1 )
        } else {
            (($a >> ($shift - 1)) + 1) >> 1
        }
    }}
}

#[macro_export]
macro_rules! skp_s_mul_b_b {
    ($a32:expr,$b32:expr) => {{
        ($a32 as i16 as i32) * ($b32 as i16 as i32)
    }}
}