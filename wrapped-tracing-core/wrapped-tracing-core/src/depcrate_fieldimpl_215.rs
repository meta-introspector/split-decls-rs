// Generated macro for impl_215 (impl)
macro_rules! Depcrate_fieldimpl_215 {
() => {
// Module: crate::field
// Provides: {"impl_215"}
// Dependencies: {}
impl PartialEq for FieldSet { fn eq (& self , other : & Self) -> bool { if core :: ptr :: eq (& self , & other) { true } else if cfg ! (not (debug_assertions)) { self . callsite == other . callsite } else { let Self { names : lhs_names , callsite : lhs_callsite , } = self ; let Self { names : rhs_names , callsite : rhs_callsite , } = & other ; lhs_callsite == rhs_callsite && lhs_names == rhs_names } } }
};
}
