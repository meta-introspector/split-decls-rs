// Generated macro for impl_69 (impl)
macro_rules! Depcrateimpl_69 {
() => {
// Module: crate
// Provides: {"impl_69"}
// Dependencies: {}
# [cfg (feature = "arbitrary")] # [cfg_attr (docsrs , doc (cfg (feature = "arbitrary")))] impl < 'a > arbitrary :: Arbitrary < 'a > for SmolStr { fn arbitrary (u : & mut arbitrary :: Unstructured < 'a >) -> Result < Self , arbitrary :: Error > { let s = < & str > :: arbitrary (u) ? ; Ok (SmolStr :: new (s)) } }
};
}
