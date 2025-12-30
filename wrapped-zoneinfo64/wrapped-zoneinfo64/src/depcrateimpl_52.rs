// Generated macro for impl_52 (impl)
macro_rules! Depcrateimpl_52 {
() => {
// Module: crate
// Provides: {"impl_52"}
// Dependencies: {}
impl < 'a > ZoneInfo64 < 'a > { # [doc = " Parse this object from 4-byte aligned data"] pub fn try_from_u32s (resb : & 'a [u32]) -> Result < Self , BinaryDeserializerError > { crate :: deserialize :: deserialize (resb) } # [cfg (test)] fn is_alias (& self , iana : & str) -> bool { let Some (idx) = self . names . binary_search_by (| & n | n . chars () . cmp (iana . chars ())) . ok () else { return false ; } ; # [expect (clippy :: indexing_slicing)] let zone = & self . zones [idx] ; matches ! (zone , & TzZone :: Int (_)) } # [cfg (test)] fn iter (& 'a self) -> impl Iterator < Item = Zone < 'a > > { (0 .. self . names . len ()) . map (move | i | Zone :: from_raw_parts ((i as u16 , self))) } # [doc = " Get data for a given IANA timezone id. Aliases are supported."] pub fn get (& 'a self , iana : & str) -> Option < Zone < 'a > > { let idx = self . names . binary_search_by (| & n | n . chars () . cmp (iana . chars ())) . ok () ? ; # [expect (clippy :: indexing_slicing)] let resolved_idx = if let TzZone :: Int (i) = self . zones [idx] { i as u16 } else { idx as u16 } ; Some (Zone { idx : idx as u16 , resolved_idx , info : self , }) } }
};
}
