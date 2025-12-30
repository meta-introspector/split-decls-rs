// Generated macro for impl_12 (impl)
macro_rules! Depcrate_serdeimpl_12 {
() => {
// Module: crate::serde
// Provides: {"impl_12"}
// Dependencies: {}
impl < 'de , T > Visitor < 'de > for SlabVisitor < T > where T : Deserialize < 'de > , { type Value = Slab < T > ; fn expecting (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (fmt , "a map") } fn visit_map < A > (self , mut map : A) -> Result < Self :: Value , A :: Error > where A : MapAccess < 'de > , { let mut builder = Builder :: with_capacity (map . size_hint () . unwrap_or (0)) ; while let Some ((key , value)) = map . next_entry () ? { builder . pair (key , value) } Ok (builder . build ()) } }
};
}
