// Generated macro for impl_202 (impl)
macro_rules! Depcrate_text_abstractionimpl_202 {
() => {
// Module: crate::text::abstraction
// Provides: {"impl_202"}
// Dependencies: {}
impl < T : DiffableStr + ? Sized > DiffableStrRef for Cow < '_ , T > { type Output = T ; fn as_diffable_str (& self) -> & T { self } }
};
}
