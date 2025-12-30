// Generated macro for impl_21 (impl)
macro_rules! Depcrateimpl_21 {
() => {
// Module: crate
// Provides: {"impl_21"}
// Dependencies: {}
impl From < Tai64 > for Tai64N { # [doc = " Remove the nanosecond component from a TAI64N value"] fn from (other : Tai64) -> Self { Tai64N (other , 0) } }
};
}
