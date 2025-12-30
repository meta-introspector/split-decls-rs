// Generated macro for impl_13 (impl)
macro_rules! Depcrate_helpersimpl_13 {
() => {
// Module: crate::helpers
// Provides: {"impl_13"}
// Dependencies: {}
impl < W > DebugMap < '_ , '_ , W > where W : uWrite + ? Sized , { # [doc = " Adds a new entry to the map output."] pub fn entry (& mut self , key : & impl uDebug , value : & impl uDebug) -> Result < & mut Self , W :: Error > { if self . first { self . first = false ; if self . formatter . pretty { self . formatter . write_str ("\n") ? ; } } else if ! self . formatter . pretty { self . formatter . write_str (", ") ? ; } if self . formatter . pretty { self . formatter . indent () ? ; } key . fmt (self . formatter) ? ; self . formatter . write_str (": ") ? ; value . fmt (self . formatter) ? ; if self . formatter . pretty { self . formatter . write_str (",\n") ? ; } Ok (self) } # [doc = " Adds the contents of an iterator of entries to the map output."] pub fn entries (& mut self , entries : impl IntoIterator < Item = (impl uDebug , impl uDebug) > ,) -> Result < & mut Self , W :: Error > { for (k , v) in entries . into_iter () { self . entry (& k , & v) ? ; } Ok (self) } # [doc = " Finishes output"] pub fn finish (& mut self) -> Result < () , W :: Error > { self . formatter . write_str ("}") } }
};
}
