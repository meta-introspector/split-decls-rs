// Generated macro for build_mdman (function)
macro_rules! Depcratebuild_mdman {
() => {
// Module: crate
// Provides: {"build_mdman"}
// Dependencies: {}
# [doc = " Builds the man pages for `mdman`."] fn build_mdman () -> io :: Result < () > { cwd_to_workspace_root () ? ; let src_paths = & ["crates/mdman/doc/mdman.md" . into ()] ; let dst_path = "crates/mdman/doc/out" ; let outs = [("md" , dst_path) , ("txt" , dst_path) , ("man" , dst_path)] ; build_man ("mdman" , src_paths , & outs , & []) }
};
}
