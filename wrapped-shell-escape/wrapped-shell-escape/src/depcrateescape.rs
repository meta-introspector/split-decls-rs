// Generated macro for escape (function)
macro_rules! Depcrateescape {
() => {
// Module: crate
// Provides: {"escape"}
// Dependencies: {}
# [doc = " Escape characters that may have special meaning in a shell."] pub fn escape (s : Cow < str >) -> Cow < str > { if cfg ! (unix) || env :: var ("MSYSTEM") . is_ok () { unix :: escape (s) } else { windows :: escape (s) } }
};
}
