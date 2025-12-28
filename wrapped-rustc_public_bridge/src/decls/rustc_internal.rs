macro_rules! rustc_internal {
    () => {
        # [deprecated (note = "please use `rustc_public::rustc_internal` instead")] pub mod rustc_internal { }
    };
}

rustc_internal!()