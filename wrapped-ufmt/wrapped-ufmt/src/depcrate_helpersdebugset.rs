// Generated macro for DebugSet (struct)
macro_rules! Depcrate_helpersDebugSet {
() => {
// Module: crate::helpers
// Provides: {"DebugSet"}
// Dependencies: {}
# [doc = " A struct to help with [`uDebug`] implementations."] # [doc = ""] # [doc = " This is useful when you wish to output a formatted set of items as a part of your"] # [doc = " [`uDebug::fmt`] implementation."] # [doc = ""] # [doc = " This can be constructed by the [`Formatter::debug_set`] method."] pub struct DebugSet < 'f , 'w , W > where W : uWrite + ? Sized , { first : bool , formatter : & 'f mut Formatter < 'w , W > , }
};
}
