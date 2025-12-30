// Generated macro for fix_path_for_mac (function)
macro_rules! Depcrate_installfix_path_for_mac {
() => {
// Module: crate::install
// Provides: {"fix_path_for_mac"}
// Dependencies: {}
fn fix_path_for_mac (sh : & Shell) -> anyhow :: Result < () > { let mut vscode_path : Vec < PathBuf > = { const COMMON_APP_PATH : & str = r"/Applications/Visual Studio Code.app/Contents/Resources/app/bin" ; const ROOT_DIR : & str = "" ; let home_dir = sh . var ("HOME") . map_err (| err | { format_err ! ("Failed getting HOME from environment with error: {}." , err) }) ? ; [ROOT_DIR , & home_dir] . into_iter () . map (| dir | dir . to_owned () + COMMON_APP_PATH) . map (PathBuf :: from) . filter (| path | path . exists ()) . collect () } ; if ! vscode_path . is_empty () { let vars = sh . var_os ("PATH") . context ("Could not get PATH variable from env.") ? ; let mut paths = env :: split_paths (& vars) . collect :: < Vec < _ > > () ; paths . append (& mut vscode_path) ; let new_paths = env :: join_paths (paths) . context ("build env PATH") ? ; sh . set_var ("PATH" , new_paths) ; } Ok (()) }
};
}
