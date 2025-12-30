// Generated macro for DebugList (struct)
macro_rules! Depcrate_helpersDebugList {
() => {
// Module: crate::helpers
// Provides: {"DebugList"}
// Dependencies: {}
# [doc = " A struct to help with [`uDebug`] implementations."] # [doc = ""] # [doc = " This is useful when you wish to output a formatted list of items as a part of your"] # [doc = " [`uDebug::fmt`] implementation."] # [doc = ""] # [doc = " This can be constructed by the [`Formatter::debug_list`] method."] pub struct DebugList < 'f , 'w , W > where W : uWrite + ? Sized , { first : bool , formatter : & 'f mut Formatter < 'w , W > , }
};
}
