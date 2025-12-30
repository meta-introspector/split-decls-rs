// Generated macro for impl_36 (impl)
macro_rules! Depcrate_msgs_baseimpl_36 {
() => {
// Module: crate::msgs::base
// Provides: {"impl_36"}
// Dependencies: {}
impl < 'a , C : Cardinality > Codec < 'a > for PayloadU24 < 'a , C > { fn encode (& self , bytes : & mut Vec < u8 >) { let inner = self . 0 . bytes () ; debug_assert ! (inner . len () >= C :: MIN) ; codec :: u24 (inner . len () as u32) . encode (bytes) ; bytes . extend_from_slice (inner) ; } fn read (r : & mut Reader < 'a >) -> Result < Self , InvalidMessage > { let len = codec :: u24 :: read (r) ? . 0 as usize ; if len < C :: MIN { return Err (InvalidMessage :: IllegalEmptyList ("PayloadU24")) ; } let mut sub = r . sub (len) ? ; Ok (Self (Payload :: read (& mut sub) , PhantomData)) } }
};
}
