// Generated macro for Safety (trait)
macro_rules! Depcrate_inherentSafety {
() => {
// Module: crate::inherent
// Provides: {"Safety"}
// Dependencies: {}
pub trait Safety < I : Interner < Safety = Self > > : Copy + Debug + Hash + Eq { fn safe () -> Self ; fn is_safe (self) -> bool ; fn prefix_str (self) -> & 'static str ; }
};
}
