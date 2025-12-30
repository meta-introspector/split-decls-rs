// Generated macro for impl_394 (impl)
macro_rules! Depcrate_revisionimpl_394 {
() => {
// Module: crate::revision
// Provides: {"impl_394"}
// Dependencies: {}
impl Revision { # [inline] pub (crate) fn max () -> Self { Self :: from (usize :: MAX) } # [inline] pub (crate) const fn start () -> Self { Self { generation : unsafe { NonZeroUsize :: new_unchecked (START) } , } } # [inline] pub (crate) fn from (g : usize) -> Self { Self { generation : NonZeroUsize :: new (g) . unwrap () , } } # [inline] pub (crate) fn from_opt (g : usize) -> Option < Self > { NonZeroUsize :: new (g) . map (| generation | Self { generation }) } # [inline] pub (crate) fn next (self) -> Revision { Self :: from (self . generation . get () + 1) } # [inline] pub (crate) fn as_usize (self) -> usize { self . generation . get () } }
};
}
