// Generated macro for impl_204 (impl)
macro_rules! Depcrate_alpn_listimpl_204 {
() => {
// Module: crate::alpn_list
// Provides: {"impl_204"}
// Dependencies: {}
impl AlpnList { pub fn new (protos : & [Vec < u8 >]) -> Self { let mut alpn_wire_format = Vec :: with_capacity (protos . iter () . map (Vec :: len) . sum :: < usize > () + protos . len ()) ; for alpn in protos { alpn_wire_format . push (alpn . len () as u8) ; alpn_wire_format . extend (alpn) ; } let size = SEC_APPLICATION_PROTOCOL_HEADER_SIZE + alpn_wire_format . len () ; let layout = alloc :: Layout :: from_size_align (size , mem :: align_of :: < Identity :: SEC_APPLICATION_PROTOCOLS > () ,) . unwrap () ; unsafe { let memory = match ptr :: NonNull :: new (alloc :: alloc (layout)) { Some (ptr) => ptr , None => alloc :: handle_alloc_error (layout) , } ; let buf = slice :: from_raw_parts_mut (memory . as_ptr () , layout . size ()) ; let protocols = & mut * (buf . as_mut_ptr () as * mut Identity :: SEC_APPLICATION_PROTOCOLS) ; protocols . ProtocolListsSize = (SEC_APPLICATION_PROTOCOL_LIST_HEADER_SIZE + alpn_wire_format . len ()) as u32 ; let protocol = & mut * protocols . ProtocolLists . as_mut_ptr () ; protocol . ProtoNegoExt = Identity :: SecApplicationProtocolNegotiationExt_ALPN ; protocol . ProtocolListSize = alpn_wire_format . len () as u16 ; let protocol_list_offset = protocol . ProtocolList . as_ptr () as usize - buf . as_ptr () as usize ; let protocol_list = & mut buf [protocol_list_offset ..] ; protocol_list . copy_from_slice (& alpn_wire_format) ; Self { layout , memory } } } }
};
}
