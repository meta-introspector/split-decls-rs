// Generated macro for impl_84 (impl)
macro_rules! Depcrate_arc_borrowimpl_84 {
() => {
// Module: crate::arc_borrow
// Provides: {"impl_84"}
// Dependencies: {}
impl < 'a , T > Deref for ArcBorrow < 'a , T > { type Target = T ; # [inline] fn deref (& self) -> & T { self . get () } }
};
}
