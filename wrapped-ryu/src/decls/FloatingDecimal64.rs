macro_rules! FloatingDecimal64 {
    () => {
        pub struct FloatingDecimal64 { pub mantissa : u64 , pub exponent : i32 , }
    };
}

FloatingDecimal64!()