// Generated macro for impl_945 (impl)
macro_rules! Depcrate_set_statusimpl_945 {
() => {
// Module: crate::set_status
// Provides: {"impl_945"}
// Dependencies: {}
impl < S > Layer < S > for SetStatusLayer { type Service = SetStatus < S > ; fn layer (& self , inner : S) -> Self :: Service { SetStatus :: new (inner , self . status) } }
};
}
