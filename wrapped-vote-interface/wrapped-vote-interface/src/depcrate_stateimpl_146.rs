// Generated macro for impl_146 (impl)
macro_rules! Depcrate_stateimpl_146 {
() => {
// Module: crate::state
// Provides: {"impl_146"}
// Dependencies: {}
impl < I : Default + Copy > Default for CircBuf < I > { fn default () -> Self { Self { buf : [I :: default () ; MAX_ITEMS] , idx : MAX_ITEMS . checked_sub (1) . expect ("`MAX_ITEMS` should be positive") , is_empty : true , } } }
};
}
