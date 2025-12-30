// Generated macro for impl_53 (impl)
macro_rules! Depcrate_deimpl_53 {
() => {
// Module: crate::de
// Provides: {"impl_53"}
// Dependencies: {}
impl < 'de > VisitMap < 'de > { # [doc = " Determine the type of the map by deserializing it"] pub fn next_key_seed < V : serde_core :: de :: MapAccess < 'de > > (visitor : & mut V ,) -> Result < Option < Self > , V :: Error > { let mut key = None ; let Some (()) = visitor . next_key_seed (DatetimeOrTable :: new (& mut key)) ? else { return Ok (None) ; } ; let result = if let Some (key) = key { VisitMap :: Key (key) } else { let date : crate :: datetime :: DatetimeFromString = visitor . next_value () ? ; VisitMap :: Datetime (date . value) } ; Ok (Some (result)) } }
};
}
