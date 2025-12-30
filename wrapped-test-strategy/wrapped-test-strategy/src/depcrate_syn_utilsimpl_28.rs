// Generated macro for impl_28 (impl)
macro_rules! Depcrate_syn_utilsimpl_28 {
() => {
// Module: crate::syn_utils
// Provides: {"impl_28"}
// Dependencies: {}
impl GenericParamSet { pub fn new (generics : & Generics) -> Self { let mut idents = HashSet :: new () ; for p in & generics . params { match p { GenericParam :: Type (t) => { idents . insert (t . ident . unraw ()) ; } GenericParam :: Const (t) => { idents . insert (t . ident . unraw ()) ; } _ => { } } } Self { idents } } fn contains (& self , ident : & Ident) -> bool { self . idents . contains (& ident . unraw ()) } pub fn contains_in_type (& self , ty : & Type) -> bool { struct Visitor < 'a > { generics : & 'a GenericParamSet , result : bool , } impl < 'ast > Visit < 'ast > for Visitor < '_ > { fn visit_path (& mut self , i : & 'ast syn :: Path) { if i . leading_colon . is_none () { if let Some (s) = i . segments . iter () . next () { if self . generics . contains (& s . ident) { self . result = true ; } } } visit_path (self , i) ; } } let mut visitor = Visitor { generics : self , result : false , } ; visit_type (& mut visitor , ty) ; visitor . result } }
};
}
