macro_rules! RUSTC_SPECIFIC_FEATURES {
    () => {
        # [doc = " Features that control behaviour of rustc, rather than the codegen."] # [doc = " These exist globally and are not in the target-specific lists below."] pub const RUSTC_SPECIFIC_FEATURES : & [& str] = & ["crt-static"] ;
    };
}

RUSTC_SPECIFIC_FEATURES!();