// Generated macro for impl_17 (impl)
macro_rules! Depcrate_helpersimpl_17 {
() => {
// Module: crate::helpers
// Provides: {"impl_17"}
// Dependencies: {}
impl < W > DebugStruct < '_ , '_ , W > where W : uWrite + ? Sized , { # [doc = " Adds a new field to the generated struct output."] pub fn field (& mut self , name : & str , value : & impl uDebug) -> Result < & mut Self , W :: Error > { if self . first { self . first = false ; self . formatter . write_str (" {") ? ; if self . formatter . pretty { self . formatter . write_str ("\n") ? ; } else { self . formatter . write_str (" ") ? ; } } else if ! self . formatter . pretty { self . formatter . write_str (", ") ? ; } if self . formatter . pretty { self . formatter . indent () ? ; } self . formatter . write_str (name) ? ; self . formatter . write_str (": ") ? ; value . fmt (self . formatter) ? ; if self . formatter . pretty { self . formatter . write_str (",\n") ? ; } Ok (self) } # [doc = " Finishes output"] pub fn finish (& mut self) -> Result < () , W :: Error > { if self . formatter . pretty { self . formatter . indentation -= 1 ; } if ! self . first { if self . formatter . pretty { self . formatter . indent () ? ; } else { self . formatter . write_str (" ") ? ; } self . formatter . write_str ("}") ? ; } Ok (()) } }
};
}
