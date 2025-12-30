// Generated macro for impl_45 (impl)
macro_rules! Depcrate_documentimpl_45 {
() => {
// Module: crate::document
// Provides: {"impl_45"}
// Dependencies: {}
impl < S : AsRef < str > > Document < S > { # [doc = " Allow editing of the [`DocumentMut`]"] pub fn into_mut (mut self) -> DocumentMut { self . despan () ; DocumentMut { root : self . root , trailing : self . trailing , } } }
};
}
