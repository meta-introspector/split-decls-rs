// Generated macro for DtorType (enum)
macro_rules! Depcrate_needs_dropDtorType {
() => {
// Module: crate::needs_drop
// Provides: {"DtorType"}
// Dependencies: {}
enum DtorType { # [doc = " Type has a `Drop` but it is considered insignificant."] # [doc = " Check the query `adt_significant_drop_tys` for understanding"] # [doc = " \"significant\" / \"insignificant\"."] Insignificant , # [doc = " Type has a `Drop` implantation."] Significant , }
};
}
