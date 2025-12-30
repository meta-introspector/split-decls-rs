// Generated macro for TransmuteRefDst (trait)
macro_rules! Depcrate_util_macro_utilTransmuteRefDst {
() => {
// Module: crate::util::macro_util
// Provides: {"TransmuteRefDst"}
// Dependencies: {}
pub trait TransmuteRefDst < 'a > { type Dst : ? Sized ; # [must_use] fn transmute_ref (self) -> & 'a Self :: Dst ; }
};
}
