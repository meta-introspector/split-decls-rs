// Generated macro for impl_19 (impl)
macro_rules! Depcrate_helpersimpl_19 {
() => {
// Module: crate::helpers
// Provides: {"impl_19"}
// Dependencies: {}
impl < W > DebugTuple < '_ , '_ , W > where W : uWrite + ? Sized , { # [doc = " Adds a new field to the generated tuple struct output."] pub fn field (& mut self , value : & impl uDebug) -> Result < & mut Self , W :: Error > { self . fields += 1 ; if self . first { self . first = false ; self . formatter . write_str ("(") ? ; if self . formatter . pretty { self . formatter . write_str ("\n") ? ; } } else if ! self . formatter . pretty { self . formatter . write_str (", ") ? ; } if self . formatter . pretty { self . formatter . indent () ? ; } value . fmt (self . formatter) ? ; if self . formatter . pretty { self . formatter . write_str (",\n") ? ; } Ok (self) } # [doc = " Finishes output"] pub fn finish (& mut self) -> Result < () , W :: Error > { if self . formatter . pretty { self . formatter . indentation -= 1 ; } if ! self . first { if self . formatter . pretty { self . formatter . indent () ? ; } else if self . unnamed && self . fields == 1 { self . formatter . write_str (",") ? ; } self . formatter . write_str (")") ? ; } Ok (()) } }
};
}
