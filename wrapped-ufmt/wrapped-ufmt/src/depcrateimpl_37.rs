// Generated macro for impl_37 (impl)
macro_rules! Depcrateimpl_37 {
() => {
// Module: crate
// Provides: {"impl_37"}
// Dependencies: {}
impl < 'w , W > Formatter < 'w , W > where W : uWrite + ? Sized , { # [doc = " Creates a formatter from the given writer"] pub fn new (writer : & 'w mut W) -> Self { Self { indentation : 0 , pretty : false , writer , } } # [doc = " Execute the closure with pretty-printing enabled"] pub fn pretty (& mut self , f : impl FnOnce (& mut Self) -> Result < () , W :: Error > ,) -> Result < () , W :: Error > { let pretty = self . pretty ; self . pretty = true ; f (self) ? ; self . pretty = pretty ; Ok (()) } # [doc = " Writes a character to the underlying buffer contained within this formatter."] pub fn write_char (& mut self , c : char) -> Result < () , W :: Error > { self . writer . write_char (c) } # [doc = " Writes a string slice to the underlying buffer contained within this formatter."] pub fn write_str (& mut self , s : & str) -> Result < () , W :: Error > { self . writer . write_str (s) } # [doc = " Write whitespace according to the current `self.indentation`"] fn indent (& mut self) -> Result < () , W :: Error > { for _ in 0 .. self . indentation { self . write_str ("    ") ? ; } Ok (()) } }
};
}
