// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () -> Result < () , Box < dyn std :: error :: Error > > { println ! ("🔧 Analyzing declaration dependencies for topological sort") ; let mut sorter = DeclSorter :: new () ; sorter . add_decl ("run_bootstrap_mode" . to_string ()) ; sorter . add_dependency ("run_bootstrap_mode" , "SplitDeclsConfig") ; sorter . add_dependency ("run_bootstrap_mode" , "PathBuf") ; sorter . add_dependency ("run_bootstrap_mode" , "run_wrapped_workspace_mode") ; sorter . add_dependency ("SplitDeclsConfig" , "HashMap") ; sorter . add_dependency ("SplitDeclsConfig" , "serde") ; let sorted = sorter . topological_sort () ? ; println ! ("📋 Topological order for bootstrap declarations:") ; for (i , decl) in sorted . iter () . enumerate () { println ! ("{}. {}" , i + 1 , decl) ; } println ! ("\n🎯 Bootstrap3 should include declarations in this order!") ; Ok (()) }
};
}
