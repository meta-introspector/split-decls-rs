macro_rules! FORCE_UNSTABLE {
    () => {
        # [doc = " If the `-Z force-unstable-if-unmarked` flag is passed then we provide"] # [doc = " a parent stability annotation which indicates that this is private"] # [doc = " with the `rustc_private` feature. This is intended for use when"] # [doc = " compiling library and `rustc_*` crates themselves so we can leverage crates.io"] # [doc = " while maintaining the invariant that all sysroot crates are unstable"] # [doc = " by default and are unable to be used."] const FORCE_UNSTABLE : Stability = Stability { level : StabilityLevel :: Unstable { reason : UnstableReason :: Default , issue : NonZero :: new (27812) , is_soft : false , implied_by : None , old_name : None , } , feature : sym :: rustc_private , } ;
    };
}

FORCE_UNSTABLE!();