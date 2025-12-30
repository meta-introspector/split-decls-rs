// Generated macro for impl_83 (impl)
macro_rules! Depcrate_msgs_codecimpl_83 {
() => {
// Module: crate::msgs::codec
// Provides: {"impl_83"}
// Dependencies: {}
impl < 'a , T : Codec < 'a > + TlsListElement + Debug > TlsListIter < 'a , T > { pub (crate) fn new (r : & mut Reader < 'a >) -> Result < Self , InvalidMessage > { let len = T :: SIZE_LEN . read (r) ? ; let sub = r . sub (len) ? ; Ok (Self { sub , _t : PhantomData , }) } }
};
}
