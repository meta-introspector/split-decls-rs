// Generated macro for SizeHint (trait)
macro_rules! Depcrate_ioSizeHint {
() => {
// Module: crate::io
// Provides: {"SizeHint"}
// Dependencies: {}
trait SizeHint { fn lower_bound (& self) -> usize ; fn upper_bound (& self) -> Option < usize > ; fn size_hint (& self) -> (usize , Option < usize >) { (self . lower_bound () , self . upper_bound ()) } }
};
}
