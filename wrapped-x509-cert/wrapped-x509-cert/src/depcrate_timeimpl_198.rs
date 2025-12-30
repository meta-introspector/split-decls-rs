// Generated macro for impl_198 (impl)
macro_rules! Depcrate_timeimpl_198 {
() => {
// Module: crate::time
// Provides: {"impl_198"}
// Dependencies: {}
impl < P > Validity < P > where P : Profile , { # [doc = " Creates a `Validity` with the provided bounds"] pub const fn new (not_before : Time , not_after : Time) -> Self { Self { not_before , not_after , _profile : PhantomData , } } # [doc = " Creates a `Validity` which starts now and lasts for `duration`."] # [cfg (feature = "std")] pub fn from_now (duration : Duration) -> der :: Result < Self > { let now = SystemTime :: now () ; let then = now + duration ; Ok (Self { not_before : Time :: try_from (now) ? , not_after : Time :: try_from (then) ? , _profile : PhantomData , }) } # [doc = " Creates a `Validity` which starts now and does not expire."] # [cfg (all (feature = "std" , feature = "hazmat"))] pub fn infinity () -> der :: Result < Self > { let now = SystemTime :: now () ; Ok (Self { not_before : Time :: try_from (now) ? , not_after : Time :: INFINITY , _profile : PhantomData , }) } }
};
}
