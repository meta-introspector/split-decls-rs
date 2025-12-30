// Generated macro for handle_compile_finished (function)
macro_rules! Depcrate_commandshandle_compile_finished {
() => {
// Module: crate::commands
// Provides: {"handle_compile_finished"}
// Dependencies: {}
# [doc = " Handle `response`, the output from running a compile on the server."] # [doc = " Return the compiler exit status."] fn handle_compile_finished (response : CompileFinished , stdout : & mut dyn Write , stderr : & mut dyn Write ,) -> Result < i32 > { trace ! ("handle_compile_finished") ; fn write_output (stream : impl IsTerminal , writer : & mut dyn Write , data : & [u8] , color_mode : ColorMode ,) -> Result < () > { let dumb_term = env :: var ("TERM") . map (| v | v == "dumb") . unwrap_or (false) ; if color_mode == ColorMode :: On || (! dumb_term && stream . is_terminal () && color_mode != ColorMode :: Off) { writer . write_all (data) ? ; } else { let mut writer = Writer :: new (writer) ; writer . write_all (data) ? ; } Ok (()) } write_output (std :: io :: stdout () , stdout , & response . stdout , response . color_mode ,) ? ; write_output (std :: io :: stderr () , stderr , & response . stderr , response . color_mode ,) ? ; if let Some (ret) = response . retcode { trace ! ("compiler exited with status {}" , ret) ; Ok (ret) } else if let Some (signal) = response . signal { println ! ("sccache: Compiler killed by signal {}" , signal) ; Ok (- 2) } else { println ! ("sccache: Missing compiler exit status!") ; Ok (- 3) } }
};
}
