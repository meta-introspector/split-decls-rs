// Generated macro for DebugStruct (struct)
macro_rules! Depcrate_helpersDebugStruct {
() => {
// Module: crate::helpers
// Provides: {"DebugStruct"}
// Dependencies: {}
# [doc = " A struct to help with [`uDebug`] implementations."] # [doc = ""] # [doc = " This is useful when you wish to output a formatted struct as a part of your [`uDebug::fmt`]"] # [doc = " implementation."] # [doc = ""] # [doc = " This can be constructed by the [`Formatter::debug_struct`] method."] pub struct DebugStruct < 'f , 'w , W > where W : uWrite + ? Sized , { first : bool , formatter : & 'f mut Formatter < 'w , W > , }
};
}
