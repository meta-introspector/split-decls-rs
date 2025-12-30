// Generated macro for impl_102 (impl)
macro_rules! Depcrate_nameimpl_102 {
() => {
// Module: crate::name
// Provides: {"impl_102"}
// Dependencies: {}
impl RdnSequence { # [doc = " Converts an `RDNSequence` string into an encoded `RDNSequence`."] # [deprecated (since = "0.2.1" , note = "use RdnSequence::from_str(...)?.to_der()")] pub fn encode_from_string (s : & str) -> Result < Vec < u8 > , der :: Error > { Self :: from_str (s) ? . to_der () } # [doc = " Is this [`RdnSequence`] empty?"] pub fn is_empty (& self) -> bool { self . 0 . is_empty () } # [doc = " Iterate over this [`RdnSequence`]."] pub fn iter (& self) -> impl Iterator < Item = & RelativeDistinguishedName > { self . 0 . iter () } # [doc = " Length of this [`RdnSequence`]."] pub fn len (& self) -> usize { self . 0 . len () } # [doc = " Push a [`RelativeDistinguishedName`] onto this [`RdnSequence`]."] pub fn push (& mut self , name : RelativeDistinguishedName) { self . 0 . push (name) } }
};
}
