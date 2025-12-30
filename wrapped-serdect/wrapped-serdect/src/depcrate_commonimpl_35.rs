// Generated macro for impl_35 (impl)
macro_rules! Depcrate_commonimpl_35 {
() => {
// Module: crate::common
// Provides: {"impl_35"}
// Dependencies: {}
impl < 'b , T : LengthCheck > Visitor < '_ > for StrIntoBufVisitor < 'b , T > { type Value = & 'b [u8] ; fn expecting (& self , formatter : & mut fmt :: Formatter < '_ >) -> fmt :: Result { T :: expecting (formatter , "a string" , self . 0 . len () * 2) } fn visit_str < E > (self , v : & str) -> Result < Self :: Value , E > where E : Error , { base16ct :: mixed :: decode (v , self . 0) . map_err (| err | match err { base16ct :: Error :: InvalidLength => { Error :: invalid_length (v . len () , & "an even number of hex digits") } base16ct :: Error :: InvalidEncoding => Error :: invalid_value (Unexpected :: Other ("<potentially secret hex string>") , & "a sequence of hex digits (0-9,a-f,A-F)" ,) , }) } }
};
}
