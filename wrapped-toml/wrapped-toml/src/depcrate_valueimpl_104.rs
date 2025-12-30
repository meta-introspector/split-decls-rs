// Generated macro for impl_104 (impl)
macro_rules! Depcrate_valueimpl_104 {
() => {
// Module: crate::value
// Provides: {"impl_104"}
// Dependencies: {}
impl < T > Index for & T where T : Index + ? Sized , { fn index < 'a > (& self , val : & 'a Value) -> Option < & 'a Value > { (* * self) . index (val) } fn index_mut < 'a > (& self , val : & 'a mut Value) -> Option < & 'a mut Value > { (* * self) . index_mut (val) } }
};
}
