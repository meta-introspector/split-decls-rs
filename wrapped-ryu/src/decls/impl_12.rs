macro_rules! deps {
    () => {
        Sealed!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl Sealed for f32 { # [inline] fn is_nonfinite (self) -> bool { const EXP_MASK : u32 = 0x7f800000 ; let bits = self . to_bits () ; bits & EXP_MASK == EXP_MASK } # [cold] # [cfg_attr (feature = "no-panic" , inline)] fn format_nonfinite (self) -> & 'static str { const MANTISSA_MASK : u32 = 0x007fffff ; const SIGN_MASK : u32 = 0x80000000 ; let bits = self . to_bits () ; if bits & MANTISSA_MASK != 0 { NAN } else if bits & SIGN_MASK != 0 { NEG_INFINITY } else { INFINITY } } # [inline] unsafe fn write_to_ryu_buffer (self , result : * mut u8) -> usize { raw :: format32 (self , result) } }
    };
}

impl_12!();