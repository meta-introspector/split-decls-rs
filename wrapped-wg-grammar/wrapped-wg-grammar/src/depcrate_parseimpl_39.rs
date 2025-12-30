// Generated macro for impl_39 (impl)
macro_rules! Depcrate_parseimpl_39 {
() => {
// Module: crate::parse
// Provides: {"impl_39"}
// Dependencies: {}
impl < 'a , 'i , I : :: gll :: runtime :: Input > Handle < 'a , 'i , I , LITERAL < 'a , 'i , I > > { pub fn one (self) -> Result < LITERAL < 'a , 'i , I > , Ambiguity < Self > > { (| | Ok ({ # [allow (unused_variables)] let sppf = & self . parser . sppf ; let node = self . node . unpack_alias () ; let r = traverse ! (one (sppf , node) _) ; LITERAL :: from_sppf (self . parser , node , r) })) () . map_err (| :: gll :: runtime :: MoreThanOne | Ambiguity (self)) } pub fn all (self) -> impl Iterator < Item = LITERAL < 'a , 'i , I > > { # [allow (unused_variables)] let sppf = & self . parser . sppf ; let node = self . node . unpack_alias () ; traverse ! (all (sppf) _) . apply (node) . map (move | r | LITERAL :: from_sppf (self . parser , node , r)) } }
};
}
