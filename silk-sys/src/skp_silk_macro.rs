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
macro_rules! skp_add_r_shift {
    ($a:expr,$b:expr,$shift:expr) => {{
        $a + crate::skp_r_shift!($b,$shift)
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
            ($a >> ($shift - 1)) + 1 >> 1
        }
    }}
}

#[macro_export]
macro_rules! skp_s_mul_l {
    ($a32:expr,$b32:expr) => {{
        ($a32 as i64) * ($b32 as i64)
    }}
}

#[macro_export]
macro_rules! skp_s_m_mul {
    ($a32:expr,$b32:expr) => {{
        crate::skp_r_shift!(crate::skp_s_mul_l!($a32, $b32),32) as i32
    }}
}

#[macro_export]
macro_rules! skp_s_mul_b_b {
    ($a32:expr,$b32:expr) => {{
        ($a32 as i16 as i32) * ($b32 as i16 as i32)
    }}
}

#[macro_export]
macro_rules! skp_s_mul_w_b {
    ($a32:expr,$b32:expr) => {{
        (($a32 >> 16) * ($b32 as i16 as i32)) + ((($a32 & 0xFFFF) * ($b32 as i16 as i32)) >> 16)
    }}
}

#[macro_export]
macro_rules! skp_s_mul_w_w {
    ($a32:expr,$b32:expr) => {{
        crate::skp_mla!(
            crate::skp_s_mul_w_b!($a32, $b32),
            $a32,
            crate::skp_r_shift_round!($b32,16)
        )
    }};
}

#[macro_export]
macro_rules! skp_mla {
    ($a32:expr,$b32:expr,$c32:expr) => {{
        ($a32) + ($b32 * $c32)
    }}
}

#[macro_export]
macro_rules! skp_s_mla_w_b {
    ($a32:expr,$b32:expr,$c32:expr) => {{
        ($a32 + ((($b32 >> 16) * ($c32 as i16 as i32)) + ((($b32 & 0x0000FFFF) * ($c32 as i16 as i32)) >> 16)))
    }}
}

#[macro_export]
macro_rules! skp_s_mla_w_w {
    ($a32:expr,$b32:expr,$c32:expr) => {{
        crate::skp_mla!(crate::skp_s_mla_w_b!($a32,$b32,$c32), $b32, crate::skp_r_shift_round!($c32, 16))
    }}
}

#[macro_export]
macro_rules! skp_dec_map {
    ($a:expr) => {{
        crate::skp_l_shift!($a,1)-1
    }}
}

#[macro_export]
macro_rules! skp_sat_16 {
    ($a:expr,$ty:ty) => {{
        if $a > i16::MAX as $ty {
            i16::MAX as $ty
        } else {
            if $a < i16::MIN as $ty {
                i16::MIN as $ty
            } else {
                $a as $ty
            }
        }
    }}
}

#[macro_export]
macro_rules! skp_l_shift_sat_32 {
    ($a:expr,$shift:expr) => {{
        crate::skp_l_shift!(crate::skp_limit_32!($a, crate::skp_r_shift!(i32::MIN,$shift), crate::skp_r_shift!(i32::MAX,$shift)),$shift)
    }}
}
