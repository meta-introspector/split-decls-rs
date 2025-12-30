// Generated macro for impl_81 (impl)
macro_rules! Depcrate_msgs_codecimpl_81 {
() => {
// Module: crate::msgs::codec
// Provides: {"impl_81"}
// Dependencies: {}
# [doc = " Implement `Codec` for lists of elements that implement `TlsListElement`."] # [doc = ""] # [doc = " `TlsListElement` provides the size of the length prefix for the list."] impl < 'a , T : Codec < 'a > + TlsListElement + Debug > Codec < 'a > for Vec < T > { fn encode (& self , bytes : & mut Vec < u8 >) { let nest = LengthPrefixedBuffer :: new (T :: SIZE_LEN , bytes) ; for i in self { i . encode (nest . buf) ; } } fn read (r : & mut Reader < 'a >) -> Result < Self , InvalidMessage > { let mut ret = Self :: new () ; for item in TlsListIter :: < T > :: new (r) ? { ret . push (item ?) ; } Ok (ret) } }
};
}
