// Generated macro for impl_156 (impl)
macro_rules! Depcrate_tinyvecimpl_156 {
() => {
// Module: crate::tinyvec
// Provides: {"impl_156"}
// Dependencies: {}
# [cfg (feature = "arbitrary")] # [cfg_attr (docs_rs , doc (cfg (feature = "arbitrary")))] impl < 'a , A > arbitrary :: Arbitrary < 'a > for TinyVec < A > where A : Array , A :: Item : arbitrary :: Arbitrary < 'a > , { fn arbitrary (u : & mut arbitrary :: Unstructured < 'a >) -> arbitrary :: Result < Self > { let v = Vec :: arbitrary (u) ? ; let mut tv = TinyVec :: Heap (v) ; tv . shrink_to_fit () ; Ok (tv) } }
};
}
