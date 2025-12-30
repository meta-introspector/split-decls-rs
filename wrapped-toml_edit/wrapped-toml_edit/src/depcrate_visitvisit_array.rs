// Generated macro for visit_array (function)
macro_rules! Depcrate_visitvisit_array {
() => {
// Module: crate::visit
// Provides: {"visit_array"}
// Dependencies: {}
pub fn visit_array < 'doc , V > (v : & mut V , node : & 'doc Array) where V : Visit < 'doc > + ? Sized , { for value in node . iter () { v . visit_value (value) ; } }
};
}
