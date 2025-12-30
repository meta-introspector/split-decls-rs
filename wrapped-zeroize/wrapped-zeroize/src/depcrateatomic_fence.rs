// Generated macro for atomic_fence (function)
macro_rules! Depcrateatomic_fence {
() => {
// Module: crate
// Provides: {"atomic_fence"}
// Dependencies: {}
# [doc = " Use fences to prevent accesses from being reordered before this"] # [doc = " point, which should hopefully help ensure that all accessors"] # [doc = " see zeroes after this point."] # [inline (always)] fn atomic_fence () { atomic :: compiler_fence (atomic :: Ordering :: SeqCst) ; }
};
}
