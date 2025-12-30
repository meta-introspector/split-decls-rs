// Generated macro for box_forwarded_impl (macro)
macro_rules! Depcrate_de_implsbox_forwarded_impl {
() => {
// Module: crate::de::impls
// Provides: {"box_forwarded_impl"}
// Dependencies: {}
macro_rules ! box_forwarded_impl { ($ (# [$ attr : meta]) * $ t : ident) => { $ (# [$ attr]) * impl <'de , T > Deserialize <'de > for $ t < T > where T : ? Sized , Box < T >: Deserialize <'de >, { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer <'de >, { Box :: deserialize (deserializer) . map (Into :: into) } } } ; }
};
}
