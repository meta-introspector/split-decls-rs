// Generated macro for forwarded_impl (macro)
macro_rules! Depcrate_de_implsforwarded_impl {
() => {
// Module: crate::de::impls
// Provides: {"forwarded_impl"}
// Dependencies: {}
macro_rules ! forwarded_impl { ($ (# [$ attr : meta]) * ($ ($ id : ident) ,*) , $ ty : ty , $ func : expr) => { $ (# [$ attr]) * impl <'de $ (, $ id : Deserialize <'de >,) *> Deserialize <'de > for $ ty { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer <'de >, { Deserialize :: deserialize (deserializer) . map ($ func) } } } }
};
}
