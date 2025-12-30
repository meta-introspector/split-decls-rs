// Generated macro for fix_dependencies (function)
macro_rules! Depcrate_dependenciesfix_dependencies {
() => {
// Module: crate::dependencies
// Provides: {"fix_dependencies"}
// Dependencies: {}
fn fix_dependencies (dependencies : & mut Map < String , Dependency > , dir : & Directory) { dependencies . remove ("trybuild") ; for dep in dependencies . values_mut () { dep . path = dep . path . as_ref () . map (| path | Directory :: new (dir . join (path))) ; } }
};
}
