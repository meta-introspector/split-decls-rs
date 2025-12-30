// Generated macro for output_filename (function)
macro_rules! Depcrate_sys_backtraceoutput_filename {
() => {
// Module: crate::sys::backtrace
// Provides: {"output_filename"}
// Dependencies: {}
# [doc = " Prints the filename of the backtrace frame."] # [doc = ""] # [doc = " See also `output`."] pub fn output_filename (fmt : & mut fmt :: Formatter < '_ > , bows : BytesOrWideString < '_ > , print_fmt : PrintFmt , cwd : Option < & PathBuf > ,) -> fmt :: Result { let file : Cow < '_ , Path > = match bows { # [cfg (unix)] BytesOrWideString :: Bytes (bytes) => { use crate :: os :: unix :: prelude :: * ; Path :: new (crate :: ffi :: OsStr :: from_bytes (bytes)) . into () } # [cfg (not (unix))] BytesOrWideString :: Bytes (bytes) => { Path :: new (crate :: str :: from_utf8 (bytes) . unwrap_or ("<unknown>")) . into () } # [cfg (windows)] BytesOrWideString :: Wide (wide) => { use crate :: os :: windows :: prelude :: * ; Cow :: Owned (crate :: ffi :: OsString :: from_wide (wide) . into ()) } # [cfg (not (windows))] BytesOrWideString :: Wide (_wide) => Path :: new ("<unknown>") . into () , } ; if print_fmt == PrintFmt :: Short && file . is_absolute () { if let Some (cwd) = cwd { if let Ok (stripped) = file . strip_prefix (& cwd) { if let Some (s) = stripped . to_str () { return write ! (fmt , ".{}{s}" , path :: MAIN_SEPARATOR) ; } } } } fmt :: Display :: fmt (& file . display () , fmt) }
};
}
