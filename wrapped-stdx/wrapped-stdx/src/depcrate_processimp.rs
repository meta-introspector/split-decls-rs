// Generated macro for imp (module)
macro_rules! Depcrate_processimp {
() => {
// Module: crate::process
// Provides: {"imp"}
// Dependencies: {}
# [cfg (target_arch = "wasm32")] mod imp { use std :: { io , process :: { ChildStderr , ChildStdout } , } ; pub (crate) fn read2 (_out_pipe : ChildStdout , _err_pipe : ChildStderr , _data : & mut dyn FnMut (bool , & mut Vec < u8 > , bool) ,) -> io :: Result < () > { panic ! ("no processes on wasm") } }
};
}
