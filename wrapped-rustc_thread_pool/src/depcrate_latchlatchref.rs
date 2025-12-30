// Generated macro for LatchRef (struct)
macro_rules! Depcrate_latchLatchRef {
() => {
// Module: crate::latch
// Provides: {"LatchRef"}
// Dependencies: {}
# [doc = " `&L` without any implication of `dereferenceable` for `Latch::set`"] pub (super) struct LatchRef < 'a , L > { inner : * const L , marker : PhantomData < & 'a L > , }
};
}
