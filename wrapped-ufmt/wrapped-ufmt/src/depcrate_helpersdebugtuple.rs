// Generated macro for DebugTuple (struct)
macro_rules! Depcrate_helpersDebugTuple {
() => {
// Module: crate::helpers
// Provides: {"DebugTuple"}
// Dependencies: {}
# [doc = " A struct to help with [`uDebug`] implementations."] # [doc = ""] # [doc = " This is useful when you wish to output a formatted tuple as a part of your [`uDebug::fmt`]"] # [doc = " implementation."] # [doc = ""] # [doc = " This can be constructed by the [`Formatter::debug_tuple`] method."] pub struct DebugTuple < 'f , 'w , W > where W : uWrite + ? Sized , { fields : u8 , first : bool , formatter : & 'f mut Formatter < 'w , W > , unnamed : bool , }
};
}
