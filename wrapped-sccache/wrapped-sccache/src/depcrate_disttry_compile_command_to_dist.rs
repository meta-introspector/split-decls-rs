// Generated macro for try_compile_command_to_dist (function)
macro_rules! Depcrate_disttry_compile_command_to_dist {
() => {
// Module: crate::dist
// Provides: {"try_compile_command_to_dist"}
// Dependencies: {}
pub fn try_compile_command_to_dist (command : compiler :: SingleCompileCommand ,) -> Option < CompileCommand > { let compiler :: SingleCompileCommand { executable , arguments , env_vars , cwd , } = command ; Some (CompileCommand { executable : executable . into_os_string () . into_string () . ok () ? , arguments : arguments . into_iter () . map (| arg | arg . into_string () . ok ()) . collect :: < Option < _ > > () ? , env_vars : env_vars . into_iter () . map (| (k , v) | Some ((k . into_string () . ok () ? , v . into_string () . ok () ?))) . collect :: < Option < _ > > () ? , cwd : cwd . into_os_string () . into_string () . ok () ? , }) }
};
}
