macro_rules! derive_from_zeroes {
    () => {
        # [doc = " Deprecated: prefer [`FromZeros`] instead."] # [deprecated (since = "0.8.0" , note = "`FromZeroes` was renamed to `FromZeros`")] # [doc (hidden)] # [proc_macro_derive (FromZeroes)] pub fn derive_from_zeroes (ts : proc_macro :: TokenStream) -> proc_macro :: TokenStream { derive_from_zeros (ts) }
    };
}

derive_from_zeroes!()