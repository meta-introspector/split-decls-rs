// Generated macro for impl_95 (impl)
macro_rules! Depcrate_filter_levelimpl_95 {
() => {
// Module: crate::filter::level
// Provides: {"impl_95"}
// Dependencies: {}
impl < S : Subscriber > crate :: Layer < S > for LevelFilter { fn register_callsite (& self , metadata : & 'static Metadata < 'static >) -> Interest { if self >= metadata . level () { Interest :: always () } else { Interest :: never () } } fn enabled (& self , metadata : & Metadata < '_ > , _ : crate :: layer :: Context < '_ , S >) -> bool { self >= metadata . level () } fn max_level_hint (& self) -> Option < LevelFilter > { Some (* self) } }
};
}
