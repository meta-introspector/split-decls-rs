// Generated macro for impl_78 (impl)
macro_rules! Depcrate_externallyimpl_78 {
() => {
// Module: crate::externally
// Provides: {"impl_78"}
// Dependencies: {}
impl < 'de , T : ? Sized > Visitor < 'de > for TaggedVisitor < T > { type Value = Box < T > ; fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { write ! (formatter , "dyn {}" , self . trait_object) } fn visit_map < A > (self , mut map : A) -> Result < Self :: Value , A :: Error > where A : MapAccess < 'de > , { let map_lookup = MapLookupVisitor { expected : & self , registry : self . registry , } ; let Some (deserialize_fn) = map . next_key_seed (map_lookup) ? else { return Err (de :: Error :: custom (format_args ! ("expected externally tagged dyn {}" , self . trait_object))) ; } ; map . next_value_seed (FnApply { deserialize_fn }) } }
};
}
