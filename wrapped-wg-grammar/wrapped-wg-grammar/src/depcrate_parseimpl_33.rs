// Generated macro for impl_33 (impl)
macro_rules! Depcrate_parseimpl_33 {
() => {
// Module: crate::parse
// Provides: {"impl_33"}
// Dependencies: {}
impl < 'a , 'i , I : :: gll :: runtime :: Input > Handle < 'a , 'i , I , PUNCT < 'a , 'i , I > > { pub fn one (self) -> Result < PUNCT < 'a , 'i , I > , Ambiguity < Self > > { (| | Ok ({ # [allow (unused_variables)] let sppf = & self . parser . sppf ; let node = self . node . unpack_alias () ; let r = traverse ! (one (sppf , node) _) ; PUNCT :: from_sppf (self . parser , node , r) })) () . map_err (| :: gll :: runtime :: MoreThanOne | Ambiguity (self)) } pub fn all (self) -> impl Iterator < Item = PUNCT < 'a , 'i , I > > { # [allow (unused_variables)] let sppf = & self . parser . sppf ; let node = self . node . unpack_alias () ; traverse ! (all (sppf) _) . apply (node) . map (move | r | PUNCT :: from_sppf (self . parser , node , r)) } }
};
}
