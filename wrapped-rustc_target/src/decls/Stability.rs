macro_rules! deps {
    () => {
        ABI!();
    };
}

macro_rules! Stability {
    () => {
        deps!();
        # [doc = " Stability information for target features."] # [derive (Debug , Copy , Clone)] pub enum Stability { # [doc = " This target feature is stable, it can be used in `#[target_feature]` and"] # [doc = " `#[cfg(target_feature)]`."] Stable , # [doc = " This target feature is unstable. It is only present in `#[cfg(target_feature)]` on"] # [doc = " nightly and using it in `#[target_feature]` requires enabling the given nightly feature."] Unstable (# [doc = " This must be a *language* feature, or else rustc will ICE when reporting a missing"] # [doc = " feature gate!"] Symbol ,) , # [doc = " This feature can not be set via `-Ctarget-feature` or `#[target_feature]`, it can only be"] # [doc = " set in the target spec. It is never set in `cfg(target_feature)`. Used in"] # [doc = " particular for features are actually ABI configuration flags (not all targets are as nice as"] # [doc = " RISC-V and have an explicit way to set the ABI separate from target features)."] Forbidden { reason : & 'static str } , }
    };
}

Stability!();