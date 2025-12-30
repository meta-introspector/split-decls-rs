// Generated macro for impl_15 (impl)
macro_rules! Depcrate_helpersimpl_15 {
() => {
// Module: crate::helpers
// Provides: {"impl_15"}
// Dependencies: {}
impl < W > DebugSet < '_ , '_ , W > where W : uWrite + ? Sized , { # [doc = " Adds a new entry to the set output."] pub fn entry (& mut self , entry : & impl uDebug) -> Result < & mut Self , W :: Error > { if self . first { self . first = false ; if self . formatter . pretty { self . formatter . write_str ("\n") ? ; } } else if ! self . formatter . pretty { self . formatter . write_str (", ") ? ; } if self . formatter . pretty { self . formatter . indent () ? ; } entry . fmt (self . formatter) ? ; if self . formatter . pretty { self . formatter . write_str (",\n") ? ; } Ok (self) } # [doc = " Adds the contents of an iterator of entries to the set output."] pub fn entries (& mut self , entries : impl IntoIterator < Item = impl uDebug > ,) -> Result < & mut Self , W :: Error > { for entry in entries { self . entry (& entry) ? ; } Ok (self) } # [doc = " Finishes output"] pub fn finish (& mut self) -> Result < () , W :: Error > { self . formatter . write_str ("}") } }
};
}
