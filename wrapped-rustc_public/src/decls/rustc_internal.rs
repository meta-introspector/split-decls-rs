macro_rules! rustc_internal {
    () => {
        # [doc = " Export the rustc_internal APIs. Note that this module has no stability"] # [doc = " guarantees and it is not taken into account for semver."] # [cfg (feature = "rustc_internal")] pub mod rustc_internal ;
    };
}

rustc_internal!()