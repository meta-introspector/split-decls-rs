// Generated macro for impl_48 (impl)
macro_rules! Depcrate_msgs_baseimpl_48 {
() => {
// Module: crate::msgs::base
// Provides: {"impl_48"}
// Dependencies: {}
impl < C : Cardinality > Codec < '_ > for PayloadU8 < C > { fn encode (& self , bytes : & mut Vec < u8 >) { debug_assert ! (self . 0 . len () >= C :: MIN) ; (self . 0 . len () as u8) . encode (bytes) ; bytes . extend_from_slice (& self . 0) ; } fn read (r : & mut Reader < '_ >) -> Result < Self , InvalidMessage > { let len = u8 :: read (r) ? as usize ; if len < C :: MIN { return Err (InvalidMessage :: IllegalEmptyValue) ; } let mut sub = r . sub (len) ? ; let body = sub . rest () . to_vec () ; Ok (Self (body , PhantomData)) } }
};
}
