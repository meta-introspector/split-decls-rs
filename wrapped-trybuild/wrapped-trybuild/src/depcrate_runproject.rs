// Generated macro for Project (struct)
macro_rules! Depcrate_runProject {
() => {
// Module: crate::run
// Provides: {"Project"}
// Dependencies: {}
# [derive (Debug)] pub (crate) struct Project { pub dir : Directory , source_dir : Directory , pub target_dir : Directory , pub name : String , update : Update , pub has_pass : bool , has_compile_fail : bool , pub features : Option < Vec < String > > , pub workspace : Directory , pub path_dependencies : Vec < PathDependency > , manifest : Manifest , pub keep_going : bool , }
};
}
