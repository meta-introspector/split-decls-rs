// Generated macro for impl_11 (impl)
macro_rules! Depcrate_helpersimpl_11 {
() => {
// Module: crate::helpers
// Provides: {"impl_11"}
// Dependencies: {}
impl < W > DebugList < '_ , '_ , W > where W : uWrite + ? Sized , { # [doc = " Adds a new entry to the list output."] pub fn entry (& mut self , entry : & impl uDebug) -> Result < & mut Self , W :: Error > { if self . first { self . first = false ; if self . formatter . pretty { self . formatter . write_str ("\n") ? ; } } else if ! self . formatter . pretty { self . formatter . write_str (", ") ? ; } if self . formatter . pretty { self . formatter . indent () ? ; } entry . fmt (self . formatter) ? ; if self . formatter . pretty { self . formatter . write_str (",\n") ? ; } Ok (self) } # [doc = " Adds the contents of an iterator of entries to the list output."] pub fn entries (& mut self , entries : impl IntoIterator < Item = impl uDebug > ,) -> Result < & mut Self , W :: Error > { for entry in entries { self . entry (& entry) ? ; } Ok (self) } # [doc = " Finishes output"] pub fn finish (& mut self) -> Result < () , W :: Error > { if self . formatter . pretty { self . formatter . indentation -= 1 ; self . formatter . indent () ? ; } self . formatter . write_str ("]") } }
};
}
