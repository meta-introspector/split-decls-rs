// Generated macro for impl_149 (impl)
macro_rules! Depcrate_discoverimpl_149 {
() => {
// Module: crate::discover
// Provides: {"impl_149"}
// Dependencies: {}
impl < K , S , E , D : ? Sized > Discover for D where D : TryStream < Ok = Change < K , S > , Error = E > , K : Eq , { type Key = K ; type Service = S ; type Error = E ; fn poll_discover (self : Pin < & mut Self > , cx : & mut Context < '_ > ,) -> Poll < Option < Result < D :: Ok , D :: Error > > > { TryStream :: try_poll_next (self , cx) } }
};
}
