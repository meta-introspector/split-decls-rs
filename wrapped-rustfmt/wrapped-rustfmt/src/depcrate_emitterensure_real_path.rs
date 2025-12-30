// Generated macro for ensure_real_path (function)
macro_rules! Depcrate_emitterensure_real_path {
() => {
// Module: crate::emitter
// Provides: {"ensure_real_path"}
// Dependencies: {}
fn ensure_real_path (filename : & FileName) -> & Path { match * filename { FileName :: Real (ref path) => path , _ => panic ! ("cannot format `{filename}` and emit to files") , } }
};
}
