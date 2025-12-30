// Generated macro for impl_30 (impl)
macro_rules! Depcrate_clausesimpl_30 {
() => {
// Module: crate::clauses
// Provides: {"impl_30"}
// Dependencies: {}
impl Clauses { # [doc = " Value of a literal if known from unit clauses."] pub fn lit_value (& self , lit : Lit) -> Option < (bool , UnitClause) > { self . unit_clauses [lit . index ()] . map (| unit_clause | (unit_clause . value ^ lit . is_negative () , unit_clause)) } }
};
}
