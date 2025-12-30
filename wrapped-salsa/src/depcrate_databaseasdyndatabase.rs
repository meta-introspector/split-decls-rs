// Generated macro for AsDynDatabase (trait)
macro_rules! Depcrate_databaseAsDynDatabase {
() => {
// Module: crate::database
// Provides: {"AsDynDatabase"}
// Dependencies: {}
# [doc = " Upcast to a `dyn Database`."] # [doc = ""] # [doc = " Only required because upcasting does not work for unsized generic parameters."] pub trait AsDynDatabase { fn as_dyn_database (& self) -> & dyn Database ; }
};
}
