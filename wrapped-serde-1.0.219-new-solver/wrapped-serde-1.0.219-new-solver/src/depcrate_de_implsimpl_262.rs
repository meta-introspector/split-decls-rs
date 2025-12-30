// Generated macro for impl_262 (impl)
macro_rules! Depcrate_de_implsimpl_262 {
() => {
// Module: crate::de::impls
// Provides: {"impl_262"}
// Dependencies: {}
# [cfg (any (feature = "std" , feature = "alloc"))] # [cfg_attr (docsrs , doc (cfg (any (feature = "std" , feature = "alloc"))))] impl < 'de , 'a , T > Deserialize < 'de > for Cow < 'a , T > where T : ? Sized + ToOwned , T :: Owned : Deserialize < 'de > , { # [inline] fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { T :: Owned :: deserialize (deserializer) . map (Cow :: Owned) } }
};
}
