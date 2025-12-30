// Generated macro for DebugMap (struct)
macro_rules! Depcrate_helpersDebugMap {
() => {
// Module: crate::helpers
// Provides: {"DebugMap"}
// Dependencies: {}
# [doc = " A struct to help with [`uDebug`] implementations."] # [doc = ""] # [doc = " This is useful when you wish to output a formatted map as a part of your [`uDebug::fmt`]"] # [doc = " implementation."] # [doc = ""] # [doc = " This can be constructed by the [`Formatter::debug_map`] method."] pub struct DebugMap < 'f , 'w , W > where W : uWrite + ? Sized , { first : bool , formatter : & 'f mut Formatter < 'w , W > , }
};
}
