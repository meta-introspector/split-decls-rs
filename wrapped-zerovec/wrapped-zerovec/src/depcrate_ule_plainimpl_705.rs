// Generated macro for impl_705 (impl)
macro_rules! Depcrate_ule_plainimpl_705 {
() => {
// Module: crate::ule::plain
// Provides: {"impl_705"}
// Dependencies: {}
impl AsULE for NonZeroI8 { type ULE = NonZeroU8 ; # [inline] fn to_unaligned (self) -> Self :: ULE { unsafe { core :: mem :: transmute (self) } } # [inline] fn from_unaligned (unaligned : Self :: ULE) -> Self { unsafe { core :: mem :: transmute (unaligned) } } }
};
}
