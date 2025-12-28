macro_rules! FloatingDecimal32 {
    () => {
        pub struct FloatingDecimal32 { pub mantissa : u32 , pub exponent : i32 , }
    };
}

FloatingDecimal32!();