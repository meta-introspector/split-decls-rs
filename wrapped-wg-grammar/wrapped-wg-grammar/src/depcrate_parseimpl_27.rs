// Generated macro for impl_27 (impl)
macro_rules! Depcrate_parseimpl_27 {
() => {
// Module: crate::parse
// Provides: {"impl_27"}
// Dependencies: {}
impl < 'a , 'i , I : :: gll :: runtime :: Input > Handle < 'a , 'i , I , IDENT < 'a , 'i , I > > { pub fn one (self) -> Result < IDENT < 'a , 'i , I > , Ambiguity < Self > > { (| | Ok ({ # [allow (unused_variables)] let sppf = & self . parser . sppf ; let node = self . node . unpack_alias () ; let r = traverse ! (one (sppf , node) _) ; IDENT :: from_sppf (self . parser , node , r) })) () . map_err (| :: gll :: runtime :: MoreThanOne | Ambiguity (self)) } pub fn all (self) -> impl Iterator < Item = IDENT < 'a , 'i , I > > { # [allow (unused_variables)] let sppf = & self . parser . sppf ; let node = self . node . unpack_alias () ; traverse ! (all (sppf) _) . apply (node) . map (move | r | IDENT :: from_sppf (self . parser , node , r)) } }
};
}
