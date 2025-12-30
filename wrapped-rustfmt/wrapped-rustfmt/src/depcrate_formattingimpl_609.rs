// Generated macro for impl_609 (impl)
macro_rules! Depcrate_formattingimpl_609 {
() => {
// Module: crate::formatting
// Provides: {"impl_609"}
// Dependencies: {}
impl < 'b , T : Write + 'b > FormatHandler for Session < 'b , T > { fn handle_formatted_file (& mut self , psess : & ParseSess , path : FileName , result : String , report : & mut FormatReport ,) -> Result < () , ErrorKind > { if let Some (ref mut out) = self . out { match source_file :: write_file (Some (psess) , & path , & result , out , & mut * self . emitter , self . config . newline_style () ,) { Ok (ref result) if result . has_diff => report . add_diff () , Err (e) => { let err_msg = format ! ("{path}: {e}") ; return Err (io :: Error :: new (e . kind () , err_msg) . into ()) ; } _ => { } } } self . source_file . push ((path , result)) ; Ok (()) } }
};
}
