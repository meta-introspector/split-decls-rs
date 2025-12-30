// Generated macro for impl_329 (impl)
macro_rules! Depcrate_msgs_handshakeimpl_329 {
() => {
// Module: crate::msgs::handshake
// Provides: {"impl_329"}
// Dependencies: {}
impl Codec < '_ > for EcParameters { fn encode (& self , bytes : & mut Vec < u8 >) { self . curve_type . encode (bytes) ; self . named_group . encode (bytes) ; } fn read (r : & mut Reader < '_ >) -> Result < Self , InvalidMessage > { let ct = ECCurveType :: read (r) ? ; if ct != ECCurveType :: NamedCurve { return Err (InvalidMessage :: UnsupportedCurveType) ; } let grp = NamedGroup :: read (r) ? ; Ok (Self { curve_type : ct , named_group : grp , }) } }
};
}
