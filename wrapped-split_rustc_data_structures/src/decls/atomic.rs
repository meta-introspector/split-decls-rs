macro_rules! atomic {
    () => {
        # [doc = " Keep the conditional imports together in a submodule, so that import-sorting"] # [doc = " doesn't split them up."] mod atomic { # [cfg (target_has_atomic = "64")] pub use std :: sync :: atomic :: AtomicU64 ; # [cfg (not (target_has_atomic = "64"))] pub use portable_atomic :: AtomicU64 ; }
    };
}

atomic!();