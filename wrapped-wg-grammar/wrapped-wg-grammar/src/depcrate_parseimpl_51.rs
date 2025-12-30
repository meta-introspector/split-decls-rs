// Generated macro for impl_51 (impl)
macro_rules! Depcrate_parseimpl_51 {
() => {
// Module: crate::parse
// Provides: {"impl_51"}
// Dependencies: {}
impl < 'a , 'i , I : :: gll :: runtime :: Input > Handle < 'a , 'i , I , LIFETIME < 'a , 'i , I > > { pub fn one (self) -> Result < LIFETIME < 'a , 'i , I > , Ambiguity < Self > > { (| | Ok ({ # [allow (unused_variables)] let sppf = & self . parser . sppf ; let node = self . node . unpack_alias () ; let r = traverse ! (one (sppf , node) _) ; LIFETIME :: from_sppf (self . parser , node , r) })) () . map_err (| :: gll :: runtime :: MoreThanOne | Ambiguity (self)) } pub fn all (self) -> impl Iterator < Item = LIFETIME < 'a , 'i , I > > { # [allow (unused_variables)] let sppf = & self . parser . sppf ; let node = self . node . unpack_alias () ; traverse ! (all (sppf) _) . apply (node) . map (move | r | LIFETIME :: from_sppf (self . parser , node , r)) } }
};
}
