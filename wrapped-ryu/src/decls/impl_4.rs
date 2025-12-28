macro_rules! deps {
    () => {
        Float!();
        Buffer!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        impl Buffer { # [doc = " This is a cheap operation; you don't need to worry about reusing buffers"] # [doc = " for efficiency."] # [inline] # [cfg_attr (feature = "no-panic" , no_panic)] pub fn new () -> Self { let bytes = [MaybeUninit :: < u8 > :: uninit () ; 24] ; Buffer { bytes } } # [doc = " Print a floating point number into this buffer and return a reference to"] # [doc = " its string representation within the buffer."] # [doc = ""] # [doc = " # Special cases"] # [doc = ""] # [doc = " This function formats NaN as the string \"NaN\", positive infinity as"] # [doc = " \"inf\", and negative infinity as \"-inf\" to match std::fmt."] # [doc = ""] # [doc = " If your input is known to be finite, you may get better performance by"] # [doc = " calling the `format_finite` method instead of `format` to avoid the"] # [doc = " checks for special cases."] # [cfg_attr (feature = "no-panic" , inline)] # [cfg_attr (feature = "no-panic" , no_panic)] pub fn format < F : Float > (& mut self , f : F) -> & str { if f . is_nonfinite () { f . format_nonfinite () } else { self . format_finite (f) } } # [doc = " Print a floating point number into this buffer and return a reference to"] # [doc = " its string representation within the buffer."] # [doc = ""] # [doc = " # Special cases"] # [doc = ""] # [doc = " This function **does not** check for NaN or infinity. If the input"] # [doc = " number is not a finite float, the printed representation will be some"] # [doc = " correctly formatted but unspecified numerical value."] # [doc = ""] # [doc = " Please check [`is_finite`] yourself before calling this function, or"] # [doc = " check [`is_nan`] and [`is_infinite`] and handle those cases yourself."] # [doc = ""] # [doc = " [`is_finite`]: f64::is_finite"] # [doc = " [`is_nan`]: f64::is_nan"] # [doc = " [`is_infinite`]: f64::is_infinite"] # [inline] # [cfg_attr (feature = "no-panic" , no_panic)] pub fn format_finite < F : Float > (& mut self , f : F) -> & str { unsafe { let n = f . write_to_ryu_buffer (self . bytes . as_mut_ptr () as * mut u8) ; debug_assert ! (n <= self . bytes . len ()) ; let slice = slice :: from_raw_parts (self . bytes . as_ptr () as * const u8 , n) ; str :: from_utf8_unchecked (slice) } } }
    };
}

impl_4!()